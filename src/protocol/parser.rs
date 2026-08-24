use std::{
    io::{Error, ErrorKind, Result},
    str::from_utf8,
};

use super::RespValue;

pub struct RespParser;

impl RespParser {
    pub fn new() -> Self {
        Self
    }
    pub fn parse(&self, bytes: &[u8]) -> Result<(RespValue, usize)> {
        match bytes.first() {
            Some(b'+') => self.parse_simple_string(bytes),
            Some(b'-') => self.parse_error(bytes),
            Some(b':') => self.parse_integer(bytes),
            Some(b'*') => self.parse_array(bytes),
            Some(b'$') => self.parse_bulk_string(bytes),
            None => Err(self.incomplete_resp()),
            _ => Err(self.invalid_resp()),
        }
    }
    fn parse_simple_string(&self, input: &[u8]) -> Result<(RespValue, usize)> {
        let (line, consumed) = self.read_line(input)?;

        Ok((RespValue::SimpleString(line.to_string()), consumed))
    }
    fn parse_error(&self, input: &[u8]) -> Result<(RespValue, usize)> {
        let (line, consumed) = self.read_line(input)?;

        Ok((RespValue::Error(line.to_string()), consumed))
    }
    fn parse_integer(&self, input: &[u8]) -> Result<(RespValue, usize)> {
        let (line, consumed) = self.read_line(input)?;

        let int_val = line.parse::<i64>().map_err(|_| self.invalid_resp())?;
        Ok((RespValue::Integer(int_val), consumed))
    }
    fn parse_bulk_string(&self, input: &[u8]) -> Result<(RespValue, usize)> {
        let first_line_end = input
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or_else(|| self.incomplete_resp())?;
        let len_bytes = &input[1..first_line_end];
        let len_str = from_utf8(len_bytes).map_err(|_| self.invalid_resp())?;
        let len = len_str.parse::<i64>().map_err(|_| self.invalid_resp())?;
        if len == -1 {
            return Ok((RespValue::BulkString(None), first_line_end + 2));
        }
        if len < 0 {
            return Err(self.invalid_resp());
        }
        let len = len as usize;
        let data_start = first_line_end
            .checked_add(2)
            .ok_or_else(|| self.invalid_resp())?;
        let data_end = data_start
            .checked_add(len)
            .ok_or_else(|| self.invalid_resp())?;

        if input.len() < data_end + 2 {
            return Err(self.incomplete_resp());
        }

        if &input[data_end..data_end + 2] != b"\r\n" {
            return Err(self.invalid_resp());
        }

        let data_byte = &input[data_start..data_end];

        Ok((
            RespValue::BulkString(Some(data_byte.to_vec())),
            data_end + 2,
        ))
    }
    fn parse_array(&self, input: &[u8]) -> Result<(RespValue, usize)> {
        let first_line_end = input
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or_else(|| self.incomplete_resp())?;
        let count_bytes = &input[1..first_line_end];
        let count_str = from_utf8(count_bytes).map_err(|_| self.invalid_resp())?;
        let count = count_str.parse::<i64>().map_err(|_| self.invalid_resp())?;
        if count == -1 {
            return Ok((RespValue::Array(None), first_line_end + 2));
        }

        if count < 0 {
            return Err(self.invalid_resp());
        }
        let mut elements = Vec::with_capacity(count as usize);
        let mut current_idx = first_line_end + 2;
        for _ in 0..count {
            if current_idx >= input.len() {
                return Err(self.incomplete_resp());
            }

            let (element, bytes_consumed) = self.parse(&input[current_idx..])?;

            elements.push(element);
            current_idx += bytes_consumed;
        }

        Ok((RespValue::Array(Some(elements)), current_idx))
    }
    fn invalid_resp(&self) -> Error {
        Error::new(ErrorKind::InvalidData, "Invalid RESP message")
    }
    fn incomplete_resp(&self) -> Error {
        Error::new(ErrorKind::UnexpectedEof, "Incomplete RESP message")
    }

