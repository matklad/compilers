use syntax::{RstFile, Node, NodeType};

use rst::{LITERAL, ID, LIST, LPAREN, RPAREN, TINY_FILE, STRING, NUMBER};

pub struct AstFile {
    rst: RstFile
}

impl AstFile {
    pub fn new(rst: RstFile) -> AstFile {
        AstFile { rst: rst }
    }

    pub fn root(&self) -> Program {
        Program::from_node(self.rst.root())
    }
}

pub trait AstElement<'f> {
    fn ty() -> NodeType;

    fn from_node(node: Node<'f>) -> Self;

    fn node(&self) -> Node<'f>;
}

pub struct Program<'f> {
    node: Node<'f>
}

impl<'f> AstElement<'f> for Program<'f> {
    fn ty() -> NodeType {
        TINY_FILE
    }

    fn from_node(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Program { node: node }
    }

    fn node(&self) -> Node<'f> {
        self.node
    }
}

impl<'f> Program<'f> {
    pub fn elements(&self) -> Vec<ListElement> {
        self.node.children().map(ListElement::from_node).collect()
    }
}


#[derive(Debug)]
pub struct List<'f> {
    node: Node<'f>
}

impl<'f> AstElement<'f> for List<'f> {
    fn ty() -> NodeType {
        LIST
    }

    fn from_node(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        List { node: node }
    }

    fn node(&self) -> Node<'f> {
        self.node
    }
}


impl<'f> List<'f> {
    pub fn elements(&self) -> Vec<ListElement> {
        let mut result = Vec::new();
        let mut children = self.node.children();
        children.skip_node(LPAREN);
        loop {
            match children.next() {
                Some(node) if node.ty() == RPAREN => {
                    children.finish();
                    return result;
                }
                Some(node) => result.push(ListElement::from_node(node)),
                None => panic!("No closing rparen")
            }
        }
    }
}

#[derive(Debug)]
pub struct Literal<'f> {
    node: Node<'f>
}

impl<'f> AstElement<'f> for Literal<'f> {
    fn ty() -> NodeType {
        LITERAL
    }

    fn from_node(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Literal { node: node }
    }

    fn node(&self) -> Node<'f> {
        self.node
    }
}

pub enum LiteralValue<'f> {
    Int(u32),
    String(&'f str)
}

impl<'f> Literal<'f> {
    pub fn value(&self) -> LiteralValue {
        let text = self.node().text();
        match self.literal_ty() {
            STRING => LiteralValue::String(&text[1..text.len() - 1]),
            NUMBER => LiteralValue::Int(text.parse().unwrap()),
            _ => panic!()
        }
    }

    fn literal_ty(&self) -> NodeType {
        let child = self.node.children().next().unwrap();
        child.ty()
    }
}


#[derive(Debug)]
pub struct Variable<'f> {
    node: Node<'f>
}


impl<'f> AstElement<'f> for Variable<'f> {
    fn ty() -> NodeType {
        ID
    }

    fn from_node(node: Node<'f>) -> Self {
        assert_eq!(node.ty(), Self::ty());
        Variable { node: node }
    }

    fn node(&self) -> Node<'f> {
        self.node
    }
}


#[derive(Debug)]
pub enum ListElement<'f> {
    List(List<'f>),
    Literal(Literal<'f>),
    Variable(Variable<'f>),
}

impl<'f> ListElement<'f> {
    fn from_node(node: Node) -> ListElement {
        match node.ty() {
            LITERAL => ListElement::Literal(Literal::from_node(node)),
            ID => ListElement::Variable(Variable::from_node(node)),
            LIST => ListElement::List(List::from_node(node)),
            _ => panic!("Bad list node {:?}", node)
        }
    }
}

#[test]
fn test_ast() {
    let file = AstFile::new(super::parse_tiny("29 (foo 1)".to_owned()));
    let program = file.root();
    let elements = program.elements();
    assert_eq!(elements.len(), 2);
    let sub = match (&elements[0], &elements[1]) {
        (&ListElement::Literal(_), &ListElement::List(ref ls)) => ls,
        _ => panic!()
    };

    let elements = sub.elements();
    assert_eq!(elements.len(), 2);

    match (&elements[0], &elements[1]) {
        (&ListElement::Variable(_), &ListElement::Literal(_)) => {},
        _ => panic!()
    };
}