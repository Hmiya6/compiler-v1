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
