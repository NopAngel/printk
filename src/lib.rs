//   lib.rs  -  lib
use colored::*;
use regex::Regex;
use std::io::{self, Write};
use thiserror::Error;

lazy_static::lazy_static! {
    static ref ICON_MAP: Vec<(&'static str, &'static str)> = vec![
        // Files & Folders
        ("folder", ""),
        ("file", ""),
        ("doc", ""),
        ("img", ""),
        ("pdf", ""),
        ("zip", ""),

        // Status
        ("ok", ""),
        ("check", ""),
        ("success", ""),
        ("error", ""),
        ("fail", ""),
        ("warning", ""),
        ("warn", ""),
        ("info", ""),
        ("question", ""),
        ("help", ""),

        // Arrows
        ("arrow", ""),
        ("arrow-right", ""),
        ("arrow-left", ""),
        ("arrow-up", ""),
        ("arrow-down", ""),

        // UI
        ("gear", ""),
        ("settings", ""),
        ("home", ""),
        ("star", ""),
        ("heart", ""),
        ("trash", ""),
        ("edit", ""),
        ("add", ""),
        ("plus", ""),
        ("minus", ""),
        ("close", ""),
        ("search", ""),

        // Git
        ("git", ""),
        ("branch", ""),
        ("commit", ""),
        ("merge", ""),

        // Programming
        ("rust", ""),
        ("python", ""),
        ("js", ""),
        ("ts", ""),
        ("java", ""),
        ("go", ""),
        ("c", ""),
        ("cpp", ""),

        // OS
        ("linux", ""),
        ("apple", ""),
        ("windows", ""),

        // Media
        ("play", ""),
        ("pause", ""),
        ("stop", ""),
        ("volume", ""),
        ("mute", ""),

        // Network
        ("wifi", ""),
        ("network", ""),
        ("cloud", ""),
        ("download", ""),
        ("upload", ""),

        // Time
        ("clock", ""),
        ("calendar", ""),
        ("time", ""),

        // People
        ("user", ""),
        ("users", ""),
        ("lock", ""),
        ("unlock", ""),

        ("demo", ""),
    ];

    static ref ICON_REGEX: Regex = Regex::new(r"\{([a-zA-Z0-9\-_]+)\}").unwrap();
}

#[derive(Error, Debug)]
pub enum PrintkError {
    #[error("Invalid cursor position: {0}")]
    InvalidCursor(i16),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

pub struct Printk {
    default_color: Option<Color>,
    show_icons: bool,
}

impl Printk {
    pub fn new() -> Self {
        Self {
            default_color: None,
            show_icons: true,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.default_color = Some(color);
        self
    }

    pub fn icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    pub fn print_at_y(&self, y: i16, message: &str) -> Result<(), PrintkError> {
        if y < 0 {
            return Err(PrintkError::InvalidCursor(y));
        }

        print!("\x1B[{};1H", y + 1);

        let processed = self.process_message(message);
        print!("{}", processed);
        io::stdout().flush()?;

        Ok(())
    }


    pub fn print_at_y_and_return(&self, y: i16, message: &str) -> Result<(), PrintkError> {
        print!("\x1B[s");

        self.print_at_y(y, message)?;


        print!("\x1B[u");
        io::stdout().flush()?;

        Ok(())
    }


    pub fn print(&self, message: &str) -> Result<(), PrintkError> {
        let processed = self.process_message(message);
        print!("{}", processed);
        io::stdout().flush()?;
        Ok(())
    }

    pub fn println(&self, message: &str) -> Result<(), PrintkError> {
        let processed = self.process_message(message);
        println!("{}", processed);
        Ok(())
    }


    pub fn process_message(&self, message: &str) -> String {
        if !self.show_icons {
            return message.to_string();
        }

        let mut result = message.to_string();


        for (key, icon) in ICON_MAP.iter() {
            let pattern = format!("{{{}}}", key);
            result = result.replace(&pattern, icon);
        }


        if let Some(color) = self.default_color {
            result = result.color(color).to_string();
        }

        result
    }

    pub fn get_icon(&self, icon_name: &str) -> Option<String> {
        if !self.show_icons {
            return Some(icon_name.to_string());
        }

        for (key, icon) in ICON_MAP.iter() {
            if key == &icon_name {
                return Some(icon.to_string());
            }
        }
        None
    }


    pub fn list_icons(&self) -> Vec<(String, String)> {
        ICON_MAP.iter()
            .map(|(name, icon)| (name.to_string(), icon.to_string()))
            .collect()
    }


    pub fn clear_from_y(&self, y: i16) -> Result<(), PrintkError> {
        if y < 0 {
            return Err(PrintkError::InvalidCursor(y));
        }

        print!("\x1B[{};1H\x1B[0K", y + 1);
        io::stdout().flush()?;
        Ok(())
    }


    pub fn clear_lines(&self, start_y: i16, count: u16) -> Result<(), PrintkError> {
        if start_y < 0 {
            return Err(PrintkError::InvalidCursor(start_y));
        }

        for i in 0..count {
            print!("\x1B[{};1H\x1B[0K", start_y + i as i16 + 1);
        }
        io::stdout().flush()?;
        Ok(())
    }
}

pub fn printk(message: &str) -> Result<(), PrintkError> {
    Printk::new().print(message)
}

pub fn printk_at_y(y: i16, message: &str) -> Result<(), PrintkError> {
    Printk::new().print_at_y(y, message)
}

pub fn printk_color(color: Color, message: &str) -> Result<(), PrintkError> {
    Printk::new().with_color(color).print(message)
}
