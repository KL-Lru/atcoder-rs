use meta::workspace_root;
use render::render;
use tera::Context;

mod contest;
mod meta;
mod render;

pub fn main() {
    let beginner_contests = contest::Beginner::contests_in_workspace();

    let mut context = Context::new();
    context.insert("beginner_contests", &beginner_contests);

    let index_path = workspace_root().join("custom/index.md");
    let content = render(context.clone()).expect("Failed to render template");
    std::fs::write(&index_path, content).expect("Failed to write file");

    println!("Index file generated at {:?}", index_path);
}
