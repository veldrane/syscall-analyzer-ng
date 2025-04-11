// Removed conflicting import: use tokio::net::unix::pipe::Sender;
use tokio::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Builder;
use chrono::prelude::*;
use colored::Colorize;



pub struct Logger {
    tx: mpsc::Sender<LogMessage>,
    _logger: tokio::task::JoinHandle<()>,
    _rt: tokio::runtime::Runtime,
}

enum LogType {
    Info,
    Warning,
    Error,
}

struct LogMessage(LogType,String);

impl Logger {


    pub fn build() -> Self {
        

        let (tx, rx): (mpsc::Sender<LogMessage>, mpsc::Receiver<LogMessage>) = mpsc::channel(100);

        let rt = Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("logger")
        .enable_all()
        .build().expect("Failed to create runtime");
    
        let logger = rt.block_on(async {
            tokio::spawn(log_receiver(rx))
        });



        let logger = Logger {
            tx,
            _logger: logger,
            _rt: rt,
        };

        Logger::info(&logger, "starting logger".to_string());

        return logger;
    }

    pub fn info(&self, message: String) {
        let _ = self.tx.blocking_send(LogMessage(LogType::Info, message));
    }

    pub fn warn(&self, message: String) {
        let _ = self.tx.blocking_send(LogMessage(LogType::Warning, message));
    }

    pub fn error(&self, message: String) {
        let _ = self.tx.blocking_send(LogMessage(LogType::Error, message));
    }
}




async fn log_receiver(mut rx: mpsc::Receiver<LogMessage>) {

    let timestamp =  Utc::now().to_string();
    
    while let Some(log_message) = rx.recv().await {

        match log_message.0 {
            LogType::Info => println!("{} {} {}",timestamp.blue(),"---| Info:".blue(), log_message.1.blue()),
            LogType::Warning => println!("{}  ---| Warning: {}",timestamp, log_message.1),
            LogType::Error => println!("{} {} {}",timestamp.red(),"---| Error:".red(), log_message.1.red()),
        }
    }
}