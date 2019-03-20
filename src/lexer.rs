/*
* Copyright (C) 2019, Miklos Maroti
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::fmt;
use std::str;

pub const OPERATORS: &'static str = "()[],=.";

#[derive(Debug, Clone)]
pub enum Item<'a> {
    Error(&'a str),     // TODO: change it to &'static
    Identifier(String), // TODO: change it to &'a str
    Operator(char),
    Integer(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    line: u32,
    col: u32,
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        return fmt::Debug::fmt(self, f);
    }
}

impl fmt::Display for Pos {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "line {} col {}", self.line, self.col);
    }
}

pub struct Lexer<'a> {
    next: Option<(usize, char)>,
    iter: str::CharIndices<'a>,
    pos: Pos,
    data: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut iter = data.char_indices();
        return Lexer {
            next: iter.next(),
            iter: iter,
            pos: Pos { line: 1, col: 1 },
            data: data,
        };
    }

    fn eat_whitespace(self: &mut Self, c: char) {
        if c == '\n' {
            self.pos.line += 1;
            self.pos.col = 1;
        } else {
            self.pos.col += 1;
        }
        self.next = self.iter.next();
    }

    fn get_error(self: &mut Self, msg: &'static str) -> Item<'a> {
        self.next = None;
        return Item::Error(msg);
    }

    fn add_digit(n: u32, d: u32) -> Option<u32> {
        return match n.checked_mul(10) {
            Some(n2) => match n2.checked_add(d) {
                Some(n3) => Some(n3),
                None => None,
            },
            None => None,
        };
    }

    fn get_integer(self: &mut Self) -> Item<'a> {
        let mut n: u32 = 0;
        while let Some((_, c)) = self.next {
            match c.to_digit(10) {
                Some(d) => match Lexer::add_digit(n, d) {
                    Some(n2) => n = n2,
                    None => return self.get_error("too large integer"),
                },
                None => break,
            }
            self.pos.col += 1;
            self.next = self.iter.next();
        }
        return Item::Integer(n);
    }

    fn get_identifier(self: &mut Self) -> Item<'a> {
        let mut s = String::new();
        while let Some((_, c)) = self.next {
            if c.is_alphanumeric() {
                s.push(c);
                self.pos.col += 1;
                self.next = self.iter.next();
            } else {
                break;
            }
        }
        return Item::Identifier(s);
    }

    fn get_operator(self: &mut Self) -> Item<'a> {
        let c = self.next.unwrap().1;
        self.pos.col += 1;
        self.next = self.iter.next();
        return Item::Operator(c);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Item<'a>, Pos);

    fn next(self: &mut Self) -> Option<Self::Item> {
        while let Some((_, c)) = self.next {
            let p = self.pos;
            if c.is_alphabetic() {
                return Some((self.get_identifier(), p));
            } else if c.is_digit(10) {
                return Some((self.get_integer(), p));
            } else if OPERATORS.contains(c) {
                return Some((self.get_operator(), p));
            } else if c.is_whitespace() {
                self.eat_whitespace(c);
            } else {
                return Some((self.get_error("unexpected character"), p));
            }
        }
        return None;
    }
}
