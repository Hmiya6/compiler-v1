use std::iter::{Iterator, Peekable};
use std::str::Chars;

use crate::node::{Node, NodeKind};
use crate::utils::strtou;

/*

program = stmt*
stmt = expr ";"
expr = assign
assign = equality ("=" assign)?
equality = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add = mul ("+" mul | "-" mul)*
mul = unary ("*" unary | "/" unary)*
unary = ("+" | "-")? primary
primary = num | ident | "(" expr ")"

*/


//TODO
// original Input
// 1. peek(n)
// 2. next(n)
// 3. inputtou(&mut Input)

// Input makes node tree from string
pub struct Input<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a str) -> Self {
        let iter = input.chars().peekable();
        Self {input: iter}
    }

    pub fn tokenize(&mut self) -> Vec<Node> {
        let head_node = self.program();
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

    // program = stmt*
    fn program(&mut self) -> Vec<Node> {
        let mut program = Vec::new();
        loop {
            self.skip_space();
            let mut stmt = self.stmt();
            program.push(stmt);
            self.skip_space();
            if self.input.peek().is_none() {
                break;
            }
        }
        program
    }

    // stmt = expr ";"
    fn stmt(&mut self) -> Node {
        self.skip_space();
        let expr = self.expr();
        self.skip_space();
        match self.input.peek() {
            Some(&c) => {
                if c == ';' {
                    self.input.next();
                    expr
                } else {
                    panic!("expected `;`, but found `{}`", c);
                }
            },
            None => {
                expr
            }
        }
    }

    // expr = assign
    fn expr(&mut self) -> Node {
        self.skip_space();
        self.assign()
    }

    // assign = equality ("=" assign)?
    fn assign(&mut self) -> Node {
        self.skip_space();
        let mut node = self.equality();
        self.skip_space();
        match self.input.peek() {
            Some(&c) => {
                if c == '=' {
                    self.input.next();
                    Node::new(
                        NodeKind::Op("=".to_string()),
                        Node::link(node),
                        Node::link(self.assign()),
                    )
                } else {
                    node
                    // panic!("expected `=`, but found `{}`", c);
                }
            },
            None => {
                node
            }
        }

    }
    
    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Node {
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
                                // panic!("invalid operator `=`, expected `==`");
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
    
    // primary = num | ident | '(' expr ')'
    fn primary(&mut self) -> Node {
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
                    'a'..='z' => {
                        // println!("variable!");
                        self.input.next();
                        let ident = c;
                        let node = Node::new(
                            NodeKind::LVar(ident.to_string()),
                            None,
                            None,
                        );
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_node() {
        test_tokenize("1*(2+3);");
        test_tokenize("1 + 20+ 4;");
        test_tokenize(" 9- 6 * 10;");
        test_tokenize("1-10/100 +1000 * 10000;");
        test_tokenize("((2-20)*200 + 2000)*(21 - 201);");
        test_tokenize("((100 + 100)* 10) + 100;");
        test_tokenize("1 == 1;");
        test_tokenize("1 != 1;");
        test_tokenize("1 <= 1;");
        test_tokenize("1 >= 1;");
        test_tokenize("1 < 1;");
        test_tokenize("1 > 1;");
        test_tokenize("1 == 1 == 1;");
        test_tokenize("1 > 1 > 1;");
        test_tokenize("a = 1;");
        test_tokenize("a = 1 + 3;");
        test_tokenize("a = b * 3 - p;");
    }
    

    fn print_node(node: &Node) {
        println!("{:?}", node.kind);
        if let Some(n) = &node.lhs {
            print_node(n);
        }
        
        if let Some(n) = &node.rhs {
            print_node(n);
        }
    }

    fn test_tokenize(s: &str) {
        let mut input = Input::new(s);
        let head = input.tokenize();
        for h in head {
            print_node(&h);
        }
        println!("------------");
    }
}

