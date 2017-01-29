use data_structures::LazyCell;

use std::fmt::{self, Write};

use {NodeType, TokenFile, Token, Range, WHITESPACE};

#[derive(Clone, Copy)]
pub struct Node<'f> {
    file: &'f RstFile,
    id: NodeId,
}

impl<'f> Node<'f> {
    pub fn ty(&self) -> NodeType {
        self.raw().ty
    }

    pub fn children_with_ws(&self) -> ChildrenIterator {
        ChildrenIterator {
            skip_ws: false,
            file: self.file,
            current: self.raw().first_child(),
        }
    }

    pub fn children(&self) -> ChildrenIterator {
        ChildrenIterator {
            skip_ws: true,
            file: self.file,
            current: self.raw().first_child(),
        }
    }

    pub fn text(&self) -> &'f str {
        &self.file.text[self.range()]
    }

    fn range(&self) -> Range {
        match self.file.raw(self.id).data {
            RawNodeData::Leaf { range } => range,
            RawNodeData::Composite { ref range, .. } => range.get(|| {
                let mut children = self.children_with_ws();
                let first = children.next().unwrap().range();
                let lo = first.lo();
                let mut hi = first.hi();
                for child in children {
                    let r = child.range();
                    assert_eq!(r.lo(), hi);
                    hi = r.hi();
                }
                Range::from_to(lo, hi)
            })
        }
    }

    fn raw(&self) -> &RawNode {
        self.file.raw(self.id)
    }

    fn dump(&self, buff: &mut String, level: u32) {
        for _ in 0..level {
            buff.push_str("  ");
        }
        buff.push_str(self.ty().name());
        match self.raw().data {
            RawNodeData::Leaf { range } =>
                write!(buff, " {:?}", &self.file.text[range]).expect("write to string can't fail"),
            RawNodeData::Composite { .. } => {}
        }
        buff.push('\n');
        for child in self.children_with_ws() {
            child.dump(buff, level + 1);
        }
    }
}

impl<'f> fmt::Debug for Node<'f> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.ty().fmt(fmt)
    }
}

pub struct RstFile {
    text: String,
    nodes: Vec<RawNode>
}


impl RstFile {
    pub fn new(file: TokenFile, file_type: NodeType, parser: &Parser) -> RstFile {
        let nodes = {
            let tokens = file.tokens();
            let mut builder = RstBuilder::new(&tokens);
            builder.start(file_type);
            builder.skip_ws();
            parser(&mut builder);
            builder.skip_ws();
            builder.finish(file_type);
            assert!(builder.stack.is_empty());
            builder.into_nodes()
        };

        RstFile {
            text: file.into_text(),
            nodes: nodes
        }
    }

    pub fn dump(&self) -> String {
        let mut buff = String::new();
        for child in self.root().children_with_ws() {
            child.dump(&mut buff, 0);
        }
        buff
    }

    pub fn root(&self) -> Node {
        Node { file: self, id: NodeId(0) }
    }

    fn raw(&self, id: NodeId) -> &RawNode {
        &self.nodes[id.0 as usize]
    }
}

pub struct ChildrenIterator<'f> {
    skip_ws: bool,
    file: &'f RstFile,
    current: Option<NodeId>
}

impl<'f> Iterator for ChildrenIterator<'f> {
    type Item = Node<'f>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_ws {
            self.skip_ws()
        }
        if let Some(id) = self.current.take() {
            self.current = self.file.raw(id).next_sibling;
            Some(Node { file: self.file, id: id })
        } else {
            None
        }
    }
}

impl<'f> ChildrenIterator<'f> {
    pub fn skip_node(&mut self, ty: NodeType) {
        let child = self.next().expect("Can't skip child");
        assert_eq!(child.ty(), ty);
    }

    pub fn finish(&mut self) {
        assert!(self.current.is_none());
    }

