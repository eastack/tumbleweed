use jotdown::Container::Heading;
use jotdown::Container::Section;
use jotdown::Event;
use jotdown::Parser;
use jotdown::Render;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct PostMeta {
    author: String,
    title: String,
}

pub struct PostParser<'a>(Parser<'a>);

impl<'a> PostParser<'a> {
    pub fn new(doc: &'a str) -> Self {
        Self(jotdown::Parser::new(&doc))
    }

    pub fn parse(&self) -> Post {
        let mut content = String::new();
        jotdown::html::Renderer::default()
            .push(self.0.clone(), &mut content)
            .unwrap();
        let metadata = self.metadata();

        Post {
            title: metadata.title,
            author: metadata.author,
            content,
        }
    }

    pub fn metadata(&self) -> PostMeta {
        let mut author = Default::default();
        let mut title = Default::default();
        let mut first_section = true;
        let mut heading_start = false;

        for event in self.0.clone() {
            match event {
                Event::Start(Section { .. }, attributes) if first_section => {
                    author = String::from_iter(attributes.get("author").unwrap().parts());
                    first_section = false;
                }
                Event::Start(Heading { level: 1, .. }, ..) => {
                    heading_start = true;
                }
                Event::Str(heading) if heading_start => {
                    title = heading.to_string();
                    heading_start = false;
                }
                _ => {}
            }
        }

        PostMeta { author, title }
    }
}
