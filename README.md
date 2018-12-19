# xhighlight

This crate can simplify implementing a parser for **any** language, and format the output.

## How to use it
There are several ways how to use this crate:

- Use a **built-in** parser and renderer ([see below](#supported-languages))
- Create a **new** parser for a language and/or a new renderer (e.g. for terminal, latex or pdf output)
- Replace the `Pattern` implementation for maximum control; however, this is error-prone and therefore _not recommended_.

## Supported languages

| Language | Limitations |
| -------- | ----------- |
| **Rust** | No support for raw string literals with more than 10 hashtags (`r###########"Hi"###########`) |
| **Toml** | No support for `{` blocks `}` **_yet_** |
| **Javascript** | Very limited support for template strings |


## Performance

Websites are fast if they require little javascript code – especially on mobiles with bad internet connection. But of course, if the task is done on the server instead, this code should be efficient.

In my tests, the performance of xhighlight was _acceptable_ on a cold start, but significantly better _after a few runs_. This makes it well suited for servers such as [Rocket](https://rocket.rs). Static site generators will profit even more.

## Example: Use existing implementation

This is how you highlight Rust using HTML/CSS:

```rust
use xhighlight::syntax::rust::Rust;
use xhighlight::render::{Renderer, HtmlRenderer};

// Define the CSS classes for tokens
// E.g. a keyword is encoded as <span class="kwd">..</span>
const RUST_CSS: [(Rust, &str); 19] = [
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
    (Rust::Char,          "str"),
    (Rust::LineComment,   "com"),
    (Rust::BlockComment,  "com"),
    (Rust::DocComment,    "doc"),
    (Rust::RawLiteral,    ""),
];

pub fn parse_rust_to_html(string: &str) -> String {
    let mut parser = Rust::make_parser();
    
    HtmlRenderer::new(&mut parser)
        .set_mapping(&RUST_CSS)
        .render(string)
}
```

## Example: Highlight your own language

To highlight keywords, strings, numbers and comments in a language, first create an enum that implements the `Highlight` trait:

```rust
use xhighlight::parse::Highlight;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum MyLanguage {
    Text,
    Keyword,
    String,
    Number,
    Comment,
}

impl Highlight for MyLanguage {}
```

This defines the supported kinds of tokens. The default token is `Text`, which will receive no styling.

Now, define your regular expressions, and define your pattern sets (explained in the comments):

```rust
#[macro_use]
extern crate lazy_static;

// List all keywords; the regex crate uses Aho-Corasick, so it should be efficient
const KEYWORD: &str = r"\b(for|while|do|switch|case|default|continue|break|if|else|try|catch|finally|throw|synchronized|this|public|protected|private|static|final|abstract|volatile|transient|native|new|var|super|void|class|enum|interface|extends|import|package|instanceof|assert|strictfp|true|false|null|boolean|byte|char|double|float|int|long|short)\b";

// Allow double-quoted string with escape sequences
const STRING: &str = r#""(\\.|[^"])*""#;

// Numbers: Floats (e.g. 3.14159e-5) and integers in decimal, binary or hex
const NUMBERS: &str = r"(\d+|\d*\.\d+)([eE][+-]?\d+)?|0x[0-9a-fA-F]+|0b[01]+";

// Allow // line comments and /* block comments */
const COMMENTS: &str = r"//.*|/\*.*?\*/";

lazy_static! {
    // Use lazy_static to compile the regexes only once
    
    // This is a pattern set. Note that the order is important:
    // When two patterns match at the same index, it will choose the first pattern
    static ref REGEXES: Vec<(RegexPat<MyLanguage>, MyLanguage)> = {
        vec![
            // This means that, if the string matches the KEYWORD pattern,
            // then a token of type Keyword is created, and the state is changed to Text.
            (RegexPat::regex(KEYWORD, Keyword), Text),
            (RegexPat::regex(COMMENT, Comment), Text),
            (RegexPat::regex(STRING,  String),  Text),
            (RegexPat::regex(NUMBER,  Number),  Text),
        ]
    };
}
```

Actually, this can parse a subset of the Java syntax!

Now, to offer the same API as the other languages, implement `make_builder()`:

```rust
impl MyLanguage {
    pub fn make_parser<'a>() -> Parser<'a, MyLanguage, RegexPat<MyLanguage>> {
        let mut parser = Parser::new(Text);
        
        // This means that the REGEXES pattern set is used while the state is Text
        // The state becomes important when context-sensitive patterns are used
        // (e.g. patterns that only apply within strings, or only in annotations)
        parser.add_matcher(Text, &REGEXES);
        
        // You can add more pattern sets for different states,
        // but in this example, it's not necessary
        
        parser
    }
}
```

How this is used, is explained in [example 1](#example-use-existing-implementation).