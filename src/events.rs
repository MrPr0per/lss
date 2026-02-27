use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: DateTime<Local>,
    pub content: String,
}

impl Event {
    pub fn new(content: String) -> Self {
        Self {
            timestamp: Local::now(),
            content,
        }
    }
}

pub struct EventStorage {
    pub events_path: PathBuf,
}

impl EventStorage {
    pub fn try_open(&self) -> io::Result<()> {
        let events_path = &self.events_path;
        if let Some(parent) = events_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let _file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(events_path)?;
        Ok(())
    }
    pub fn append(&self, event: &Event) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.events_path)?;
        let json =
            serde_json::to_string(event).expect(&format!("Failed to serialize event: {:?}", event));
        writeln!(file, "{json}")?;
        Ok(())
    }

    pub fn read_all(&self) -> io::Result<Vec<Event>> {
        let file = File::open(&self.events_path)?;
        let reader = BufReader::new(file);
        let mut events = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<Event>(&line) {
                Ok(event) => events.push(event),
                Err(e) => eprintln!("Failed to parse line: {e}"),
            }
        }

        Ok(events)
    }
}
