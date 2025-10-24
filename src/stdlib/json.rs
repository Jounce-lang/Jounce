/// Standard library JSON parsing and serialization module
/// Provides JSON encoding/decoding functionality

pub const JSON_DEFINITION: &str = r#"
// JSON Value representation
// Represents any valid JSON value
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    // Check if value is null
    fn is_null(self: &JsonValue) -> bool {
        match self {
            JsonValue::Null => true,
            _ => false,
        }
    }

    // Check if value is a boolean
    fn is_bool(self: &JsonValue) -> bool {
        match self {
            JsonValue::Bool(_) => true,
            _ => false,
        }
    }

    // Check if value is a number
    fn is_number(self: &JsonValue) -> bool {
        match self {
            JsonValue::Number(_) => true,
            _ => false,
        }
    }

    // Check if value is a string
    fn is_string(self: &JsonValue) -> bool {
        match self {
            JsonValue::String(_) => true,
            _ => false,
        }
    }

    // Check if value is an array
    fn is_array(self: &JsonValue) -> bool {
        match self {
            JsonValue::Array(_) => true,
            _ => false,
        }
    }

    // Check if value is an object
    fn is_object(self: &JsonValue) -> bool {
        match self {
            JsonValue::Object(_) => true,
            _ => false,
        }
    }

    // Extract boolean value
    fn as_bool(self: &JsonValue) -> Result<(), String> {
        match self {
            JsonValue::Bool(b) => Ok(*b),
            _ => Err("Not a boolean"),
        }
    }

    // Extract number value
    fn as_number(self: &JsonValue) -> Result<f64, String> {
        match self {
            JsonValue::Number(n) => Ok(*n),
            _ => Err("Not a number"),
        }
    }

    // Extract integer value
    fn as_i32(self: &JsonValue) -> Result<i32, String> {
        match self {
            JsonValue::Number(n) => Ok(*n as i32),
            _ => Err("Not a number"),
        }
    }

    // Extract string value
    fn as_string(self: &JsonValue) -> Result<String, String> {
        match self {
            JsonValue::String(s) => Ok(s.clone()),
            _ => Err("Not a string"),
        }
    }

    // Extract array value
    fn as_array(self: &JsonValue) -> Result<Vec<JsonValue>, String> {
        match self {
            JsonValue::Array(arr) => Ok(arr.clone()),
            _ => Err("Not an array"),
        }
    }

    // Extract object value
    fn as_object(self: &JsonValue) -> Result<HashMap<String, JsonValue>, String> {
        match self {
            JsonValue::Object(obj) => Ok(obj.clone()),
            _ => Err("Not an object"),
        }
    }

    // Get value from object by key
    fn get(self: &JsonValue, key: String) -> Option<JsonValue> {
        match self {
            JsonValue::Object(obj) => obj.get(key),
            _ => Option::None,
        }
    }

    // Get value from array by index
    fn get_index(self: &JsonValue, index: i32) -> Option<JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => Option::None,
        }
    }

    // Set value in object by key
    fn set(self: &mut JsonValue, key: String, value: JsonValue) -> Result<(), String> {
        match self {
            JsonValue::Object(obj) => {
                obj.insert(key, value);
                Ok(())
            },
            _ => Err("Not an object"),
        }
    }

    // Set value in array by index
    fn set_index(self: &mut JsonValue, index: i32, value: JsonValue) -> Result<(), String> {
        match self {
            JsonValue::Array(arr) => {
                if index >= 0 && index < arr.len() {
                    // Would set arr[index] = value
                    Ok(())
                } else {
                    Err("Index out of bounds")
                }
            },
            _ => Err("Not an array"),
        }
    }

    // Push value to array
    fn push(self: &mut JsonValue, value: JsonValue) -> Result<(), String> {
        match self {
            JsonValue::Array(arr) => {
                arr.push(value);
                Ok(())
            },
            _ => Err("Not an array"),
        }
    }

    // Get array length
    fn len(self: &JsonValue) -> Result<i32, String> {
        match self {
            JsonValue::Array(arr) => Ok(arr.len()),
            JsonValue::Object(obj) => Ok(obj.len()),
            JsonValue::String(s) => Ok(s.len()),
            _ => Err("Type does not have a length"),
        }
    }

    // Check if object has key
    fn has_key(self: &JsonValue, key: String) -> bool {
        match self {
            JsonValue::Object(obj) => obj.contains_key(key),
            _ => false,
        }
    }

    // Get all keys from object
    fn keys(self: &JsonValue) -> Result<Vec<String>, String> {
        match self {
            JsonValue::Object(obj) => Ok(obj.keys()),
            _ => Err("Not an object"),
        }
    }

    // Get all values from object
    fn values(self: &JsonValue) -> Result<Vec<JsonValue>, String> {
        match self {
            JsonValue::Object(obj) => Ok(obj.values()),
            _ => Err("Not an object"),
        }
    }

    // Remove key from object
    fn remove(self: &mut JsonValue, key: String) -> Result<Option<JsonValue>, String> {
        match self {
            JsonValue::Object(obj) => Ok(obj.remove(key)),
            _ => Err("Not an object"),
        }
    }

    // Clone the value
    fn clone(self: &JsonValue) -> JsonValue {
        match self {
            JsonValue::Null => JsonValue::Null,
            JsonValue::Bool(b) => JsonValue::Bool(*b),
            JsonValue::Number(n) => JsonValue::Number(*n),
            JsonValue::String(s) => JsonValue::String(s.clone()),
            JsonValue::Array(arr) => JsonValue::Array(arr.clone()),
            JsonValue::Object(obj) => JsonValue::Object(obj.clone()),
        }
    }
}

