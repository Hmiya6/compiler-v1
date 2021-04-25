
// TODO
// - if no problem to return &str insted of String, remove to_string-like functions

// `Iter`-like structure.
// It only deal with &str (Vec<char>) (, so that we can build code simply).
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
    
    // Return next char as `String`.
    pub fn next(&mut self) -> Option<String> {
        if let Some(c) = self.next_char() {
            Some(c.to_string())
        } else {
            None
        }
    }
    
    // Inner function for `next` and `next_n`
    // Return next char if `self.queue` has the next element,
    fn next_char(&mut self) -> Option<char> {
        if self.pos < self.queue.len() {
            let res = self.queue[self.pos];
            self.pos +=1;
            Some(res)
        } else {
            None
        }
    }
    
    // return next n chars as `String`
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
    
    // return string from `self.pos` to next white space
    pub fn next_until_space(&mut self) -> Option<String> {
        let mut vec = Vec::new();
        loop {
            match self.peek_char() {
                Some(c) => {
                    match c {
                        ' ' | '\t' => {
                            break;
                        }
                        _ => {
                            self.next();
                            vec.push(c)
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }
        let res = vec.into_iter().collect::<String>();
        Some(res)
    }
    
    // return a char as `String`
    pub fn peek(&self) -> Option<String> {
        if let Some(c) = self.peek_char() {
            Some(c.to_string())
        } else {
            None
        }

    }
    
    // inner function for mainly `peek` and `peek_n`
    fn peek_char(&self) -> Option<char> {
        let res: char;
        if self.pos < self.queue.len() {
            res = self.queue[self.pos];
            Some(res)
        } else {
            None
        }
    }
    
    // return chars as `String`
    pub fn peek_n(&self, n: usize) -> Option<String> {
        let mut vec = Vec::new();
        for i in 0..n {
            if self.pos+i < self.queue.len() {
                vec.push(self.queue[self.pos+i]);
            } else {
                return None;
            }
        }
        let res = vec.iter().collect::<String>();
        Some(res)
    }
    
    // return `usize` integer
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
    
    // skip white spaces
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
        consumer_next_until_space();
    }

    #[test]
    fn consumer_next() {
        let mut con = Consumer::new("consumer");
        assert_eq!(con.next(), Some("c".to_string()));
        assert_eq!(con.next_n(3), Some("ons".to_string()));
        assert_eq!(con.next_n(1000), None);
        assert_eq!(con.next(), None);
    }

    #[test]
    fn consumer_next_until_space() {
        let mut con = Consumer::new("This is a pen.\t Say hi.");
        assert_eq!(con.next_until_space(), Some("This".to_string()));
        con.skip_space();
        assert_eq!(con.next_until_space(), Some("is".to_string()));
        con.skip_space();
        assert_eq!(con.next_until_space(), Some("a".to_string()));
        con.skip_space();
        assert_eq!(con.next_until_space(), Some("pen.".to_string()));
        con.skip_space();
        assert_eq!(con.next_until_space(), Some("Say".to_string()));
    }

    #[test]
    fn consumer_peek() {
        let mut con = Consumer::new("Hello people");
        assert_eq!(con.peek(), Some("H".to_string()));
        assert_eq!(con.next_n(4), Some("Hell".to_string()));
        assert_eq!(con.peek_n(1000), None);
        assert_eq!(con.peek_n(8), Some("o people".to_string()));
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
        assert_eq!(con.peek(), Some("k".to_string()));
        con.skip_space();
        assert_eq!(con.next_n(13), Some("konnichwassup".to_string()));
        con.skip_space();
        assert_eq!(con.peek(), None);

    }
}


