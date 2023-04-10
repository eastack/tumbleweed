use selaginella::{Config, PostParser, HtmlRenderer};
use std::fs::{self, read_to_string, write};

fn main() -> anyhow::Result<()> {
    let config = Config::new("config.toml");
    let posts_dir = config.post_dir();
    let publish_dir = config.publish_dir();

    let posts: Vec<_> = fs::read_dir(&posts_dir)?
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or(Default::default()) == "dj")
        .map(|path| PostParser::new(path).parse())
        .collect();

    let mut render = HtmlRenderer::new();
    let index_template = read_to_string("./assets/index.html")?;
    let post_template = read_to_string("./assets/post.html")?;
    render.add_template("index", &index_template);
    render.add_template("post", &post_template);

    for post in &posts {
        let path = &post.path.strip_prefix(&posts_dir)?;
        let path = path.with_extension("html");
        let path = publish_dir.join(path);

        fs::create_dir_all(&publish_dir)?;
        let post = render.render_post(post);
        write(path, post)?;
    }

    let posts: Vec<_> = posts
        .into_iter()
        .map(|mut post| {
            post.path = post.path.strip_prefix(&posts_dir).unwrap().to_path_buf();
            post.path.set_extension("html");
            post
        })
        .collect();

    let index = render.render_index(&posts);
    write(publish_dir.join("index.html"), index)?;

    Ok(())
}
