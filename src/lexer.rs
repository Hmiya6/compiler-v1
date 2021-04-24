use std::iter::{Iterator, Peekable};
use std::str::Chars;

use crate::node::{Node, NodeKind};
use crate::utils::strtou;


/*
Grammer

expr = equality
equality = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add = mul ("+" mul | "-" mul)*
mul = unary ("*" unary | "/" unary)*
unary = ('+' | '-')? primary
primary = num | "(" expr ")"

*/

// Input makes node tree from string
pub struct Input<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a str) -> Self {
        let iter = input.chars().peekable();
        Self {input: iter}
    }

    pub fn tokenize(&mut self) -> Node {
        let head_node = self.expr();
        head_node
    }

    fn skip_space(&mut self) {
        loop {
            match self.input.peek() {
                Some(&space) => {
                    if space == ' ' {
                        self.input.next();
                    } else {
                        break;
                    }
                },
                None => {break;},
            }
        }
    }
    
    // expr = equality
    fn expr(&mut self) -> Node {
        // println!("expr");
        self.skip_space();
        self.equality()
    }
    
    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Node {
        // println!("equality");
        let mut node = self.relational();

        loop {
            self.skip_space();
            match self.input.peek() {
                Some(&c) => {
                    match c {
                        '=' => {
                            self.input.next();
                            if *self.input.peek().unwrap() == '=' {
                                self.input.next();
                                node = Node::new(
                                    NodeKind::Op("==".to_string()), 
                                    Node::link(node), 
                                    Node::link(self.relational()),
                                );
                            } else {
                                panic!("invalid operator `=`, expected `==`");
                            }
                        },
                        '!' => {
                            self.input.next();
                            if *self.input.peek().unwrap() == '=' {
                                self.input.next();
                                node = Node::new(
                                    NodeKind::Op("!=".to_string()), 
                                    Node::link(node), 
                                    Node::link(self.relational()),
                                );
                            } else {
                                panic!("invalid operator `!`, expected `!=`");
                            }
                        }
                        _ => {
                            return node;
                        }
                    }
                },
                None => {
                    return node;
                }
            }
        }
    }
    
    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> Node {
        // println!("relational");
        let mut node = self.add();

        loop {
            self.skip_space();
            match self.input.peek() {
                Some(&c) => {
                    match c {
                        '<' => {
                            self.input.next();
                            if *self.input.peek().unwrap() == '=' {
                                self.input.next();
                                node = Node::new(
                                    NodeKind::Op("<=".to_string()), 
                                    Node::link(node), 
                                    Node::link(self.add()),
                                );
                            } else {
                                node = Node::new(
                                    NodeKind::Op("<".to_string()), 
                                    Node::link(node), 
                                    Node::link(self.add()),
                                );
                            }
                        }
                        '>' => {
                            self.input.next();
                            // instead of A > B, implement B < A
                            if *self.input.peek().unwrap() == '=' {
                                self.input.next();
                                node = Node::new(
                                    NodeKind::Op("<=".to_string()), 
                                    Node::link(self.add()), 
                                    Node::link(node),
                                );
                            } else {
                                node = Node::new(
                                    NodeKind::Op("<".to_string()), 
                                    Node::link(self.add()), 
                                    Node::link(node),
                                );
                            }
                        }
                        _ => {
                            // panic!("invalid operator {}, expected `<` or `>`", c);
                            return node;
                        }
                    }
                }
                None => {
                    return node;
                }
            }
        }
    }
    
    // add = mul ('+' mul | '-' mul)*
    fn add(&mut self) -> Node {
        // println!("add");
        let mut node = self.mul();

        loop {
            self.skip_space();
            match self.input.peek() {
                Some(&c) => {
                    match c {
                        '+' => {
                            self.input.next();
                            node = Node::new(
                                NodeKind::Op("+".to_string()), 
                                Node::link(node), 
                                Node::link(self.mul()),
                            );
                        },
                        '-' => {
                            self.input.next();
                            node = Node::new(
                                NodeKind::Op("-".to_string()), 
                                Node::link(node), 
                                Node::link(self.mul()),
                            );
                        },
                        _ => {
                            return node;
                        },
                    }
                },
                None => {
                    return node;
                }
            }
        }

    }
    
    // mul = uary ('*' unary | '/' uary)*
    fn mul(&mut self) -> Node {
        // println!("mul");
        let mut node = self.unary();

        loop {
            self.skip_space();
            match self.input.peek() {
                Some(&c) => {
                    match c {
                        '*' => {
                            self.input.next();
                            node = Node::new(
                                NodeKind::Op("*".to_string()), 
                                Node::link(node), 
                                Node::link(self.unary()),
                            );
                            
                        },
                        '/' => {
                            self.input.next();
                            node = Node::new(
                                NodeKind::Op("/".to_string()), 
                                Node::link(node), 
                                Node::link(self.unary()),
                                );
                        },
                        _ => {
                            return node;
                        }
                    }
                },
                None => {
                    return node;
                }
            }
        }
    }

    // unary = ('+' | '-')? primary
    fn unary(&mut self) -> Node {
        // println!("unary");
        self.skip_space();
        match self.input.peek() {
            Some(&c) => {
                match c {
                    '+' => {
                        self.input.next();
                        return self.primary();
                    },
                    '-' => {
                        self.input.next();
                        // returns 0 - primary
                        return Node::new(
                            NodeKind::Op("-".to_string()), 
                            Node::link(Node::new(NodeKind::Num(0), None, None)),
                            Node::link(self.primary()),
                        );
                    },
                    _ => {
                        return self.primary();
                    }
                }
            },
            None => {
                panic!("Expected value: found None");
            }
        }
    }
    
    // primary = num | '(' expr ')'
    fn primary(&mut self) -> Node {
        // println!("primary");
        self.skip_space();
        match self.input.peek() {
            Some(&c) => {
                match c {
                    '0'..='9' => {
                        let num = strtou(&mut self.input);
                        // println!("digit: {}", num);
                        let node = Node::new(
                            NodeKind::Num(num), 
                            None,
                            None,
                        );
                        return node;
                    },
                    '(' => {
                        self.input.next();
                        let node = self.expr();
                        self.skip_space();
                        if *self.input.peek().unwrap() == ')' {
                            self.input.next();
                        } else {
                            panic!("')' not found!");
                        }
                        return node;
                    },
                    _ => {
                        panic!("Invalid number: {}", c);
                    }
                }
            },
            None => {
                panic!("Expected some value!");
            }
        }
    }
}
