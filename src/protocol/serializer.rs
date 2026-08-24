use super::RespValue;

pub struct RespSerializer;

impl RespSerializer {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize(&self, value: &RespValue) -> Vec<u8> {
        match value {
            RespValue::SimpleString(s) => self.serialize_simple_string(s),
            RespValue::Error(e) => self.serialize_error(e),
            RespValue::Integer(i) => self.serialize_integer(*i),
            RespValue::BulkString(s) => self.serialize_bulk_string(s),
            RespValue::Array(items) => self.serialize_array(items),
        }
    }

    fn serialize_simple_string(&self, s: &str) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(b'+');
        result.extend_from_slice(s.as_bytes());
        result.extend_from_slice(b"\r\n");
        result
    }

    fn serialize_error(&self, e: &str) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(b'-');
        result.extend_from_slice(e.as_bytes());
        result.extend_from_slice(b"\r\n");
        result
    }

    fn serialize_integer(&self, i: i64) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(b':');
        result.extend_from_slice(i.to_string().as_bytes());
        result.extend_from_slice(b"\r\n");
        result
    }

    fn serialize_bulk_string(&self, s: &Option<Vec<u8>>) -> Vec<u8> {
        match s {
            Some(data) => {
                let mut result = Vec::new();
                result.push(b'$');
                result.extend_from_slice(data.len().to_string().as_bytes());
                result.extend_from_slice(b"\r\n");
                result.extend_from_slice(data);
                result.extend_from_slice(b"\r\n");
                result
            }
            None => {
                let mut result = Vec::new();
                result.extend_from_slice(b"$-1\r\n");
                result
            }
        }
    }

    fn serialize_array(&self, value: &Option<Vec<RespValue>>) -> Vec<u8> {
        match value {
            Some(vec) => {
                let mut result = Vec::new();
                result.push(b'*');
                result.extend_from_slice(vec.len().to_string().as_bytes());
                result.extend_from_slice(b"\r\n");
                for item in vec {
                    result.extend_from_slice(&self.serialize(item));
                }
                result
            }
            None => {
                let mut result = Vec::new();
                result.extend_from_slice(b"*-1\r\n");
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_simple_string() {
        let serializer = RespSerializer::new();

        let value = RespValue::SimpleString("OK".to_string());

        let result = serializer.serialize(&value);

        assert_eq!(result, b"+OK\r\n");
    }

    #[test]
    fn serializes_error() {
        let serializer = RespSerializer::new();

        let value = RespValue::Error("ERR unknown command".to_string());

        let result = serializer.serialize(&value);

        assert_eq!(result, b"-ERR unknown command\r\n");
    }

    #[test]
    fn serializes_integer() {
        let serializer = RespSerializer::new();

        let value = RespValue::Integer(42);

        let result = serializer.serialize(&value);

        assert_eq!(result, b":42\r\n");
    }

    #[test]
    fn serializes_negative_integer() {
        let serializer = RespSerializer::new();

        let value = RespValue::Integer(-42);

        let result = serializer.serialize(&value);

        assert_eq!(result, b":-42\r\n");
    }

    #[test]
    fn serializes_bulk_string() {
        let serializer = RespSerializer::new();

        let value = RespValue::BulkString(Some(b"hello".to_vec()));

        let result = serializer.serialize(&value);

        assert_eq!(result, b"$5\r\nhello\r\n");
    }

    #[test]
    fn serializes_empty_bulk_string() {
        let serializer = RespSerializer::new();

        let value = RespValue::BulkString(Some(Vec::new()));

        let result = serializer.serialize(&value);

        assert_eq!(result, b"$0\r\n\r\n");
    }

    #[test]
    fn serializes_null_bulk_string() {
        let serializer = RespSerializer::new();

        let value = RespValue::BulkString(None);

        let result = serializer.serialize(&value);

        assert_eq!(result, b"$-1\r\n");
    }

    #[test]
    fn serializes_empty_array() {
        let serializer = RespSerializer::new();

        let value = RespValue::Array(Some(Vec::new()));

        let result = serializer.serialize(&value);

        assert_eq!(result, b"*0\r\n");
    }

    #[test]
    fn serializes_null_array() {
        let serializer = RespSerializer::new();

        let value = RespValue::Array(None);

        let result = serializer.serialize(&value);

        assert_eq!(result, b"*-1\r\n");
    }

    #[test]
    fn serializes_array() {
        let serializer = RespSerializer::new();

        let value = RespValue::Array(Some(vec![
            RespValue::BulkString(Some(b"GET".to_vec())),
            RespValue::BulkString(Some(b"name".to_vec())),
        ]));

        let result = serializer.serialize(&value);

        assert_eq!(result, b"*2\r\n$3\r\nGET\r\n$4\r\nname\r\n");
    }

    #[test]
    fn serializes_command() {
        let serializer = RespSerializer::new();

        let value = RespValue::Array(Some(vec![
            RespValue::BulkString(Some(b"GET".to_vec())),
            RespValue::BulkString(Some(b"name".to_vec())),
        ]));

        assert_eq!(
            serializer.serialize(&value),
            b"*2\r\n$3\r\nGET\r\n$4\r\nname\r\n"
        );
    }
}
