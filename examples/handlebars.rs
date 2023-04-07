use std::collections::BTreeMap;

use handlebars::Handlebars;

fn main() {
    let mut handlebars = Handlebars::new();

    let source = "hello {{world}}";
    assert!(handlebars.register_template_string("t1", source).is_ok());

    let mut data = BTreeMap::new();
    data.insert("world".to_string(), "世界！".to_string());
    println!("{}", handlebars.render("t1", &data).unwrap())
}
