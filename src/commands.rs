use crate::errors::Errors;
use std::str::FromStr;


#[derive(Debug)]
pub enum Commands {
    Get,
    Put,
    // Delete,
    // Exit,
}

impl FromStr for Commands {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(Commands::Get),
            "put" => Ok(Commands::Put),
            // "exit" => Ok(Commands::Exit),
            _ => Err(Errors::UnknownCommand),
        }
    }
}
