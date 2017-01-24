use std::iter::{Peekable, Cloned};
use std::slice;
use std::fmt::{self, Write};

use {NodeType, TokenFile, Token, Range, WHITESPACE};

#[derive(Clone, Copy)]
pub struct Node<'file> {
    file: &'file RstFile,
    id: NodeId,
}

impl<'file> Node<'file> {
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

    fn raw(&self) -> RawNode {
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
            let tokens = tokens.iter().cloned().peekable();
            let mut builder = RstBuilder::new(tokens);
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

    fn raw(&self, id: NodeId) -> RawNode {
        self.nodes[id.0 as usize]
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

#[derive(Clone, Copy, Debug)]
struct RawNode {
    ty: NodeType,
    parent: Option<NodeId>,
    next_sibling: Option<NodeId>,
    data: RawNodeData,
}

#[derive(Clone, Copy, Debug)]
enum RawNodeData {
    Leaf {
        range: Range
    },
    Composite {
        first_child: Option<NodeId>
    }
}

impl RawNode {
    fn first_child(&self) -> Option<NodeId> {
        match self.data {
            RawNodeData::Leaf { .. } => None,
            RawNodeData::Composite { first_child } => first_child,
        }
    }

    fn set_first_child(&mut self, id: NodeId) {
        match self.data {
            RawNodeData::Leaf { .. } => panic!("Leaf node can't have children"),
            RawNodeData::Composite { ref mut first_child } => *first_child = Some(id),
        }
    }
}


pub type Parser = Fn(&mut RstBuilder);

type TokenIter<'a> = Peekable<Cloned<slice::Iter<'a, Token<'a>>>>;

#[derive(Debug)]
pub struct RstBuilder<'a> {
    tokens: TokenIter<'a>,
    nodes: Vec<RawNode>,
    stack: Vec<(NodeId, Option<NodeId>)>,
}


impl<'a> RstBuilder<'a> {
    pub fn peek(&mut self) -> Option<NodeType> {
        self.tokens.peek().map(|t| t.ty)
    }

    pub fn bump(&mut self) {
        let token = self.tokens.next().expect("EOF");
        let (parent, sibling) = self.stack.pop()
            .expect("Token without parent");

        let id = self.new_leaf_node(parent, token);
        if let Some(prev) = sibling {
            self.node_mut(prev).next_sibling = Some(id)
        } else {
            self.node_mut(parent).set_first_child(id)
        }
        self.stack.push((parent, Some(id)))
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
        let ps = self.stack.pop();
        let parent = ps.map(|(p, _)| p);
        let id = self.new_composite_node(parent, ty);
        match ps {
            Some((parent, Some(prev))) => {
                self.node_mut(prev).next_sibling = Some(id);
                self.stack.push((parent, Some(id)))
            },
            Some((parent, None)) => {
                self.node_mut(parent).set_first_child(id);
                self.stack.push((parent, Some(id)))
            },
            None => {}
        }

        self.stack.push((id, None));
    }

    pub fn finish(&mut self, ty: NodeType) {
        let (p, _) = self.stack.pop()
            .expect("Empty parent stack");
        assert_eq!(self.node_mut(p).ty, ty);
    }

    fn new(tokens: TokenIter) -> RstBuilder {
        RstBuilder {
            tokens: tokens,
            nodes: Vec::new(),
            stack: Vec::new()
        }
    }

    fn into_nodes(self) -> Vec<RawNode> {
        self.nodes
    }

    fn node_mut(&mut self, id: NodeId) -> &mut RawNode {
        &mut self.nodes[id.0 as usize]
    }

    fn new_leaf_node(&mut self, parent: NodeId, token: Token) -> NodeId {
        let node = RawNode {
            ty: token.ty,
            parent: Some(parent),
            next_sibling: None,
            data: RawNodeData::Leaf { range: token.range },
        };
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    fn new_composite_node(&mut self, parent: Option<NodeId>, ty: NodeType) -> NodeId {
        let node = RawNode {
            ty: ty,
            parent: parent,
            next_sibling: None,
            data: RawNodeData::Composite { first_child: None }
        };
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }
}