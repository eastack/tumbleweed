use std::path::Path;

fn main() {
    let path = Path::new("./public");
    let path = path.join("test").with_file_name("index.html");

    println!("Path: {path:?}");
    println!("Dir?: {}", path.is_dir());
    println!("File?: {}", path.is_file());
    path.to
}
