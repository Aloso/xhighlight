
use crate::{
    parse::{Highlight, Parser, RegexPat},
    syntax::javascript::JS::*,
};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum JS {
    Text,
    Keyword,
    Identifier,
    Operator,
    CommonType,
    Punctuation,
    LineComment,
    BlockComment,
    String,
    StringEscape,
    Number,
    Bool,
    FnCall,
    Regex,
    TemplateString,
    TplInner
}

impl Highlight for JS {}

const KEYWORD: &str = r"(abstract|arguments|await|boolean|break|byte|case|catch|char|class|const|continue|debugger|default|delete|do|double|else|enum|eval|export|extends|final|finally|float|for|function|goto|if|implements|import|in|instanceof|int|interface|let|long|native|new|null|package|private|protected|public|return|short|static|super|switch|synchronized|this|throw|throws|transient|try|typeof|var|void|volatile|while|with|yield)\b";

const OPERATOR: &str = r"[=&+\-*/%~!|^?:<>]\s*";
const PUNCTUATION: &str = r"[.,;()\[\]{}]\s*";

const TYPE: &str = r"(prototype|constructor|window|get|set|Object|Array|Function|Math|RegExp|Date|Boolean|Number|String)\b";

const LINE_COMMENT: &str = r"//.*";
const BLOCK_COMMENT: &str = r"/\*.*?\*/";

const NUMBER: &str = r"0b[01]+|0x[0-9a-fA-F]+|0o[0-7]+|(\d*\.\d+|\d+)([eE][+-]?\d+)?";

const BOOL: &str = r"\b(true|false)\b";

const FUNCTION: &str = r"[\p{Alphabetic}_]\w*\b";
const FUNCTION_AHEAD: &str = r"\s*\(";

const STRING: &str = r###""(\\.|[^"])*"|'(\\.|[^'])*'"###;

const TPL_STR_R: &str = r###"`[^`]*?\$\{"###;
const TPL_STR: &str = r###"`.*?`"###;
const TPL_STR_LR: &str = r###"}[^`]*?\$\{"###;
const TPL_STR_L: &str = r###"}.*?`"###;

const REG_KWD: &str = r"(else|in|instanceof|typeof)\b\s*";
const REG_OPER: &str = r"[=&+\-*/%~!|^?:<>]\s*";
const REG_PUNCT: &str = r"[,;()\[\]{}]\s*";

const REGEX: &str = r###"/(\\.|[^*/])(\\.|[^/])*/i?g?m?y?"###;

lazy_static! {
    static ref TEXT_REG: Vec<(RegexPat<JS>, JS)> = {
        vec![
            (RegexPat::regex     (LINE_COMMENT,             LineComment),        Text),
            (RegexPat::shortest  (BLOCK_COMMENT,            BlockComment),       Text),

            (RegexPat::look_ahead(REG_KWD, REGEX,           Keyword),            Regex),
            (RegexPat::look_ahead(REG_OPER, REGEX,          Operator),           Regex),
            (RegexPat::look_ahead(REG_PUNCT, REGEX,         Punctuation),        Regex),

            (RegexPat::shortest  (KEYWORD,                  Keyword),            Text),
            (RegexPat::shortest  (OPERATOR,                 Operator),           Text),
            (RegexPat::regex     (PUNCTUATION,              Punctuation),        Text),
            (RegexPat::shortest  (TYPE,                     CommonType),         Text),
            (RegexPat::shortest  (BOOL,                     Bool),               Text),
            (RegexPat::optional  (FUNCTION, FUNCTION_AHEAD, FnCall, Identifier), Text),
            (RegexPat::regex     (NUMBER,                   Number),             Text),
            (RegexPat::regex     (STRING,                   String),             Text),

            (RegexPat::regex     (TPL_STR_R,                TemplateString),     TplInner),
            (RegexPat::regex     (TPL_STR,                  TemplateString),     Text),
        ]
    };
}
lazy_static! {
    static ref REGEX_REG: Vec<(RegexPat<JS>, JS)> = {
        vec![ (RegexPat::regex   (REGEX,                    Regex),              Text) ]
    };
}
lazy_static! {
    static ref TPL_REG: Vec<(RegexPat<JS>, JS)> = {
        vec![
            (RegexPat::regex     (TPL_STR_LR,               TemplateString),     TplInner),
            (RegexPat::regex     (TPL_STR_L,                TemplateString),     Text),
        ]
    };
}

impl JS {
    pub fn make_parser<'a>() -> Parser<'a, JS, RegexPat<JS>> {
        let mut parser = Parser::new(Text);
        parser.add_matcher(Text, &TEXT_REG);
        parser.add_matcher(Regex, &REGEX_REG);
        parser.add_matcher(TplInner, &TPL_REG);
        parser
    }
}