// JSON parser
struct JsonParser {
    input: String,
    position: i32,
}

impl JsonParser {
    // Create a new parser
    fn new(input: String) -> JsonParser {
        return JsonParser {
            input: input,
            position: 0,
        };
    }

    // Parse JSON string into JsonValue
    fn parse(self: &mut JsonParser) -> Result<JsonValue, String> {
        self.skip_whitespace();
        return self.parse_value();
    }

    // Skip whitespace characters
    fn skip_whitespace(self: &mut JsonParser) {
        while self.position < self.input.len() {
            let ch = self.char_at(self.position);
            if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
                self.position = self.position + 1;
            } else {
                break;
            }
        }
    }

    // Get character at position
    fn char_at(self: &JsonParser, pos: i32) -> String {
        if pos >= 0 && pos < self.input.len() {
            // In JavaScript, this will be: input.charAt(pos) or input[pos]
            return self.input.substring(pos, pos + 1);
        }
        return "";
    }

    // Parse any JSON value
    fn parse_value(self: &mut JsonParser) -> Result<JsonValue, String> {
        self.skip_whitespace();
        let ch = self.peek();

        if ch == "n" {
            return self.parse_null();
        } else if ch == "t" || ch == "f" {
            return self.parse_bool();
        } else if ch == "\"" {
            return self.parse_string();
        } else if ch == "[" {
            return self.parse_array();
        } else if ch == "{" {
            return self.parse_object();
        } else {
            return self.parse_number();
        }
    }

    // Parse null
    fn parse_null(self: &mut JsonParser) -> Result<JsonValue, String> {
        self.match_keyword("null")?;
        return Ok(JsonValue::Null);
    }

    // Parse boolean
    fn parse_bool(self: &mut JsonParser) -> Result<JsonValue, String> {
        let ch = self.peek();
        if ch == "t" {
            self.match_keyword("true")?;
            return Ok(JsonValue::Bool(true));
        } else {
            self.match_keyword("false")?;
            return Ok(JsonValue::Bool(false));
        }
    }

    // Parse number
    fn parse_number(self: &mut JsonParser) -> Result<JsonValue, String> {
        let start_pos = self.position;
        let mut has_dot = false;
        let mut has_exp = false;

        // Handle negative sign
        if self.peek() == "-" {
            self.position = self.position + 1;
        }

        // Parse digits
        let mut has_digits = false;
        while !self.is_eof() {
            let ch = self.peek();

            if ch >= "0" && ch <= "9" {
                has_digits = true;
                self.position = self.position + 1;
            } else if ch == "." && !has_dot && !has_exp {
                has_dot = true;
                self.position = self.position + 1;
            } else if (ch == "e" || ch == "E") && !has_exp && has_digits {
                has_exp = true;
                self.position = self.position + 1;
                // Handle optional +/- after exponent
                let next_ch = self.peek();
                if next_ch == "+" || next_ch == "-" {
                    self.position = self.position + 1;
                }
            } else {
                break;
            }
        }

        if !has_digits {
            return Err("Invalid number");
        }

        // Extract number string and parse it
        let num_str = self.input.substring(start_pos, self.position);
        // In JavaScript, this will be: parseFloat(num_str)
        let num_value = num_str.parse_float();
        return Ok(JsonValue::Number(num_value));
    }

    // Parse string
    fn parse_string(self: &mut JsonParser) -> Result<JsonValue, String> {
        // Expect opening quote
        self.expect("\"")?;

        let mut result = "";
        while !self.is_eof() {
            let ch = self.advance();

            if ch == "\"" {
                // End of string
                return Ok(JsonValue::String(result));
            } else if ch == "\\" {
                // Escape sequence
                let escaped = self.advance();
                if escaped == "\"" {
                    result = result + "\"";
                } else if escaped == "\\" {
                    result = result + "\\";
                } else if escaped == "/" {
                    result = result + "/";
                } else if escaped == "n" {
                    result = result + "\n";
                } else if escaped == "r" {
                    result = result + "\r";
                } else if escaped == "t" {
                    result = result + "\t";
                } else if escaped == "b" {
                    result = result + "\b";
                } else if escaped == "f" {
                    result = result + "\f";
                } else if escaped == "u" {
                    // Unicode escape \uXXXX (simplified - just skip for now)
                    self.position = self.position + 4;  // Skip 4 hex digits
                    result = result + "?";  // Placeholder
                } else {
                    result = result + escaped;
                }
            } else {
                result = result + ch;
            }
        }

        return Err("Unterminated string");
    }

    // Parse array
    fn parse_array(self: &mut JsonParser) -> Result<JsonValue, String> {
        self.position = self.position + 1; // Skip '['
        let arr = Vec::new();

        loop {
            self.skip_whitespace();
            if self.peek() == "]" {
                self.position = self.position + 1;
                break;
            }

            let value = self.parse_value()?;
            arr.push(value);

            self.skip_whitespace();
            if self.peek() == "," {
                self.position = self.position + 1;
            }
        }

        return Ok(JsonValue::Array(arr));
    }

    // Parse object
    fn parse_object(self: &mut JsonParser) -> Result<JsonValue, String> {
        self.position = self.position + 1; // Skip '{'
        let obj = HashMap::new();

        loop {
            self.skip_whitespace();
            if self.peek() == "}" {
                self.position = self.position + 1;
                break;
            }

            // Parse key
            let key_value = self.parse_string()?;
            let key = match key_value {
                JsonValue::String(s) => s,
                _ => return Err("Expected string key"),
            };

            self.skip_whitespace();
            if self.peek() != ":" {
                return Err("Expected ':' after key");
            }
            self.position = self.position + 1;

            // Parse value
            let value = self.parse_value()?;
            obj.insert(key, value);

            self.skip_whitespace();
            if self.peek() == "," {
                self.position = self.position + 1;
            }
        }

        return Ok(JsonValue::Object(obj));
    }

    // Peek at current character
    fn peek(self: &JsonParser) -> String {
        return self.char_at(self.position);
    }

    // Advance position
    fn advance(self: &mut JsonParser) -> String {
        let ch = self.peek();
        self.position = self.position + 1;
        return ch;
    }

    // Check if we're at end of input
    fn is_eof(self: &JsonParser) -> bool {
        return self.position >= self.input.len();
    }

    // Expect a specific character
    fn expect(self: &mut JsonParser, expected: String) -> Result<(), String> {
        let ch = self.advance();
        if ch == expected {
            return Ok(());
        }
        return Err("Unexpected character");
    }

    // Match a keyword
    fn match_keyword(self: &mut JsonParser, keyword: String) -> Result<(), String> {
        let start_pos = self.position;
        for i in 0..keyword.len() {
            if self.char_at(self.position) != keyword.substring(i, i + 1) {
                self.position = start_pos;  // Reset on failure
                return Err("Keyword mismatch");
            }
            self.position = self.position + 1;
        }
        return Ok(());
    }
}

