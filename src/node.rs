
#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Op(String),
    Num(usize),
    LVar(String),
}

type Link = Option<Box<Node>>;

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Link,
    pub rhs: Link,
}

impl Node {
    pub fn new(kind: NodeKind, lhs: Link, rhs: Link) -> Self {
        Self {kind, lhs, rhs}
    }

    pub fn link(node: Node) -> Link {
        Some(Box::new(node))
    }

}
