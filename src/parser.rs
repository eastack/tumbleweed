use std::fs::read_to_string;
use std::path::PathBuf;

use chrono::Local;
use jotdown::Container::Heading;
use jotdown::Container::Section;
use jotdown::Event;
use jotdown::Parser;
use jotdown::Render;

use crate::Post;

#[derive(Debug)]
pub struct PostParser {
    path: PathBuf,
}

impl PostParser {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn parse(self) -> Post {
        let document = read_to_string(&self.path).unwrap();
        let parser = Parser::new(&document);

        let mut html = String::new();
        jotdown::html::Renderer::default()
            .push(parser.clone(), &mut html)
            .unwrap();

        let mut author = Default::default();
        let mut title = Default::default();
        let mut date = Default::default();
        let mut first_section = true;
        let mut heading_start = false;

        for event in parser {
            match event {
                Event::Start(Section { .. }, attributes) if first_section => {
                    author = String::from_iter(attributes.get("author").unwrap().parts());
                    date = Local::now();
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

        Post {
            path: self.path,
            title,
            date,
            author,
            html,
        }
    }
}