    fn skip_ws(&mut self) {
        while let Some(node) = self.current {
            let node = self.file.raw(node);
            if node.ty != WHITESPACE {
                break
            }
            self.current = node.next_sibling
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct NodeId(u32);

#[derive(Debug)]
struct RawNode {
    ty: NodeType,
    parent: Option<NodeId>,
    next_sibling: Option<NodeId>,
    data: RawNodeData,
}

#[derive(Debug)]
enum RawNodeData {
    Leaf {
        range: Range
    },
    Composite {
        first_child: Option<NodeId>,
        range: LazyCell<Range>,
    }
}

impl RawNode {
    fn first_child(&self) -> Option<NodeId> {
        match self.data {
            RawNodeData::Leaf { .. } => None,
            RawNodeData::Composite { first_child, .. } => first_child,
        }
    }

    fn set_first_child(&mut self, id: NodeId) {
        match self.data {
            RawNodeData::Leaf { .. } => panic!("Leaf node can't have children"),
            RawNodeData::Composite { ref mut first_child, .. } => *first_child = Some(id),
        }
    }
}


pub type Parser = Fn(&mut RstBuilder);

#[derive(Debug)]
pub struct RstBuilder<'f> {
    tokens: &'f [Token<'f>],
    pos: usize,
    nodes: Vec<RawNode>,
    stack: Vec<Frame>,
}

#[derive(Debug)]
struct Frame {
    parent: NodeId,
    last_child: Option<NodeId>,
}

impl Frame {
    fn new_leaf_node(&mut self, nodes: &mut Vec<RawNode>, token: Token) {
        let node = RawNode {
            ty: token.ty,
            parent: Some(self.parent),
            next_sibling: None,
            data: RawNodeData::Leaf { range: token.range },
        };
        let id = NodeId(nodes.len() as u32);
        nodes.push(node);

        self.add_child(&mut *nodes, id);
    }

    fn new_composite_node(&mut self, nodes: &mut Vec<RawNode>, ty: NodeType) -> NodeId {
        let node = RawNode {
            ty: ty,
            parent: Some(self.parent),
            next_sibling: None,
            data: RawNodeData::Composite { first_child: None, range: LazyCell::new() }
        };
        let id = NodeId(nodes.len() as u32);
        nodes.push(node);

        self.add_child(&mut *nodes, id);
        id
    }

    fn add_child(&mut self, nodes: &mut [RawNode], id: NodeId) {
        if let Some(prev) = self.last_child {
            nodes[prev.0 as usize].next_sibling = Some(id)
        } else {
            nodes[self.parent.0 as usize].set_first_child(id)
        }
        self.last_child = Some(id);
    }
}

impl<'f> RstBuilder<'f> {
    pub fn peek(&self) -> Option<NodeType> {
        if self.pos < self.tokens.len() {
            Some(self.tokens[self.pos].ty)
        } else {
            None
        }
    }

    pub fn bump(&mut self) {
        let token = {
            if self.pos >= self.tokens.len() {
                panic!("EOF")
            }
            self.pos += 1;
            self.tokens[self.pos - 1]
        };

        let frame = self.stack.last_mut()
            .expect("Token without parent");

        frame.new_leaf_node(&mut self.nodes, token);
    }

    pub fn eat(&mut self, ty: NodeType) {
        match self.peek() {
            Some(t) if t == ty => self.bump(),
            None => panic!("EOF"),
            Some(t) => panic!("Expected {:?}, got {:?}", ty, t),
        }
    }

    pub fn skip_ws(&mut self) {
        while let Some(t) = self.peek() {
            if t != WHITESPACE {
                break;
            }

            self.bump()
        }
    }

    pub fn start(&mut self, ty: NodeType) {
        if self.stack.is_empty() {
            let id = self.new_composite_node(None, ty);
            self.stack.push(Frame {
                parent: id,
                last_child: None,
            });
            return
        }

        let id = self.stack
            .last_mut().unwrap()
            .new_composite_node(&mut self.nodes, ty);

        self.stack.push(Frame {
            parent: id,
            last_child: None,
        });
    }

    pub fn finish(&mut self, ty: NodeType) {
        let frame = self.stack.pop()
            .expect("Empty parent stack");
        assert_eq!(self.node_mut(frame.parent).ty, ty);
    }

    fn new(tokens: &'f [Token<'f>]) -> RstBuilder<'f> {
        RstBuilder {
            tokens: tokens,
            pos: 0,
            nodes: Vec::new(),
            stack: Vec::new(),
        }
    }

    fn into_nodes(self) -> Vec<RawNode> {
        self.nodes
    }

    fn node_mut(&mut self, id: NodeId) -> &mut RawNode {
        &mut self.nodes[id.0 as usize]
    }

    fn new_composite_node(&mut self, parent: Option<NodeId>, ty: NodeType) -> NodeId {
        let node = RawNode {
            ty: ty,
            parent: parent,
            next_sibling: None,
            data: RawNodeData::Composite { first_child: None, range: LazyCell::new() }
        };
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }
}