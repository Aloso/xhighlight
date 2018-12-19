
use crate::{
    parse::{Highlight, Parser, RegexPat},
    syntax::rust::Rust::*,
};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Rust {
    Text,
    Keyword,
    Identifier,
    Lifetime,
    Operator,
    Punctuation,
    Annotation,
    LineComment,
    BlockComment,
    DocComment,
    PrimitiveType,
    String,
    StringEscape,
    Char,
    Number,
    Bool,
    RawLiteral,
    FnCall,
    MacroCall,
}

impl Highlight for Rust {}


const KEYWORD: &str = r"(as|async|box|break|const|continue|crate|dyn|else|enum|extern|fn|for|if|impl|in|let|loop|match|mod|move|mut|pub|ref|return|[sS]elf|static|struct|super|trait|type|union|unsafe|use|where|while|yield)\b";

const LIFETIME: &str = r"'[\p{Alphabetic}_]\w*\b";

const OPERATOR: &str = r"[=&+\-*/%~!|@^?]|<<|>>";
const PUNCTUATION: &str = r"[.,:;()\[\]{}<>][.,:;()\[\]{}<\-> ]*";
const ANNOTATION: &str = r"#!?\[.*?]";

const TYPE: &str = r"(bool|char|[ui](8|16|32|64|128|size)|f32|f64|Option|Some|None|Result|Ok|Err|String|str)\b";

const DOC_COMMENT: &str = r"//[/!].*|/\*[/*].*?\*/";
const LINE_COMMENT: &str = r"//.*";
const BLOCK_COMMENT: &str = r"/\*.*?\*/";

const NUMBER: &str = r"(?x)
    0b ( [01] _* )+                         ( [ui] (8|1(6|28)|32|64|size) )? |
    0x ( [\da-fA-F] _* )+                   ( [ui] (8|1(6|28)|32|64|size) )? |
    (\d_*)+ ( \. (\d_*)+ )? ( [eE] [+-]? _* (\d_*)+ )? ( [ui] (8|1(6|28)|32|64|size) )?
";

const BOOL: &str = r"\b(true|false)\b";

const RAW_LITERAL: &str = r"r#[\p{Alphabetic}_]\w*\b";

const MACRO: &str = r"[\p{Alphabetic}_]\w*!";
const FUNCTION: &str = r"[\p{Alphabetic}_]\w*\b";
const FUNCTION_AHEAD: &str = r"\s*\(";

const CHAR: &str = r"'[^'\\]'|'\\.+'";
const STRING_QUOTE: &str = r#"b?""#;
const STRING_ESC: &str = r#"\\([nrt\\0'"\n]|x[\da-fA-F]{2}|u[\da-fA-F]{1,6})"#;
// This should be enough:
const RAW_STRING: &str = r#################"(?sx)
b?r(      ".*?"          |
         #".*?"#         |
        ##".*?"##        |
       ###".*?"###       |
      ####".*?"####      |
     #####".*?"#####     |
    ######".*?"######    |
   #######".*?"#######   |
  ########".*?"########  |
 #########".*?"######### |
##########".*?"##########)"#################;

lazy_static! {
    static ref TEXT_REG: Vec<(RegexPat<Rust>, Rust)> = {
        vec![
            (RegexPat::shortest  (KEYWORD,                  Keyword),            Text),
            (RegexPat::regex     (DOC_COMMENT,              DocComment),         Text),
            (RegexPat::regex     (LINE_COMMENT,             LineComment),        Text),
            (RegexPat::shortest  (BLOCK_COMMENT,            BlockComment),       Text),
            (RegexPat::shortest  (OPERATOR,                 Operator),           Text),
            (RegexPat::regex     (PUNCTUATION,              Punctuation),        Text),
            (RegexPat::shortest  (TYPE,                     PrimitiveType),      Text),
            (RegexPat::shortest  (BOOL,                     Bool),               Text),
            (RegexPat::shortest  (RAW_LITERAL,              RawLiteral),         Text),
            (RegexPat::optional  (FUNCTION, FUNCTION_AHEAD, FnCall, Identifier), Text),
            (RegexPat::regex     (NUMBER,                   Number),             Text),
            (RegexPat::shortest  (MACRO,                    MacroCall),          Text),
            (RegexPat::regex     (RAW_STRING,               String),             Text),
            (RegexPat::shortest  (CHAR,                     Char),               Text),
            (RegexPat::shortest  (LIFETIME,                 Lifetime),           Text),
            (RegexPat::regex     (STRING_QUOTE,             String),             String),
            (RegexPat::shortest  (ANNOTATION,               Annotation),         Text),
        ]
    };
}
lazy_static! {
    static ref STRING_REG: Vec<(RegexPat<Rust>, Rust)> = {
        vec![
            (RegexPat::shortest  (STRING_ESC,   StringEscape), String),
            (RegexPat::shortest  (STRING_QUOTE, String),       Text),
        ]
    };
}


impl Rust {
    pub fn make_parser<'a>() -> Parser<'a, Rust, RegexPat<Rust>> {
        let mut parser = Parser::new(Text);
        parser.add_matcher(Text, &TEXT_REG);
        parser.add_matcher(String, &STRING_REG);
        parser
    }
}
