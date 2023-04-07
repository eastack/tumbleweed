use crate::Post;
use minijinja::Environment;

pub struct Renderer<'a> {
    env: Environment<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.add_template(
            "post",
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}} - {{author}}</title>
</head>
<body>
{{content}}</body>
</html>"#,
        )
        .unwrap();
        Renderer { env }
    }

    pub fn render(&self, post: Post) -> String {
        let tmpl = self.env.get_template("post").unwrap();
        tmpl.render(post).unwrap()
    }
}
