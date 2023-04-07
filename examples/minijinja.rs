fn main() {
    use minijinja::{context, Environment};

    let mut env = Environment::new();
    env.add_template("hello", "Hello {{ name }}!").unwrap();
    let tmpl = env.get_template("hello").unwrap();
    println!("{}", tmpl.render(context!(name => "John")).unwrap());
}
