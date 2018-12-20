
use crate::{
    parse::{Highlight, Parser, RegexPat},
    syntax::toml::Toml::*,
};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Toml {
    Text,
    Arr0,
    Arr1,
    Comment,
    Section,
    Equals,
    Punctuation,
    Name,
    String,
    Literal,
}

impl Highlight for Toml {}

const COMMENT: &str = r"#[^\n]*";

const SECTION: &str = r"(?m)\n\s*\[[^]]*]";

const NAME: &str = r"^\s*\w+\s*";

const EQUALS: &str = r"=\s*";

const LITERAL: &str = r"[\w.+\-:]+";
const STRING: &str = r###""(\\.|[^"])*""###;
const NEW_LN: &str = r"\n";

const OPEN_ARR: &str = r"\[";
const CLOSE_ARR: &str = r"]";

const COMMA: &str = r",\s*";

lazy_static! {
    static ref TEXT_REG: Vec<(RegexPat<Toml>, Toml)> = {
        vec![
            (RegexPat::regex(COMMENT,      Comment),     Text),
            (RegexPat::regex(SECTION,      Section),     Text),
            (RegexPat::regex(CLOSE_ARR,    Punctuation), Arr0),
            (RegexPat::regex(COMMA,        Punctuation), Arr0),
            (RegexPat::regex(NAME,         Name),        Equals),
        ]
    };
}
lazy_static! {
    static ref EQUALS_REG: Vec<(RegexPat<Toml>, Toml)> = {
        vec![ (RegexPat::regex(EQUALS,     Equals),      Arr0) ]
    };
}
lazy_static! {
    static ref ARR0_REG: Vec<(RegexPat<Toml>, Toml)> = {
        vec![
            (RegexPat::regex(STRING,       String),      Text),
            (RegexPat::regex(LITERAL,      Literal),     Text),
            (RegexPat::regex(OPEN_ARR,     Punctuation), Arr1),
            (RegexPat::regex(CLOSE_ARR,    Punctuation), Arr0),
            (RegexPat::regex(COMMA,        Punctuation), Arr0),
            (RegexPat::regex(COMMENT,      Comment),     Text),
            (RegexPat::regex(NEW_LN,       Text),        Text),
        ]
    };
}
lazy_static! {
    static ref ARR1_REG: Vec<(RegexPat<Toml>, Toml)> = {
        vec![
            (RegexPat::regex(STRING,       String),      Arr1),
            (RegexPat::regex(LITERAL,      Literal),     Arr1),
            (RegexPat::regex(OPEN_ARR,     Punctuation), Arr1),
            (RegexPat::regex(CLOSE_ARR,    Punctuation), Arr0),
            (RegexPat::regex(COMMENT,      Comment),     Arr1),
            (RegexPat::regex(COMMA,        Punctuation), Arr1),
        ]
    };
}


impl Toml {
    pub fn make_parser<'a>() -> Parser<'a, Toml, RegexPat<Toml>> {
        let mut parser = Parser::new(Text);
        parser.add_matcher(Text,   &TEXT_REG);
        parser.add_matcher(Equals, &EQUALS_REG);
        parser.add_matcher(Arr0,   &ARR0_REG);
        parser.add_matcher(Arr1,   &ARR1_REG);
        parser
    }
}

#[test]
pub fn test() {
    let s = r#######"# This is a TOML document.

title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00 # First class dates

[database]
server = "192.168.1.1"
ports = [ 8001, 8001, 8002 ]
connection_max = 5000
enabled = true"#######;

    let mut parser = Toml::make_parser();
    parser.parse(s);

    while let Some((s, h)) = parser.next_match() {
        //println!("~{}~      {}", s, h.get_name());
    }
}