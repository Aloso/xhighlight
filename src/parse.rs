use std::{
    fmt::{Debug, Formatter, Error},
    hash::Hash,
    collections::HashMap
};

use regex::Regex;



/// This trait stores information about how a word should be highlighted.
///
/// The easiest way to implement this is with an enum:
///
/// ```
/// use xhighlight::parse::Highlight;
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// enum MyHighlight {
///     Text,
///     Number,
///     String,
/// }
/// impl Highlight for MyHighlight {}
/// ```
pub trait Highlight : Copy + Debug + Eq + Hash {
    fn get_name(&self) -> String {
        format!("{:?}", self)
    }
}
/// Matches the pattern against a string slice. If successful,
/// it returns the end offset of the match **in bytes**.
/// The match must start exactly at 0.
pub trait Pattern<H: Highlight> : Debug {
    fn get_match(&self, str_pointer: &str, index: usize, next: H) -> Option<Match<H>>;
    fn highlight(&self) -> H;
}



pub enum RegexPat<H: Highlight> {
    Regex      { regex: Regex,               hl: H },
    AtBoundary { regex: Regex,               hl: H },
    Shortest   { regex: Regex,               hl: H },
    LookAhead  { regex: Regex, ahead: Regex, hl: H },
    OptionalLA { regex: Regex, ahead: Regex, hl: H, fhl: H },
}

impl<H: Highlight> Debug for RegexPat<H> {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
        match self {
            RegexPat::Regex      {regex: _, hl} => write!(f, "Regex<{}>", hl.get_name()),
            RegexPat::Shortest   {regex: _, hl} => write!(f, "Shortest<{}>", hl.get_name()),
            RegexPat::AtBoundary {regex: _, hl} => write!(f, "AtBoundary<{}>", hl.get_name()),
            RegexPat::LookAhead  {regex: _, ahead: _, hl} =>
                write!(f, "LookAhead<{}>", hl.get_name()),
            RegexPat::OptionalLA {regex: _, ahead: _, hl, fhl} =>
                write!(f, "OptionalLA<{}, {}>", hl.get_name(), fhl.get_name()),
        }
    }
}

impl<H: Highlight> RegexPat<H> {
    pub fn regex(regex: &str, hl: H) -> Self {
        RegexPat::Regex {
            regex: Regex::new(format!("^({})", regex).as_str()).unwrap(),
            hl
        }
    }
    pub fn shortest(regex: &str, hl: H) -> Self {
        RegexPat::Shortest {
            regex: Regex::new(format!("^({})", regex).as_str()).unwrap(),
            hl
        }
    }
    pub fn at_boundary(regex: &str, hl: H) -> Self {
        RegexPat::AtBoundary {
            regex: Regex::new(format!("^({})", regex).as_str()).unwrap(),
            hl
        }
    }
    pub fn look_ahead(regex: &str, ahead: &str, hl: H) -> Self {
        RegexPat::LookAhead {
            regex: Regex::new(format!("^({})", regex).as_str()).unwrap(),
            ahead: Regex::new(format!("^({})", ahead).as_str()).unwrap(),
            hl
        }
    }
    pub fn optional(regex: &str, ahead: &str, hl: H, fhl: H) -> Self {
        RegexPat::OptionalLA {
            regex: Regex::new(format!("^({})", regex).as_str()).unwrap(),
            ahead: Regex::new(format!("^({})", ahead).as_str()).unwrap(),
            hl,
            fhl,
        }
    }
}

impl<H: Highlight> Pattern<H> for RegexPat<H> {
    fn get_match(&self, str_slice: &str, index: usize, next: H) -> Option<Match<H>> {
        match self {
            RegexPat::Regex { regex, hl } => {
                regex.find(&str_slice[index .. ])
                    .map(|m| index + m.end())
                    .map(|end| {
                        Match { highlight: *hl, next, start: index, end }
                    })
            },
            RegexPat::Shortest { regex, hl } => {
                regex.shortest_match(&str_slice[index .. ])
                    .map(|m| index + m)
                    .map(|end| Match { highlight: *hl, next, start: index, end })
            },
            RegexPat::AtBoundary { regex, hl } => {
                regex.find_at(str_slice, index)
                    .filter(|m| m.start() == index)
                    .map(|m| Match { highlight: *hl, next, start: m.start(), end: m.end() })
            },
            RegexPat::LookAhead { regex, ahead, hl } => {
                regex.find(&str_slice[index .. ])
                    .filter(|m| ahead.is_match(&str_slice[index + m.end() .. ]))
                    .map(|m| index + m.end())
                    .map(|end| Match { highlight: *hl, next, start: index, end })
            },
            RegexPat::OptionalLA { regex, ahead, hl, fhl } => {
                let end = regex.find(&str_slice[index .. ])?.end() + index;
                let hl = if ahead.is_match(&str_slice[end .. ]) {
                    *hl
                } else {
                    *fhl
                };
                Some(Match { highlight: hl, next, start: index, end })
            },
        }
    }
    fn highlight(&self) -> H {
        match self {
            RegexPat::Regex      { regex: _,           hl         } => *hl,
            RegexPat::Shortest   { regex: _,           hl         } => *hl,
            RegexPat::AtBoundary { regex: _,           hl         } => *hl,
            RegexPat::LookAhead  { regex: _, ahead: _, hl         } => *hl,
            RegexPat::OptionalLA { regex: _, ahead: _, hl, fhl: _ } => *hl,
        }
    }
}



