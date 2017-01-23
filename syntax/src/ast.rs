use std::iter::{Peekable, Cloned};
use std::slice;
use std::fmt::Write;

use {NodeType, TokenFile, Token, Range};

#[derive(Clone, Copy)]
pub struct Node<'file> {
    file: &'file AstFile,
    id: NodeId,
}

impl<'file> Node<'file> {
    pub fn ty(&self) -> NodeType {
        self.raw().ty
    }

    pub fn children(&self) -> ChildrenIterator {
        ChildrenIterator {
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
        for child in self.children() {
            child.dump(buff, level + 1);
        }
    }
}

pub struct AstFile {
    text: String,
    nodes: Vec<RawNode>
}


impl AstFile {
    pub fn new(file: TokenFile, file_type: NodeType, parser: &Parser) -> AstFile {
        let nodes = {
            let tokens = file.tokens();
            let tokens = tokens.iter().cloned().peekable();
            let mut builder = AstBuilder::new(tokens);
            builder.start(file_type);
            parser(&mut builder);
            builder.finish(file_type);
            assert!(builder.stack.is_empty());
            builder.into_nodes()
        };

        AstFile {
            text: file.into_text(),
            nodes: nodes
        }
    }

    pub fn dump(&self) -> String {
        let mut buff = String::new();
        for child in self.root().children() {
            child.dump(&mut buff, 0);
        }
        buff
    }

    fn root(&self) -> Node {
        Node { file: self, id: NodeId(0) }
    }

    fn raw(&self, id: NodeId) -> RawNode {
        self.nodes[id.0 as usize]
    }
}

pub struct ChildrenIterator<'file> {
    file: &'file AstFile,
    current: Option<NodeId>
}

impl<'file> Iterator for ChildrenIterator<'file> {
    type Item = Node<'file>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.current.take() {
            self.current = self.file.raw(id).next_sibling;
            Some(Node { file: self.file, id: id })
        } else {
            None
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


pub type Parser = Fn(&mut AstBuilder);

type TokenIter<'a> = Peekable<Cloned<slice::Iter<'a, Token<'a>>>>;

#[derive(Debug)]
pub struct AstBuilder<'a> {
    tokens: TokenIter<'a>,
    nodes: Vec<RawNode>,
    stack: Vec<(NodeId, Option<NodeId>)>,
}


impl<'a> AstBuilder<'a> {
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

    fn new(tokens: TokenIter) -> AstBuilder {
        AstBuilder {
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