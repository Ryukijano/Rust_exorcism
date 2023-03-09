#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// Retourne message d'information (log Info)
pub fn info(message: &str) -> String {
    "[INFO]: ".to_string() + &message.to_string()
}

/// Retourne message d'attention (log Warning)
pub fn warn(message: &str) -> String {
    "[WARNING]: ".to_string() + &message.to_string()
}

/// Retourne message d'erreur (log Error)
pub fn error(message: &str) -> String {
    "[ERROR]: ".to_string() + &message.to_string()
}

/// Fonction qui match les logs avec le message passé en paramètre
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(&message),
        LogLevel::Warning => warn(&message),
        LogLevel::Error => error(&message),
    }
}
