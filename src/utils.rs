use std::iter::{Iterator, Peekable};

pub fn strtou<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> usize {
    let mut result: usize = 0;
    loop {
        match iter.peek() {
            Some(c) => match c.to_digit(10) {
                Some(i) => result = result * 10 + i as usize,
                None => break,
            },
            None => break,
        }
        iter.next();
    }
    result
}



pub struct Consumer {
    queue: Vec<char>,
    pos: usize,
}


impl Consumer {
    pub fn new(s: &str) -> Self {
        let vec = s.chars().collect::<Vec<char>>();
        Self {
            queue: vec,
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if let Some(c) = self.next_char() {
            Some(c.to_string())
        } else {
            None
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let res: char;
        if self.pos < self.queue.len() {
            res = self.queue[self.pos];
            self.pos +=1;
            Some(res)
        } else {
            None
        }
    }

    pub fn next_n(&mut self, n: usize) -> Option<String> {
        let mut vec = Vec::new();
        for _ in 0..n {
            if let Some(c) = self.next_char() {
                vec.push(c);
            } else {
                return None;
            }
        }
        let res = vec.into_iter().collect::<String>();
        Some(res)

    }

    pub fn peek(&self) -> Option<String> {
        if let Some(c) = self.peek_char() {
            Some(c.to_string())
        } else {
            None
        }

    }

    fn peek_char(&self) -> Option<char> {
        let res: char;
        if self.pos < self.queue.len() {
            res = self.queue[self.pos];
            Some(res)
        } else {
            None
        }
    }

    pub fn peek_n(&self, n: usize) -> Option<String> {
        let mut vec = Vec::new();
        for i in 0..n {
            if self.pos+i < self.queue.len() {
                vec.push(self.queue[self.pos+i]);
            } else {
                return None;
            }
        }
        let res = vec.into_iter().collect::<String>();
        Some(res)
    }
    
    pub fn to_usize(&mut self) -> Option<usize> {
        let mut result: usize = 0;

        // check whether the first char is number
        match self.peek_char() {
            Some(c) => {
                match c {
                    '0'..='9' => {
                    }
                    _ => {
                        return None;
                    }
                }
            }
            None => {
                return None;
            }
        }

        loop {
            match self.peek_char() {
                Some(c) => {
                    match c {
                        '0'..='9' => {
                            self.next();
                            let n = c.to_digit(10).unwrap() as usize;
                            result = result*10 + n;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        return Some(result);
    }

    pub fn skip_space(&mut self) {
        loop {
            if let Some(c) = self.peek_char() {
                if " \t".contains(c) {
                    self.next_char();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    // pub fn next_until_space(&mut self) -> Option<String> {
    //
    // }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn consumer() {
        consumer_next();
        consumer_peek();
        consumer_to_usize();
        consumer_skip_space();
    }

    #[test]
    fn consumer_next() {
        let mut con = Consumer::new("consumer");
        assert_eq!(con.next(), Some(String::from("c")));
        assert_eq!(con.next_n(3), Some(String::from("ons")));
        assert_eq!(con.next_n(1000), None);
        assert_eq!(con.next(), None);
    }

    #[test]
    fn consumer_peek() {
        let mut con = Consumer::new("Hello people");
        assert_eq!(con.peek(), Some(String::from("H")));
        assert_eq!(con.next_n(4), Some(String::from("Hell")));
        assert_eq!(con.peek_n(1000), None);
        assert_eq!(con.peek_n(8), Some(String::from("o people")));
    }
    
    #[test]
    fn consumer_to_usize() {
        let mut con = Consumer::new("12345");
        assert_eq!(con.to_usize(), Some(12345));
        let mut con = Consumer::new("a");
        assert_eq!(con.to_usize(), None);
        let mut con = Consumer::new("123a");
        assert_eq!(con.to_usize(), Some(123));
    }
    
    #[test]
    fn consumer_skip_space() {
        let mut con = Consumer::new("  \t  konnichwassup   ");
        con.skip_space();
        assert_eq!(con.peek(), Some(String::from("k")));
        con.skip_space();
        assert_eq!(con.next_n(13), Some(String::from("konnichwassup")));
        con.skip_space();
        assert_eq!(con.peek(), None);

    }
}


