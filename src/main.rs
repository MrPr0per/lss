use crate::app_config::AppConfig;
use crate::events::{Event, EventStorage};
use colored::Colorize;
use std::io;
use std::path::PathBuf;
mod app_config;
mod events;

fn main() {
    // пока сборка и интерфейс будут перемешаны, потом спросить возможно ли разделить
    let stdin = io::stdin();
    let config_storage = app_config::ConfigStorage::new();
    let config = config_storage.load().unwrap_or_else(|| {
        println!("Enter the path to the project (empty string = current directory)");
        let path = {
            let mut path_string = String::new();
            stdin
                .read_line(&mut path_string)
                .expect("Failed to read string");
            path_string = path_string.trim().to_string();
            if path_string.is_empty() {
                path_string = ".".to_string()
            }
            PathBuf::from(&path_string)
        };
        let config = AppConfig { project_path: path };
        config_storage.save(&config);
        config
    });

    let events_storage = EventStorage {
        events_path: config.project_path.join("events.jsons"),
    };
    events_storage
        .try_open()
        .unwrap_or_else(|e| panic!("Failed to open events storage: {e}"));

    println!("Enter events:");
    for line in stdin.lines() {
        let line = line.expect("Failed to read string");
        let event = &Event::new(line);
        println!(
            "{} {}",
            event
                .timestamp
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .as_str()
                .green(),
            event.content
        );
        events_storage
            .append(event)
            .unwrap_or_else(|e| panic!("Failed to save line, {e}"));
    }
}