#[derive(Debug)]
pub struct Match<H: Highlight> {
    pub highlight: H,
    pub next: H,
    pub start: usize,
    pub end: usize,
}



#[derive(Debug)]
pub struct Matcher<'a, H: Highlight, P: Pattern<H>> {
    expressions: &'a Vec<(P, H)>,   // pattern, highlight for next pattern
}

impl<'a, H: Highlight, P: Pattern<H>> Matcher<'a, H, P> {
    pub fn new(expressions: &'a Vec<(P, H)>) -> Self {
        Matcher { expressions }
    }
    pub fn next_match(&self, str_slice: &str, mut index: usize) -> Option<Match<H>> {
        let len = str_slice.len();
        // Iterate through str_slice while it's not empty
        while index < len {
            // pat: pattern; hl: highlight for next pattern
            for (pat, hl) in self.expressions.iter() {
                let m = pat.get_match(str_slice, index, *hl);
                if m.is_some() {
                    return m;
                }
            }

            // Figure out if the next char is 1, 2, 3 or 4 bytes long
            // This is safe since str_slice can't be empty
            let next_byte = unsafe { str_slice.as_bytes().get_unchecked(index) };
            let ch_len = char_len(*next_byte);
            index += ch_len;
        }

        None
    }
}



pub struct Parser<'a, H: Highlight, P: Pattern<H>> {
    input: &'a str,
    offset: usize,
    hl: H,
    buffered: Option<Match<H>>,
    matchers: HashMap<H, Matcher<'a, H, P>>,
}

impl<'a, H: Highlight, P: Pattern<H>> Parser<'a, H, P> {
    pub fn new(default_hl: H) -> Self {
        Parser {
            input: "",
            offset: 0,
            hl: default_hl,
            buffered: None,
            matchers: HashMap::new(),
        }
    }

    pub fn add_matcher(&mut self, before: H, expressions: &'a Vec<(P, H)>) -> &mut Self {
        self.matchers.insert(before, Matcher::new(expressions));
        self
    }

    pub fn parse(&mut self, input: &'a str) -> &mut Self {
        self.input = input;
        self.offset = 0;
        self.buffered = None;
        self
    }

    pub fn offset(&mut self, offset: usize) -> Result<(), &str> {
        if offset < self.input.len() {
            self.offset = offset;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn next_match(&mut self) -> Option<(&'a str, H)> {
        match &self.buffered {
            Some(m) => {
                let string = &self.input[m.start .. m.end];
                let hl = m.highlight;

                self.hl = m.next;
                self.offset = m.end;
                self.buffered = None;
                Some((string, hl))
            },
            None => {
                let matcher = self.matchers.get(&self.hl)?;
                if let Some(m) = matcher.next_match(self.input, self.offset) {
                    if m.start > self.offset {
                        let string = &self.input[self.offset .. m.start];
                        let hl = self.hl;

                        self.hl = m.highlight;
                        self.buffered = Some(m);
                        Some((string, hl))
                    } else {
                        let string = &self.input[m.start .. m.end];
                        let hl = m.highlight;

                        self.offset = m.end;
                        self.hl = m.next;
                        self.buffered = None;
                        Some((string, hl))
                    }
                } else if self.offset < self.input.len() {
                    let res = (&self.input[self.offset .. ], self.hl);
                    self.offset = self.input.len();
                    Some(res)
                } else {
                    None
                }
            }
        }
    }
}

impl<'a, H: Highlight, P: Pattern<H>> Iterator for &mut Parser<'a, H, P> {
    type Item = (&'a str, H);
    fn next(&mut self) -> Option<(&'a str, H)> {
        self.next_match()
    }
}


/// This implementation is based on the UTF-8 binary representation.
/// Note that the most common case of ASCII characters is fastest!
///
/// This doesn't work if the byte is a continuation byte of a multi-byte code point
#[inline]
fn char_len(ch: u8) -> usize {
    if ch < 128 {
        1
    } else if (ch >> 5) == 0b110 {
        2
    } else if (ch >> 4) == 0b1110 {
        3
    } else {
        4
    }
}

#[inline]
pub unsafe fn next_char(string: &[u8], index: usize) -> char {
    let ch = string[index];
    if ch < 128 {
        ch as char
    } else {
        let (len, mut cp) = if (ch >> 5) == 0b110 {
            (1, (ch & 0b11111) as u32)
        } else if (ch >> 4) == 0b1110 {
            (2, (ch & 0b1111) as u32)
        } else {
            (3, (ch & 0b111) as u32)
        };
        cp = (cp << 6) + (string.get_unchecked(index + 1) & 0b111111) as u32;
        if len != 1 {
            cp = (cp << 6) + (string.get_unchecked(index + 2) & 0b111111) as u32;
            if len != 2 {
                cp = (cp << 6) + (string.get_unchecked(index + 3) & 0b111111) as u32;
            }
        }
        use std::mem;
        mem::transmute(cp)
    }
}

/// Checks whether a string has a word boundary at a specified index.
/// This is unsafe because it doesn't test whether the index is within the string bounds
#[inline]
pub unsafe fn is_word_boundary(string: &str, index: usize) -> bool {
    //if index == 0 {
    //    string[0]
    //}
    unimplemented!()
}