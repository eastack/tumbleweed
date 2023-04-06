use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use jotdown::Container::Heading;
use jotdown::Container::Section;
use jotdown::Event;
use jotdown::Parser;
use jotdown::Render;

fn main() {
    let filepath = "/home/radix10/desktop/personal/tumbleweed/README.dj";
    let doc = read_to_string(filepath).unwrap();
    let parser = PostParser::new(&doc);
    let out_filename = get_out_filename(filepath);
    println!("Out filename:{out_filename}");
    let mut file = File::create(out_filename).unwrap();
    let html = template(parser.parse());
    file.write_all(html.as_bytes()).unwrap();
}

fn get_out_filename(filename: &str) -> String {
    let file_stem = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    format!("{file_stem}.html")
}

fn template(
    Post {
        content,
        title,
        author,
        ..
    }: Post,
) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - {author}</title>
</head>
<body>
{content}</body>
</html>"#
    )
}

#[derive(Debug)]
struct Post {
    author: String,
    title: String,
    content: String,
}

#[derive(Debug)]
struct PostMeta {
    author: String,
    title: String,
}

struct PostParser<'a>(Parser<'a>);

impl<'a> PostParser<'a> {
    fn new(doc: &'a str) -> Self {
        Self(jotdown::Parser::new(&doc))
    }

    fn parse(&self) -> Post {
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

    fn metadata(&self) -> PostMeta {
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
