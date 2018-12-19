use std::collections::HashMap;

use crate::parse::{Parser, Highlight, Pattern};

pub trait Renderer<'a> {
    fn render(&mut self, s: &'a str) -> String;
}

pub struct HtmlRenderer<'a, H: Highlight, P: Pattern<H>> {
    parser: &'a mut Parser<'a, H, P>,
    class_map: HashMap<H, &'a str>,
}

impl<'a, H: Highlight, P: Pattern<H>> HtmlRenderer<'a, H, P> {
    pub fn new(parser: &'a mut Parser<'a, H, P>) -> Self {
        Self {
            parser,
            class_map: HashMap::with_capacity(0),
        }
    }
    pub fn set_mapping(mut self, class_map: &[(H, &'a str)]) -> Self {
        self.class_map = HashMap::with_capacity(class_map.len());
        for &(hl, s) in class_map {
            self.class_map.insert(hl, s);
        }
        self
    }
}

impl<'a, H: Highlight, P: Pattern<H>> Renderer<'a> for HtmlRenderer<'a, H, P> {
    fn render(&mut self, s: &'a str) -> String {
        self.parser.parse(s);

        let mut s = String::new();
        for (token, hl) in &mut self.parser {
            let &cls = self.class_map.get(&hl).unwrap_or(&"");
            if cls.is_empty() {
                s.push_str(token);
            } else {
                s.push_str("<span class=\"");
                s.push_str(cls);
                s.push_str("\">");
                s.push_str(token.replace("<", "&lt;").as_str());
                s.push_str("</span>");
            }
        }
        s
    }
}