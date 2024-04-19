use std::env;
use clap::Parser;
use cli::Cli;
use app::App;
mod cli;
mod app;
mod common;

fn main() {
    let cli = Cli::parse();
    let mut app = App::default();

    if let Some(path) = &cli.path {
        app.path = path.clone();
    } else {
        app.path = env::current_dir().unwrap();
    }
    app.lang = app.lang;

    app.scan();
    app.del();
    app.show_info();

    // println!("app: {:?}", app.files);
}