    fn read_line<'a>(&self, input: &'a [u8]) -> Result<(&'a str, usize)> {
        let line_end = input
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or_else(|| self.incomplete_resp())?;

        let line = from_utf8(&input[1..line_end]).map_err(|_| self.invalid_resp())?;

        // +2 for "\r\n"
        Ok((line, line_end + 2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_string() {
        let parser = RespParser::new();
        let (result, consumed) = parser.parse(b"+OK\r\n").unwrap();
        assert_eq!(result, RespValue::SimpleString("OK".to_string()));
        assert_eq!(consumed, 5);
    }

    #[test]
    fn parses_error() {
        let parser = RespParser::new();
        let (result, consumed) = parser.parse(b"-ERR unknown command\r\n").unwrap();
        assert_eq!(result, RespValue::Error("ERR unknown command".to_string()));
        assert_eq!(consumed, 22);
    }

    #[test]
    fn parses_integer() {
        let parser = RespParser::new();
        let (result, consumed) = parser.parse(b":1000\r\n").unwrap();
        assert_eq!(result, RespValue::Integer(1000));
        assert_eq!(consumed, 7);
    }

    #[test]
    fn parses_bulk_string() {
        let parser = RespParser::new();
        let (result, consumed) = parser.parse(b"$5\r\nhello\r\n").unwrap();
        assert_eq!(result, RespValue::BulkString(Some(b"hello".to_vec())));
        assert_eq!(consumed, 11);
    }

    #[test]
    fn parses_null_bulk_string() {
        let parser = RespParser::new();
        let (result, consumed) = parser.parse(b"$-1\r\n").unwrap();
        assert_eq!(result, RespValue::BulkString(None));
        assert_eq!(consumed, 5);
    }

    #[test]
    fn parses_array() {
        let parser = RespParser::new();
        let (result, consumed) = parser.parse(b"*2\r\n$3\r\nGET\r\n$4\r\nname\r\n").unwrap();

        let expected = RespValue::Array(Some(vec![
            RespValue::BulkString(Some(b"GET".to_vec())),
            RespValue::BulkString(Some(b"name".to_vec())),
        ]));
        assert_eq!(result, expected);
        assert_eq!(consumed, 23);
    }

    #[test]
    fn incomplete_empty_bytes() {
        let parser = RespParser::new();
        let err = parser.parse(b"").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn incomplete_simple_string_missing_crlf() {
        let parser = RespParser::new();
        let err = parser.parse(b"+OK").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn incomplete_bulk_string_missing_length_crlf() {
        let parser = RespParser::new();
        let err = parser.parse(b"$5").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn incomplete_bulk_string_truncated_payload() {
        let parser = RespParser::new();
        let err = parser.parse(b"$5\r\nhel").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn incomplete_bulk_string_missing_terminating_crlf() {
        let parser = RespParser::new();
        let err = parser.parse(b"$5\r\nhello").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn incomplete_array_header_missing_crlf() {
        let parser = RespParser::new();
        let err = parser.parse(b"*2").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn incomplete_array_missing_elements() {
        let parser = RespParser::new();
        let err = parser.parse(b"*2\r\n$3\r\nGET\r\n").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn parses_first_message_and_preserves_remaining_bytes() {
        let parser = RespParser::new();

        let input = b"+OK\r\n:42\r\n";

        let (result, consumed) = parser.parse(input).unwrap();

        assert_eq!(result, RespValue::SimpleString("OK".to_string()));

        assert_eq!(consumed, 5);

        let (result, consumed) = parser.parse(&input[consumed..]).unwrap();

        assert_eq!(result, RespValue::Integer(42));
        assert_eq!(consumed, 5);
    }

    #[test]
    fn parses_nested_array() {
        let parser = RespParser::new();

        let input = b"*2\r\n:1\r\n*1\r\n$4\r\nPING\r\n";

        let (result, consumed) = parser.parse(input).unwrap();

        let expected = RespValue::Array(Some(vec![
            RespValue::Integer(1),
            RespValue::Array(Some(vec![RespValue::BulkString(Some(b"PING".to_vec()))])),
        ]));

        assert_eq!(result, expected);
        assert_eq!(consumed, input.len());
    }

    #[test]
    fn invalid_unknown_type_byte() {
        let parser = RespParser::new();
        let err = parser.parse(b"PING\r\n").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn invalid_integer_format() {
        let parser = RespParser::new();
        let err = parser.parse(b":abc\r\n").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn invalid_bulk_string_wrong_crlf_delimiter() {
        let parser = RespParser::new();
        // Fully received bytes, but payload closure is not \r\n (it's \tx)
        let err = parser.parse(b"$5\r\nhello\tx").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn invalid_bulk_string_negative_out_of_bounds() {
        let parser = RespParser::new();
        let err = parser.parse(b"$-5\r\n").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }
}
