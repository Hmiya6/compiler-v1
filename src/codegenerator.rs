use crate::node::{Node, NodeKind};
use crate::lexer::Input;

pub struct CodeGenerator {
    head: Option<Node>,
}


impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }


    pub fn from_node(head: Node) -> Self {
        Self {
            head: Some(head),
        }
    }
        
    pub fn from_str(s: &str) -> Self {
        let mut input = Input::new(s);
        let mut nodes = input.tokenize();
        // TODO
        let head = nodes.pop().unwrap();
        Self {
            head: Some(head),
        }
    }

    pub fn compile(&mut self) {
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("main:");
        if let Some(head) = Option::take(&mut self.head) {
            Self::gen(head);
            println!("    pop rax");
        }
        println!("    ret");
    }

    fn gen(node: Node) {

        if let Some(child) = node.lhs {
            Self::gen(*child);
        }
        if let Some(child) = node.rhs {
            Self::gen(*child);
        }
        
        match node.kind {
            NodeKind::Num(n) => {
                println!("    push {}", n);
                return;
            },
            NodeKind::Op(op) => {
                println!("    pop rdi");
                println!("    pop rax");
                match &*op {
                    "+" => {
                        println!("    add rax, rdi");
                    },
                    "-" => {
                        println!("    sub rax, rdi");
                    },
                    "*" => {
                        println!("    imul rax, rdi");
                    },
                    "/" => {
                        println!("    cqo");
                        println!("    idiv rdi");
                    },
                    "==" => {
                        // if rax == rdi, then set 1 to flag register
                        println!("    cmp rax, rdi");
                        // load the value of the flag register to al, which is lower 8 bits of rax
                        println!("    sete al");
                        // clear upper 56 bits with 0s
                        println!("    movzb rax, al");
                    },
                    "!=" => {
                        println!("    cmp rax, rdi");
                        println!("    setne al");
                        println!("    movzb rax, al");
                    },
                    "<" => {
                        println!("    cmp rax, rdi");
                        println!("    setl al");
                        println!("    movzb rax, al");
                    },
                    "<=" => {
                        println!("    cmp rax, rdi");
                        println!("    setle al");
                        println!("    movzb rax, al");
                    },
                    "=" => {

                    },
                    _ => {
                        panic!("compile error");
                    }
                }
                println!("    push rax");
                return;
            },
            NodeKind::LVar(ident) => {
                
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compile() {
        /*
        CodeGenerator::from_str("((100 + 100)* 10) + 100").compile();
        CodeGenerator::from_str("-5").compile();
        CodeGenerator::from_str("123 +  (  + 33 - 99 )* 24").compile();
        CodeGenerator::from_str("123 > 122").compile();
        CodeGenerator::from_str("42 == 43").compile();
        */
    }

}


