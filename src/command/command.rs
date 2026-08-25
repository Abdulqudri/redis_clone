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

        let command_name = String::from_utf8_lossy(command_bytes).to_ascii_uppercase();
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

#[cfg(test)]
mod command_pipeline_tests {
    use super::*;
    use crate::protocol::RespValue;

    #[test]
    fn test_valid_ping_conversion() {
        let input = RespValue::Array(Some(vec![RespValue::BulkString(Some(b"ping".to_vec()))]));
        assert_eq!(Command::from_resp(input), Ok(Command::Ping));
    }

    #[test]
    fn test_valid_echo_conversion() {
        let input = RespValue::Array(Some(vec![
            RespValue::BulkString(Some(b"ECHO".to_vec())),
            RespValue::BulkString(Some(b"hello".to_vec())),
        ]));
        assert_eq!(
            Command::from_resp(input),
            Ok(Command::Echo(b"hello".to_vec()))
        );
    }

    #[test]
    fn test_unknown_command() {
        let input = RespValue::Array(Some(vec![RespValue::BulkString(Some(
            b"NOT_A_COMMAND".to_vec(),
        ))]));
        assert_eq!(
            Command::from_resp(input),
            Err(RespValue::Error(
                "ERR unknown command 'NOT_A_COMMAND'".into()
            ))
        );
    }

    #[test]
    fn test_empty_command() {
        let input = RespValue::Array(Some(vec![]));
        assert_eq!(
            Command::from_resp(input),
            Err(RespValue::Error(
                "ERR syntax error: empty command array".into()
            ))
        );
    }

    #[test]
    fn test_null_array() {
        let input = RespValue::Array(None);
        assert_eq!(
            Command::from_resp(input),
            Err(RespValue::Error(
                "ERR syntax error: command cannot be a null array".into()
            ))
        );
    }

    #[test]
    fn test_argument_errors() {
        // PING with an extra argument
        let bad_ping = RespValue::Array(Some(vec![
            RespValue::BulkString(Some(b"PING".to_vec())),
            RespValue::BulkString(Some(b"extra".to_vec())),
        ]));
        assert_eq!(
            Command::from_resp(bad_ping),
            Err(RespValue::Error(
                "ERR wrong number of arguments for 'PING' command".into()
            ))
        );

        // ECHO with missing argument
        let bad_echo = RespValue::Array(Some(vec![RespValue::BulkString(Some(b"ECHO".to_vec()))]));
        assert_eq!(
            Command::from_resp(bad_echo),
            Err(RespValue::Error(
                "ERR wrong number of arguments for 'ECHO' command".into()
            ))
        );
    }
}
