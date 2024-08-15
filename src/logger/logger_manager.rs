use std::{fs::{File, OpenOptions}, io::Write, path::PathBuf,path::Path};
use chrono::prelude::*;
pub enum LogType{
    Information,
    Error,
    Warning
}

pub struct Logger<'a> {
    message: &'a str,
    log_type: LogType,
}

impl<'a> Logger<'a> {
    pub fn log_info(message: &'a str) -> Self {
        Logger {
            message,
            log_type: LogType::Information,
        }
    }

    pub fn log_error(message: &'a str) -> Self {
        Logger {
            message,
            log_type: LogType::Error,
        }
    }

    pub fn log_warn(message: &'a str) -> Self {
        Logger {
            message,
            log_type: LogType::Warning,
        }
    }

    pub fn write_log_to_file(&self) {
        let mut file_dir_path:PathBuf = std::env::current_exe().unwrap(); 
        file_dir_path.push("..");
        file_dir_path.push("Logs");
        let now: DateTime<Local> = Local::now();
        let formatted_date = now.format("%d-%m-%Y %H:%M:%S %A").to_string();
        if !file_dir_path.exists() {
            std::fs::create_dir_all(file_dir_path.clone());
            file_dir_path.push("logger.txt");
            if !file_dir_path.exists(){
                File::create(file_dir_path.clone()).expect("Failed to create logger file");
            }
        }
        file_dir_path.push("logger.txt");
        let mut file = OpenOptions::new()
            .append(true)
            .open(file_dir_path.clone())
            .expect("Failed to open logger file");
        let formatted_message = match self.log_type {
            LogType::Information => format!("[INFO] at:{} => {}\n",formatted_date, self.message),
            LogType::Error => format!("[ERROR] at:{} =>  {}\n",formatted_date, self.message),
            LogType::Warning => format!("[WARNING] at:{} => {}\n",formatted_date, self.message),
        };
        file.write_all(formatted_message.as_bytes())
            .expect("Failed to write to logger file");
    }
}