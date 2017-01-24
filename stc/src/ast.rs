use syntax::{TokenFile, RstFile, Node};

use rst::{tiny_tokenizer, tiny_parser, LITERAL, ID, LIST, LPAREN, RPAREN, TINY_FILE};

pub struct AstFile {
    rst: RstFile
}

impl AstFile {
    pub fn new(rst: RstFile) -> AstFile {
        AstFile { rst: rst }
    }

    fn from_text(text: &str) -> AstFile {
        let tokens = TokenFile::new(text.to_owned(), &tiny_tokenizer);
        let rst = RstFile::new(tokens, TINY_FILE, &tiny_parser);
        AstFile::new(rst)
    }

    pub fn root(&self) -> Program {
        Program { node: self.rst.root() }
    }
}

pub struct Program<'f> {
    node: Node<'f>
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

#[derive(Debug)]
pub struct Variable<'f> {
    node: Node<'f>
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
            LITERAL => ListElement::Literal(Literal { node: node }),
            ID => ListElement::Variable(Variable { node: node }),
            LIST => ListElement::List(List { node: node }),
            _ => panic!("Bad list node {:?}", node)
        }
    }
}

#[test]
fn test_ast() {
    let file = AstFile::from_text("29 (foo 1)");
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