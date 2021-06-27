
use std::io;
use std::str;

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    pub fn token<T>(&mut self) -> T
    where
        T: str::FromStr
    {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn tokens<T>(&mut self) -> Vec<T>
        where
            T: str::FromStr
    {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        return input.split_whitespace().map(|s| T::from_str(s).ok().expect("parse error")).collect();
    }

    pub fn pairs<T>(&mut self, count : u32) -> Vec<Pair<T>>
        where
            T: Copy + str::FromStr
    {
        let mut vec = Vec::new();
        for _i in 0..count {
            let v = self.tokens();
            vec.push(Pair{ x: v[0], y: v[1]});
        }
        vec
    }
}

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input
}

pub fn print_vec<A:std::fmt::Display, W: io::Write>(v : &Vec<A>, out: &mut W) {
    writeln!(out, "{}", v.len()).ok();
    for u in v {
        write!(out, "{} ", *u);
    }
}

pub fn char_to_u8(c : char) -> u8 {
    c as u8 - 'A' as u8
}

pub fn u8_to_char(u : u8) -> char {
    (u + 'A' as u8) as char
}
