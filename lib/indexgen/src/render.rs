use tera::{Context, Tera};

fn templates() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template("index.md", include_str!("templates/index.md"))
        .expect("Failed to add template");
    tera
}

pub fn render(context: Context) -> Result<String, tera::Error> {
    templates().render("index.md", &context)
}
