use std::fs::read_to_string;

use crate::Post;
use minijinja::{context, Environment, Source};

pub struct HtmlRenderer<'a>(Environment<'a>);

impl<'a> HtmlRenderer<'a> {
    pub fn new() -> Self {
        let mut env = Environment::new();
        let mut source = Source::new();
        let index_template = read_to_string("./assets/index.html").unwrap();
        let post_template = read_to_string("./assets/post.html").unwrap();
        source.add_template("index", index_template).unwrap();
        source.add_template("post", post_template).unwrap();
        env.set_source(source);
        HtmlRenderer(env)
    }

    pub fn render_post(&self, post: &Post) -> String {
        let tmpl = self.0.get_template("post").unwrap();
        tmpl.render(post).unwrap()
    }

    pub fn render_index(&self, posts: &Vec<Post>) -> String {
        let tmpl = self.0.get_template("index").unwrap();
        tmpl.render(context!(posts => posts)).unwrap()
    }
}