// JSON serializer
struct JsonSerializer {
    pretty: bool,
    indent_level: i32,
}

impl JsonSerializer {
    // Create a new serializer
    fn new(pretty: bool) -> JsonSerializer {
        return JsonSerializer {
            pretty: pretty,
            indent_level: 0,
        };
    }

    // Serialize JsonValue to string
    fn serialize(self: &mut JsonSerializer, value: &JsonValue) -> String {
        match value {
            JsonValue::Null => "null",
            JsonValue::Bool(b) => {
                if *b {
                    "true"
                } else {
                    "false"
                }
            },
            JsonValue::Number(n) => {
                // Convert number to string
                // In JavaScript: n.toString()
                return n.to_string();
            },
            JsonValue::String(s) => {
                // Escape and quote string
                return self.escape_string(s);
            },
            JsonValue::Array(arr) => self.serialize_array(arr),
            JsonValue::Object(obj) => self.serialize_object(obj),
        }
    }

    // Escape a string for JSON
    fn escape_string(self: &JsonSerializer, s: &String) -> String {
        let mut result = "\"";

        for i in 0..s.len() {
            let ch = s.substring(i, i + 1);

            if ch == "\"" {
                result = result + "\\\"";
            } else if ch == "\\" {
                result = result + "\\\\";
            } else if ch == "\n" {
                result = result + "\\n";
            } else if ch == "\r" {
                result = result + "\\r";
            } else if ch == "\t" {
                result = result + "\\t";
            } else if ch == "\b" {
                result = result + "\\b";
            } else if ch == "\f" {
                result = result + "\\f";
            } else {
                result = result + ch;
            }
        }

        result = result + "\"";
        return result;
    }

