use crate::protocol::RespValue;

use super::Command;

pub struct CommandHandler;

impl CommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, command: Command) -> RespValue {
        match command {
            Command::Ping => RespValue::SimpleString("PONG".to_string()),
            Command::Echo(message) => RespValue::BulkString(Some(message)),
        }
    }
}
