use std::iter::Peekable;

use {NodeType, TokenFile, Token};

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
            current: self.raw().first_child,
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
        buff.push('\n');
        for child in self.children() {
            child.dump(buff, level + 1);
        }
    }
}

pub struct AstFile {
    nodes: Vec<RawNode>
}


impl AstFile {
    pub fn new(file: TokenFile, file_type: NodeType, parser: &Parser) -> AstFile {
        let tokens = file.tokens();
        let tokens: &mut Iterator<Item = &Token> = &mut tokens.iter();

        let mut builder = AstBuilder::new();
        builder.start(file_type);
        parser(tokens.peekable(), &mut builder);
        builder.finish(file_type);
        assert!(builder.stack.is_empty());
        AstFile { nodes: builder.into_nodes() }
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
    first_child: Option<NodeId>,
    next_sibling: Option<NodeId>,
}


pub type TokenIterator<'a, 'file> = Peekable<&'a mut Iterator<Item = &'a Token<'file>>>;
pub type Parser = Fn(TokenIterator, &mut AstBuilder);


#[derive(Debug)]
pub struct AstBuilder {
    nodes: Vec<RawNode>,
    stack: Vec<(NodeId, Option<NodeId>)>,
}


impl AstBuilder {
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
                self.node_mut(parent).first_child = Some(id);
                self.stack.push((parent, Some(id)))
            },
            None => {}
        }

        self.stack.push((id, None));
    }

    pub fn advance(&mut self, token: &Token) {
        let (parent, sibling) = self.stack.pop()
            .expect("Token without parent");

        let id = self.new_lead_node(parent, token);
        if let Some(prev) = sibling {
            self.node_mut(prev).next_sibling = Some(id)
        } else {
            self.node_mut(parent).first_child = Some(id)
        }
        self.stack.push((parent, Some(id)))
    }

    pub fn finish(&mut self, ty: NodeType) {
        let (p, _) = self.stack.pop()
            .expect("Empty parent stack");
        assert_eq!(self.node_mut(p).ty, ty);
    }

    fn new() -> AstBuilder {
        AstBuilder { nodes: Vec::new(), stack: Vec::new() }
    }

    fn into_nodes(self) -> Vec<RawNode> {
        self.nodes
    }

    fn node_mut(&mut self, id: NodeId) -> &mut RawNode {
        &mut self.nodes[id.0 as usize]
    }

    fn new_lead_node(&mut self, parent: NodeId, token: &Token) -> NodeId {
        let node = RawNode {
            ty: token.ty,
            parent: Some(parent),
            first_child: None,
            next_sibling: None,
        };
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    fn new_composite_node(&mut self, parent: Option<NodeId>, ty: NodeType) -> NodeId {
        let node = RawNode {
            ty: ty,
            parent: parent,
            first_child: None,
            next_sibling: None,
        };
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }
}