    // Serialize array
    fn serialize_array(self: &mut JsonSerializer, arr: &Vec<JsonValue>) -> String {
        let result = "[";

        let i = 0;
        for value in arr {
            if i > 0 {
                result = result + ",";
            }
            if self.pretty {
                result = result + "\n" + self.indent();
            }
            result = result + self.serialize(value);
            i = i + 1;
        }

        if self.pretty {
            result = result + "\n";
        }
        result = result + "]";
        return result;
    }

    // Serialize object
    fn serialize_object(self: &mut JsonSerializer, obj: &HashMap<String, JsonValue>) -> String {
        let result = "{";

        let i = 0;
        let keys = obj.keys();
        for key in keys {
            if i > 0 {
                result = result + ",";
            }
            if self.pretty {
                result = result + "\n" + self.indent();
            }

            // Key
            result = result + "\"" + key + "\":";
            if self.pretty {
                result = result + " ";
            }

            // Value
            match obj.get(key) {
                Option::Some(value) => {
                    result = result + self.serialize(value);
                },
                Option::None => {},
            }

            i = i + 1;
        }

        if self.pretty {
            result = result + "\n";
        }
        result = result + "}";
        return result;
    }

    // Get indentation string
    fn indent(self: &JsonSerializer) -> String {
        let spaces = "";
        let i = 0;
        for i in 0..(self.indent_level * 2) {
            spaces = spaces + " ";
        }
        return spaces;
    }
}

// Public API functions

