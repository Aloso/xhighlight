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

To highlight keywords, strings, numbers and comments in a language, first we create an enum with all possible tokens that implements the `Highlight` trait:

```rust
use xhighlight::parse::Highlight;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum MyLang {
    Text,
    Keyword,
    String,
    Number,
    Comment,
}

impl Highlight for MyLang {}
```

The tokens also work as _states_. We'll define the initial state as `Text` in a moment.

Before that, let's define our patterns and our pattern sets. You most certainly want to use regular expressions for patterns. A _pattern set_ is an ordered list of patterns, which are all used for the same state.

While parsing, every time a token is requested, we go forward in the string until a pattern matches the current string index. If more than one pattern matches, we use the first one, so make sure they're not in the wrong order!

We usually put pattern sets in `lazy_static!` blocks, because so the regular expressions aren't compiled more than once.

```rust
#[macro_use]
extern crate lazy_static;

// List all keywords; the regex crate uses Aho-Corasick, so it should be efficient
const KEYWORD: &str = r"\b(for|while|do|switch|case|default|continue|break|if|else|try|catch|finally|throw|synchronized|this|public|protected|private|static|final|abstract|volatile|transient|native|new|var|super|void|class|enum|interface|extends|import|package|instanceof|assert|strictfp|true|false|null|boolean|byte|char|double|float|int|long|short)\b";

// Allow double-quoted string with escape sequences
const STRING: &str = r#""(\\.|[^"])*""#;

// Numbers: Floats (e.g. 3.14159e-5) and integers in decimal, binary or hex
const NUMBER: &str = r"(\d+|\d*\.\d+)([eE][+-]?\d+)?|0x[0-9a-fA-F]+|0b[01]+";

// Allow // line comments and /* block comments */
const COMMENT: &str = r"//.*|/\*.*?\*/";

lazy_static! {
    static ref REGEXES: Vec<(RegexPat<MyLang>, MyLang)> = {
        vec![
            // The 1st tuple means that, if the string matches the KEYWORD pattern,
            // then a token of type Keyword is created, and the state is changed to Text.
            // Note that the state transition is only important for the *next* token.
            (RegexPat::regex(KEYWORD, MyLang::Keyword), MyLang::Text),
            (RegexPat::regex(COMMENT, MyLang::Comment), MyLang::Text),
            (RegexPat::regex(STRING,  MyLang::String),  MyLang::Text),
            (RegexPat::regex(NUMBER,  MyLang::Number),  MyLang::Text),
        ]
    };
}
```

Actually, this can parse a subset of the Java syntax!

Now, to offer the same API as the other languages, we should implement `make_parser()`:

```rust
impl MyLang {
    pub fn make_parser<'a>() -> Parser<'a, MyLang, RegexPat<MyLang>> {
        let mut parser = Parser::new(Text);
        
        // This means that the REGEXES pattern set is used while the state is Text.
        // The state becomes important when context-sensitive patterns are used
        // (e.g. patterns that only apply within strings, or only in annotations)
        parser.add_matcher(MyLang::Text, &REGEXES);
        
        // You can add more pattern sets for different states,
        // but in this example, it's not necessary
        
        parser
    }
}
```

How this is used, is explained in [example 1](#example-use-existing-implementation).