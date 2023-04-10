mod config;
mod parser;
mod render;

pub use config::*;
pub use parser::*;
pub use render::*;

use std::fs::{self, write};
use std::path::PathBuf;

struct Selaginella<'a> {
    posts_dir: PathBuf,
    publish_dir: PathBuf,
    posts: Vec<Post>,
    render: HtmlRenderer<'a>,
}

impl<'a> Selaginella<'a> {
    fn build(config: Config) -> anyhow::Result<Self> {
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

    fn resurrection(&self) -> anyhow::Result<()> {
        for post in &self.posts {
            let path = &post.path.strip_prefix(&self.posts_dir)?;
            let path = path.with_extension("html");
            let path = self.publish_dir.join(path);

            fs::create_dir_all(&self.publish_dir)?;
            let post = self.render.render_post(&post);

            write(path, post)?;

            let posts: Vec<_> = self
                .posts
                .iter_mut()
                .map(|mut post| {
                    post.path = post
                        .path
                        .strip_prefix(&self.posts_dir)
                        .unwrap()
                        .to_path_buf();
                    post.path.set_extension("html");
                    post
                })
                .collect();

            let index = self.render.render_index(posts);
            write(self.publish_dir.join("index.html"), index)?;
        }
        Ok(())

        //Ok(())
    }
}