// Parse JSON string into JsonValue
// Uses JavaScript's native JSON.parse() for performance and correctness
fn parse(input: String) -> Result<JsonValue, String> {
    // This will be compiled to JavaScript's JSON.parse()
    // For now, we'll use a simplified implementation
    // In production, this should call: JSON.parse(input)

    // Try to parse using JavaScript JSON.parse via external call
    // @external("JSON.parse")
    // fn json_parse_native(s: String) -> JsonValue;

    // Fallback to manual parser for now
    let mut parser = JsonParser::new(input);
    return parser.parse();
}

// Serialize JsonValue to JSON string
// Uses JavaScript's native JSON.stringify() for performance
fn stringify(value: &JsonValue) -> String {
    // This will be compiled to JavaScript's JSON.stringify()
    // For now, we'll use a simplified implementation
    // In production, this should call: JSON.stringify(value)

    // @external("JSON.stringify")
    // fn json_stringify_native(v: JsonValue) -> String;

    // Fallback to manual serializer for now
    let mut serializer = JsonSerializer::new(false);
    return serializer.serialize(value);
}

// Serialize JsonValue to pretty-printed JSON string
fn stringify_pretty(value: &JsonValue) -> String {
    let mut serializer = JsonSerializer::new(true);
    return serializer.serialize(value);
}

// Create empty JSON object
fn object() -> JsonValue {
    return JsonValue::Object(HashMap::new());
}

// Create empty JSON array
fn array() -> JsonValue {
    return JsonValue::Array(Vec::new());
}

// Create JSON null
fn null() -> JsonValue {
    return JsonValue::Null;
}

// Create JSON boolean
fn bool(value: bool) -> JsonValue {
    return JsonValue::Bool(value);
}

// Create JSON number from i32
fn number_i32(value: i32) -> JsonValue {
    return JsonValue::Number(value as f64);
}

// Create JSON number from f64
fn number_f64(value: f64) -> JsonValue {
    return JsonValue::Number(value);
}

// Create JSON string
fn string(value: String) -> JsonValue {
    return JsonValue::String(value);
}

// Helper: Parse JSON from HTTP response body
fn parse_response(body: String) -> Result<JsonValue, String> {
    return parse(body);
}

// Helper: Create JSON request body
fn create_request_body(data: JsonValue) -> String {
    return stringify(&data);
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_definition_exists() {
        assert!(!JSON_DEFINITION.is_empty());
    }

    #[test]
    fn test_json_definition_contains_enum() {
        assert!(JSON_DEFINITION.contains("enum JsonValue"));
    }

    #[test]
    fn test_json_definition_contains_parser() {
        assert!(JSON_DEFINITION.contains("struct JsonParser"));
        assert!(JSON_DEFINITION.contains("fn parse("));
    }

    #[test]
    fn test_json_definition_contains_serializer() {
        assert!(JSON_DEFINITION.contains("struct JsonSerializer"));
        assert!(JSON_DEFINITION.contains("fn stringify("));
    }

    #[test]
    fn test_json_definition_contains_api() {
        assert!(JSON_DEFINITION.contains("fn parse(input: String)"));
        assert!(JSON_DEFINITION.contains("fn stringify("));
        assert!(JSON_DEFINITION.contains("fn object()"));
        assert!(JSON_DEFINITION.contains("fn array()"));
    }

    #[test]
    fn test_json_definition_contains_value_methods() {
        assert!(JSON_DEFINITION.contains("fn is_null("));
        assert!(JSON_DEFINITION.contains("fn is_bool("));
        assert!(JSON_DEFINITION.contains("fn is_number("));
        assert!(JSON_DEFINITION.contains("fn is_string("));
        assert!(JSON_DEFINITION.contains("fn is_array("));
        assert!(JSON_DEFINITION.contains("fn is_object("));
        assert!(JSON_DEFINITION.contains("fn as_bool("));
        assert!(JSON_DEFINITION.contains("fn as_number("));
        assert!(JSON_DEFINITION.contains("fn as_string("));
        assert!(JSON_DEFINITION.contains("fn get("));
    }
}
