mod config;
mod parser;
mod render;

use chrono::{DateTime, Local};
pub use config::*;
pub use parser::*;
pub use render::*;
use serde::Serialize;

use std::fs::{self, write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Clone)]
pub struct Post {
    pub path: PathBuf,
    pub author: String,
    pub title: String,
    pub html: String,
    pub date: DateTime<Local>,
}

pub struct Index<'a> {
    pub posts: &'a Vec<Post>,
    pub author: String,
    pub title: String,
    pub html: String,
    pub date: DateTime<Local>,
}

pub struct Selaginella<'a> {
    posts_dir: PathBuf,
    publish_dir: PathBuf,
    posts: Vec<Post>,
    render: HtmlRenderer<'a>,
}

impl<'a> Selaginella<'a> {
    pub fn build(config: Config) -> anyhow::Result<Self> {
        let posts_dir = config.post_dir();
        let publish_dir = config.publish_dir();

        let paths = fs::read_dir(&posts_dir)?
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_type().unwrap().is_file())
            .map(|entry| entry.path())
            .filter(|path| path.extension().unwrap_or(Default::default()) == "dj");

        let posts = paths
            .map(|path| PostParser::new(path))
            .map(|parser| parser.parse())
            .collect();

        let render = HtmlRenderer::new();

        Ok(Selaginella {
            posts_dir,
            publish_dir,
            posts,
            render,
        })
    }

    pub fn resurrection(&self) -> anyhow::Result<()> {
        for post in &self.posts {
            let path = self
                .publish_dir
                .join(post.path.file_stem().unwrap())
                .with_extension("html");

            fs::create_dir_all(&self.publish_dir)?;
            let post = self.render.render_post(&post);

            write(path, post)?;

            let posts = self
                .posts
                .clone()
                .into_iter()
                .map(|post| Post {
                    path: post
                        .path
                        .strip_prefix(&self.posts_dir)
                        .unwrap()
                        .to_path_buf().with_extension("html"),
                    ..post
                })
                .collect();

            let index = self.render.render_index(&posts);
            write(self.publish_dir.join("index.html"), index)?;
        }
        Ok(())
    }
}
