use crate::protocol::RespValue;

#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Echo(Vec<u8>),
}

impl Command {
    pub fn from_resp(value: RespValue) -> Result<Self, RespValue> {
        let elements = match value {
            RespValue::Array(Some(items)) => items,
            RespValue::Array(None) => {
                return Err(RespValue::Error("Empty command array".to_string()));
            }
            _ => {
                return Err(RespValue::Error(
                    "Invalid command format: command must be an array".to_string(),
                ));
            }
        };

        if elements.is_empty() {
            return Err(RespValue::Error("ERR empty command array".into()));
        }
        let command_bytes = match &elements[0] {
            RespValue::BulkString(Some(bytes)) => bytes,
            _ => {
                return Err(RespValue::Error(
                    "ERR syntax error: command name must be a bulk string".into(),
                ));
            }
        };

        let command_name = String::from_utf8_lossy(command_bytes).to_uppercase();
        match command_name.as_str() {
            "PING" => {
                if elements.len() != 1 {
                    return Err(RespValue::Error(
                        "ERR wrong number of arguments for 'PING' command".into(),
                    ));
                }
                Ok(Command::Ping)
            }
            "ECHO" => {
                if elements.len() != 2 {
                    return Err(RespValue::Error(
                        "ERR wrong number of arguments for 'ECHO' command".into(),
                    ));
                }
                let message = match &elements[1] {
                    RespValue::BulkString(Some(bytes)) => bytes.clone(),
                    _ => {
                        return Err(RespValue::Error(
                            "ERR syntax error: ECHO argument must be a bulk string".into(),
                        ));
                    }
                };
                Ok(Command::Echo(message))
            }
            _ => Err(RespValue::Error(format!(
                "ERR unknown command '{}'",
                command_name
            ))),
        }
    }
}
