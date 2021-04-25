use crate::node::{Node, NodeKind};
use crate::utils::Consumer;

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
// pub struct Input<'a> {
//     input: Peekable<Chars<'a>>,
// }

pub struct Input {
    input: Consumer,
}

impl Input {
    pub fn new(input: &str) -> Self {
        // let iter = input.chars().peekable();
        let consumer = Consumer::new(input);
        Self {input: consumer}
    }

    pub fn tokenize(&mut self) -> Vec<Node> {
        let head_node = self.program();
        head_node
    }


    // program = stmt*
    fn program(&mut self) -> Vec<Node> {
        let mut program = Vec::new();
        loop {
            self.input.skip_space();
            let stmt = self.stmt();
            program.push(stmt);
            self.input.skip_space();
            if self.input.peek().is_none() {
                break;
            }
        }
        program
    }

    // stmt = expr ";"
    fn stmt(&mut self) -> Node {
        self.input.skip_space();
        let expr = self.expr();
        self.input.skip_space();
        match self.input.peek() {
            Some(s) => {
                if s == ";" {
                    self.input.next();
                    expr
                } else {
                    panic!("expected `;`, but found `{}`", s);
                }
            },
            None => {
                expr
            }
        }
    }

    // expr = assign
    fn expr(&mut self) -> Node {
        self.input.skip_space();
        self.assign()
    }

    // assign = equality ("=" assign)?
    fn assign(&mut self) -> Node {
        self.input.skip_space();
        let node = self.equality();
        self.input.skip_space();
        match self.input.peek() {
            Some(s) => {
                if s == "=" {
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
            self.input.skip_space();
            match self.input.peek_n(2) {
                Some(s) => {
                    match &*s {
                        "==" => {
                            self.input.next_n(2);
                            node = Node::new(
                                NodeKind::Op("==".to_string()),
                                Node::link(node),
                                Node::link(self.relational()),
                            );
                        },
                        "!=" => {
                            self.input.next_n(2);
                            node = Node::new(
                                NodeKind::Op("!=".to_string()),
                                Node::link(node),
                                Node::link(self.relational()),
                            );
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
            self.input.skip_space();
            match self.input.peek() {
                Some(s) => {
                    match &*s {
                        "<" => {
                            self.input.next();
                            if self.input.peek().unwrap() == "=" {
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
                        ">" => {
                            self.input.next();
                            // instead of A > B, implement B < A
                            if self.input.peek().unwrap() == "=" {
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
            self.input.skip_space();
            match self.input.peek() {
                Some(s) => {
                    match &*s {
                        "+" => {
                            self.input.next();
                            node = Node::new(
                                NodeKind::Op("+".to_string()), 
                                Node::link(node), 
                                Node::link(self.mul()),
                            );
                        },
                        "-" => {
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
            self.input.skip_space();
            match self.input.peek() {
                Some(s) => {
                    match &*s {
                        "*" => {
                            self.input.next();
                            node = Node::new(
                                NodeKind::Op("*".to_string()), 
                                Node::link(node), 
                                Node::link(self.unary()),
                            );
                            
                        },
                        "/" => {
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
        self.input.skip_space();
        match self.input.peek() {
            Some(s) => {
                match &*s {
                    "+" => {
                        self.input.next();
                        return self.primary();
                    },
                    "-" => {
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
        self.input.skip_space();
        match self.input.peek() {
            Some(s) => {
                if s.chars().all(|c| char::is_numeric(c)) {
                    let num = self.input.to_usize().unwrap();
                    return Node::new(
                        NodeKind::Num(num),
                        None,
                        None,
                    );
                }
                if s == "(" {
                    self.input.next();
                    let node = self.expr();
                    self.input.skip_space();
                    if self.input.peek().unwrap() == ")" {
                        self.input.next();
                    } else {
                        panic!("expected `)`, but not found");
                    }
                    return node;
                }
                if s.chars().all(|c| char::is_alphabetic(c)) {
                    let ident = s;
                    return Node::new(
                        NodeKind::LVar(ident.to_string()),
                        None,
                        None,
                    );
                }
                panic!("invalid element: {}", s);
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

