#![feature(test)]
extern crate test;
use test::Bencher;

use xhighlight::syntax::rust::Rust;
use xhighlight::render::{HtmlRenderer, Renderer};

#[bench]
pub fn bench_parse(b: &mut Bencher) {
    let mut parser = Rust::make_parser();
    let mut renderer = HtmlRenderer::new(&mut parser)
        .set_mapping(&[
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
        ]);
    b.iter(|| {
        renderer.render(STRING);
    });
}


#[bench]
pub fn bench_parse_shorter(b: &mut Bencher) {
    let mut parser = Rust::make_parser();
    let mut renderer = HtmlRenderer::new(&mut parser)
        .set_mapping(&[
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
        ]);
    b.iter(|| {
        renderer.render(SHORTER);
    });
}

#[bench]
pub fn bench_build(b: &mut Bencher) {
    Rust::make_parser();

    b.iter(|| {
        HtmlRenderer::new(&mut Rust::make_parser())
            .set_mapping(&[
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
            ]);
    });
}


#[cfg(test)]
pub mod string {
    use test::Bencher;

    const STRING1: &str = "abcdefg";
    const STRING2: &str = "äöüßéáì";
    const STRING3: &str = "ÄÖÜ¿É×Ŧ";

    #[bench]
    pub fn next_iter(b: &mut Bencher) {
        b.iter(|| {
            let c1 = (STRING1[3 ..]).chars().next().unwrap();
            let c2 = (STRING2[4 ..]).chars().next().unwrap();
            let c3 = (STRING3[6 ..]).chars().next().unwrap();
            let c4 = (STRING1[2 ..]).chars().next().unwrap();

            (c1, c2, c3, c4)
        })
    }

    //#[bench]
    //pub fn next_own(b: &mut Bencher) {
    //    b.iter(|| {
    //        let c1 = parse::next_char(STRING1, 3);
    //        let c2 = parse::next_char(STRING2, 4);
    //        let c3 = parse::next_char(STRING3, 5);
    //        let c4 = parse::next_char(STRING1, 2);
    //
    //        (c1, c2, c3, c4)
    //    })
    //}

}






const STRING: &'static str = r########"fn main() {
    // Let's use the singleton in a few threads
    let threads: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 10));
                let s = singleton();
                let mut data = s.inner.lock().unwrap();
                *data = i as u8;
            })
        })
        .collect();

    // And let's check the singleton every so often
    for _ in 0u8..20 {
        thread::sleep(Duration::from_millis(5));

        let s = singleton();
        let data = s.inner.lock().unwrap();
        println!("It is: {}", *data);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}

#[ignore]
impl Rust {
    pub fn make_parser<'a>() -> Parser<'a, Rust, RegexPat<Rust>> {
        let s = Some("Hello \n World!");
        let t = 0x10_usize;

        let mut parser = Parser::new(Text);
        parser.add_matcher(Text, &TEXT_REG);
        parser.add_matcher(String, &STRING_REG);
        parser
    }

    pub fn make_fancy_parser<'a>() -> Parser<'a, Rust, FancyPat<Rust>> {
        let mut parser = Parser::new(Text);
        parser.add_matcher(Text, &FANCY_TEXT);
        parser.add_matcher(String, &FANCY_STRING);
        parser
    }
}"########;

const SHORTER: &'static str = r########"fn main() {
    // Let's use the singleton in a few threads
    let threads = (0..10)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 10));
                let mut data = singleton().inner.lock().unwrap();
                *data = i as u8;
            })
        });
}"########;