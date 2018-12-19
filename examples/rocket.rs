#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

use xhighlight::syntax::rust::Rust;
use xhighlight::syntax::toml::Toml;
use xhighlight::syntax::javascript::JS;
use xhighlight::render::{Renderer, HtmlRenderer};


#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::Html<String> {
    let mut file = File::open("examples/index.html").expect("File could not be opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File could not be read");
    content::Html(contents)
}

#[get("/ajax?<data>")]
fn ajax<'a>(data: String) -> String {
    let tm = SystemTime::now();

    let s = HtmlRenderer::new(&mut JS::make_parser())
        .set_mapping(&[
            (JS::Text,           ""),
            (JS::CommonType,     "typ"),
            (JS::Punctuation,    "pun"),
            (JS::String,         "str"),
            (JS::Identifier,     ""),
            (JS::BlockComment,   "com"),
            (JS::LineComment,    "com"),
            (JS::StringEscape,   "esc"),
            (JS::Number,         "num"),
            (JS::FnCall,         "fun"),
            (JS::Operator,       "opr"),
            (JS::Keyword,        "kwd"),
            (JS::Bool,           "boo"),
            (JS::Regex,          "reg"),
            (JS::TemplateString, "tpl"),
            (JS::TplInner,       "tpi"),
        ])
        .render(data.as_str());
    let tm2 = SystemTime::now().duration_since(tm)
        .expect("Time went backwards");

    let millis = tm2.subsec_millis();
    let micros = tm2.subsec_micros() % 1000;
    let nanos = tm2.subsec_nanos() % 1000;
    println!("calculated in s/ms/mc/ns  {}:{}:{}:{}", tm2.as_secs(), millis, micros, nanos);
    s
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, ajax])
        .launch();
}