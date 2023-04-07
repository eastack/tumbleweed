use std::fs::{self, read_to_string};
use std::path::Path;
use tumbleweed::{PostParser, Renderer};

const POST_NAME: &str = "index.dj";

fn main() {
    let config = tumbleweed::Config::new("config.toml");
    let root = Path::new(".");
    let publish_dir = Path::new(&config.publish_dir);

    let post_paths: Vec<_> = fs::read_dir(root.join("posts"))
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.path())
        .collect();

    for post_path in post_paths {
        let djot_doc = read_to_string(post_path.join(POST_NAME)).unwrap();
        let parser = PostParser::new(&djot_doc);
        let post = parser.parse();

        let html = Renderer::new().render(post);

        let post_path_name = post_path.file_name().unwrap();
        let out_path = publish_dir.join(post_path_name);
        fs::create_dir_all(&out_path).unwrap();

        let out_filename = get_out_filename(POST_NAME);
        let out_filename = out_path.join(out_filename);
        fs::write(out_filename, html).unwrap();
    }
}

fn get_out_filename(filename: &str) -> String {
    let file_stem = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    format!("{file_stem}.html")
}
