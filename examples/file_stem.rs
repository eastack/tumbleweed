use std::path::Path;

fn main() {
    let file_stem = Path::new("/tmp/hello/world.rs").file_name().unwrap().to_str().unwrap();
    println!("File stem: {file_stem}");
}
