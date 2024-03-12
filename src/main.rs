use core::app::App;

// #![allow(unused)]
use crate::init::config::AppConfig;
use cli::command_handler::parse_arguments;
use glob::Pattern;
use init::initializer::ProjectPaths;

mod cli;
mod core;
mod error;
mod init;
mod prelude;
mod utils;

fn main() {
    yew::Renderer::<App>::new().render();

    let init_flag = parse_arguments();
    if init_flag {
        let project_paths = ProjectPaths::initialize_project().unwrap();
        println!("{:?}", project_paths);

        let pattern_str = format!(
            "{}/{}",
            project_paths.test_components_path, "**/*.{html,rs}"
        );
        let pattern = Pattern::new(&pattern_str).expect("Invalid glob pattern");

        let app_conf = AppConfig {
            path_dir: pattern.to_string(),
        };

        let file_path = project_paths.config_path;

        app_conf.to_file(file_path.clone()).unwrap();
        let _config_path_value = app_conf.read_file(file_path.clone()).unwrap();
    }
}
