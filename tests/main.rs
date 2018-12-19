use xhighlight::syntax::rust::Rust;
use xhighlight::render::{Renderer, HtmlRenderer};

#[macro_use]
extern crate lazy_static;

const RUST_CSS: [(Rust, &str); 19] = [
    // Here, you define the CSS classes for tokens
    // E.g. a keyword is encoded as <span class="kwd">...</span>
    (Rust::Text,          ""),
    (Rust::Identifier,    ""),
    (Rust::Keyword,       "kwd"),
    (Rust::Operator,      "opr"),
    (Rust::FnCall,        "fun"),
    (Rust::Punctuation,   "pun"),
    (Rust::Lifetime,      "lif"),
    (Rust::PrimitiveType, "typ"),
    (Rust::Number,        "num"),
    (Rust::Bool,          "boo"),
    (Rust::MacroCall,     "mac"),
    (Rust::Annotation,    "ann"),
    (Rust::String,        "str"),
    (Rust::StringEscape,  "esc"),
    (Rust::Char,          "chr"),
    (Rust::LineComment,   "com"),
    (Rust::BlockComment,  "com"),
    (Rust::DocComment,    "doc"),
    (Rust::RawLiteral,    "raw"),
];

const TEST_STR: &str = r#####################################"
    /// This is some serious shit!
    /// This, too.
    #[allow(dead_code)]
    pub struct HelloWorld { x: bool, y: String } // test
    fn main() {
        let mut my_val = HelloWorld { x: /* blah */ true, y: r#######"Hello \n \"World!\""####### };
        my_val.y += 3;
    }"#####################################;

#[test]
pub fn test() {

    let mut parser = Rust::make_parser();

    let output = HtmlRenderer::new(&mut parser)
        .set_mapping(&RUST_CSS)
        .render(TEST_STR);

    println!("{}", output);
}

#[cfg(test)]
pub mod self_impl {
    use xhighlight::parse::Highlight;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    enum MyLanguage {
        Text,
        Keyword,
        Str,
        Number,
        Comment,
    }
    impl Highlight for MyLanguage {}

    // List all keywords; this should be efficient thanks to regex optimizations
    const KEYWORD: &str = r"\b(for|while|do|switch|case|default|continue|break|if|else|try|catch|finally|throw|synchronized|this|public|protected|private|static|final|abstract|volatile|transient|native|new|var|super|void|class|enum|interface|extends|import|package|instanceof|assert|strictfp|true|false|null|boolean|byte|char|double|float|int|long|short)\b";

    // Allow double-quoted string with escape sequences
    const STRING: &str = r#""(\\.|[^"])*""#;

    // Numbers: Floats (e.g. 3.14159e-5) and integers in decimal, binary or hex
    const NUMBER: &str = r"(\d+|\d*\.\d+)([eE][+-]?\d+)?|0x[0-9a-fA-F]+|0b[01]+";

    // Allow // line comments and /* block comments */
    const COMMENT: &str = r"//.*|/\*.*?\*/";

    use xhighlight::parse::{RegexPat, Parser};
    use xhighlight::render::{Renderer, HtmlRenderer};
    use self::MyLanguage::*;

    const MY_MAPPING: [(MyLanguage, &str); 5] = [
        (Text,    ""),
        (Keyword, "kwd"),
        (Str,     "str"),
        (Number,  "num"),
        (Comment, "com"),
    ];

    lazy_static! {
        // Use lazy_static to compile the regexes only once
        // Note that the order is important:
        // When two regexes match at a string index, it will always choose the first one
        static ref REGEXES: Vec<(RegexPat<MyLanguage>, MyLanguage)> = {
            vec![
                (RegexPat::regex(KEYWORD, Keyword), Text),
                (RegexPat::regex(COMMENT, Comment), Text),
                (RegexPat::regex(STRING,  Str),     Text),
                (RegexPat::regex(NUMBER,  Number),  Text),
            ]
        };
    }

    pub fn parse_java_to_html(string: &str) -> String {
        let mut parser = Parser::new(Text);
        parser.add_matcher(Text, &REGEXES);

        HtmlRenderer::new(&mut parser)
            .set_mapping(&MY_MAPPING)
            .render(string)
    }

    #[test]
    pub fn test() {
        println!("{}", parse_java_to_html(r#"public class HelloWorld {\
        private String s = "Hello World";
    }"#));
    }
}