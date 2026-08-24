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
            .ok_or_else(|| self.invalid_resp())?;
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
        let data_start = first_line_end + 2;
        let data_end = data_start + len;
        if input.len() < data_end + 2 || &input[data_end..data_end + 2] != b"\r\n" {
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
            .ok_or_else(|| self.invalid_resp())?;
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
                return Err(self.invalid_resp());
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
    // fn incomplete_resp(&self) -> Error {
    //     Error::new(ErrorKind::UnexpectedEof, "Incomplete RESP message")
    // }

    fn read_line<'a>(&self, input: &'a [u8]) -> Result<(&'a str, usize)> {
        let line_end = input
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or_else(|| self.invalid_resp())?;

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

        let (result, consumed) =
            parser.parse(b"+OK\r\n").unwrap();

        assert_eq!(
            result,
            RespValue::SimpleString("OK".to_string())
        );

        assert_eq!(consumed, 5);
    }
}