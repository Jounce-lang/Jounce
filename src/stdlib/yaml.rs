/// Standard library YAML parser and serializer
/// Provides YAML parsing and serialization capabilities

pub const YAML_DEFINITION: &str = r##"
// YAML parsing and serialization module
// Supports scalars, sequences, mappings, and basic multi-line strings

// YAML value types
enum YamlValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Sequence(Vec<YamlValue>),
    Mapping(HashMap<String, YamlValue>),
}

impl YamlValue {
    // Type checking methods
    fn is_null(self: &YamlValue) -> bool {
        match self {
            YamlValue::Null => true,
            _ => false,
        }
    }

    fn is_bool(self: &YamlValue) -> bool {
        match self {
            YamlValue::Bool(_) => true,
            _ => false,
        }
    }

    fn is_number(self: &YamlValue) -> bool {
        match self {
            YamlValue::Number(_) => true,
            _ => false,
        }
    }

    fn is_string(self: &YamlValue) -> bool {
        match self {
            YamlValue::String(_) => true,
            _ => false,
        }
    }

    fn is_sequence(self: &YamlValue) -> bool {
        match self {
            YamlValue::Sequence(_) => true,
            _ => false,
        }
    }

    fn is_mapping(self: &YamlValue) -> bool {
        match self {
            YamlValue::Mapping(_) => true,
            _ => false,
        }
    }

    // Conversion methods
    fn as_bool(self: &YamlValue) -> Option<bool> {
        match self {
            YamlValue::Bool(b) => Option::Some(*b),
            _ => Option::None,
        }
    }

    fn as_number(self: &YamlValue) -> Option<f64> {
        match self {
            YamlValue::Number(n) => Option::Some(*n),
            _ => Option::None,
        }
    }

    fn as_string(self: &YamlValue) -> Option<String> {
        match self {
            YamlValue::String(s) => Option::Some(s.clone()),
            _ => Option::None,
        }
    }

    fn as_sequence(self: &YamlValue) -> Option<Vec<YamlValue> > {
        match self {
            YamlValue::Sequence(seq) => Option::Some(seq.clone()),
            _ => Option::None,
        }
    }

    fn as_mapping(self: &YamlValue) -> Option<HashMap<String, YamlValue> > {
        match self {
            YamlValue::Mapping(map) => Option::Some(map.clone()),
            _ => Option::None,
        }
    }

    // Access methods for mappings
    fn get(self: &YamlValue, key: String) -> Option<YamlValue> {
        match self {
            YamlValue::Mapping(map) => {
                if map.contains_key(&key) {
                    Option::Some(map.get(&key).unwrap().clone())
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        }
    }

    fn set(self: &mut YamlValue, key: String, value: YamlValue) -> bool {
        match self {
            YamlValue::Mapping(map) => {
                map.insert(key, value);
                return true;
            }
            _ => false,
        }
    }

    fn keys(self: &YamlValue) -> Vec<String> {
        match self {
            YamlValue::Mapping(map) => {
                let mut result = Vec::new();
                for key in map.keys() {
                    result.push(key.clone());
                }
                return result;
            }
            _ => Vec::new(),
        }
    }

    // Access methods for sequences
    fn get_index(self: &YamlValue, index: i64) -> Option<YamlValue> {
        match self {
            YamlValue::Sequence(seq) => {
                if index >= 0 && (index as usize) < seq.len() {
                    Option::Some(seq[index as usize].clone())
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        }
    }

    fn push(self: &mut YamlValue, value: YamlValue) -> bool {
        match self {
            YamlValue::Sequence(seq) => {
                seq.push(value);
                return true;
            }
            _ => false,
        }
    }

    fn len(self: &YamlValue) -> i64 {
        match self {
            YamlValue::Sequence(seq) => seq.len() as i64,
            YamlValue::Mapping(map) => map.len() as i64,
            YamlValue::String(s) => s.len() as i64,
            _ => 0,
        }
    }

    // String representation
    fn to_string(self: &YamlValue) -> String {
        match self {
            YamlValue::Null => String::from("null"),
            YamlValue::Bool(b) => if *b { String::from("true") } else { String::from("false") },
            YamlValue::Number(n) => n.to_string(),
            YamlValue::String(s) => s.clone(),
            YamlValue::Sequence(_) => String::from("[Sequence]"),
            YamlValue::Mapping(_) => String::from("[Mapping]"),
        }
    }
}

// YAML Parser
struct YamlParser {
    input: String,
    position: i64,
    line: i64,
    column: i64,
    indent_level: i64,
}

impl YamlParser {
    fn new(input: String) -> YamlParser {
        return YamlParser {
            input: input,
            position: 0,
            line: 1,
            column: 0,
            indent_level: 0,
        };
    }

    fn current_char(self: &YamlParser) -> String {
        if self.position >= self.input.len() as i64 {
            return String::new();
        }
        return self.input.char_at(self.position as usize);
    }

    fn peek_char(self: &YamlParser, offset: i64) -> String {
        let pos = self.position + offset;
        if pos >= self.input.len() as i64 {
            return String::new();
        }
        return self.input.char_at(pos as usize);
    }

    fn advance(self: &mut YamlParser) {
        if self.position < self.input.len() as i64 {
            let ch = self.current_char();
            if ch == "\n" {
                self.line = self.line + 1;
                self.column = 0;
            } else {
                self.column = self.column + 1;
            }
            self.position = self.position + 1;
        }
    }

    fn skip_whitespace(self: &mut YamlParser) {
        loop {
            let ch = self.current_char();
            if ch == " " || ch == "\t" {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line(self: &mut YamlParser) {
        loop {
            let ch = self.current_char();
            if ch.is_empty() || ch == "\n" {
                break;
            }
            self.advance();
        }
        if self.current_char() == "\n" {
            self.advance();
        }
    }

    fn parse_value(self: &mut YamlParser) -> Result<YamlValue, String> {
        self.skip_whitespace();

        let ch = self.current_char();

        // Handle comments
        if ch == "#" {
            self.skip_line();
            return self.parse_value();
        }

        // Handle sequences (flow style: [1, 2, 3])
        if ch == "[" {
            return self.parse_flow_sequence();
        }

        // Handle mappings (flow style: {key: value})
        if ch == "{" {
            return self.parse_flow_mapping();
        }

        // Handle sequences (block style with -)
        if ch == "-" && (self.peek_char(1) == " " || self.peek_char(1) == "\n") {
            return self.parse_block_sequence();
        }

        // Handle scalars
        return self.parse_scalar();
    }

    fn parse_scalar(self: &mut YamlParser) -> Result<YamlValue, String> {
        let start = self.position;
        let mut value = String::new();

        // Read until newline, comma, or end of input
        loop {
            let ch = self.current_char();
            if ch.is_empty() || ch == "\n" || ch == "," || ch == "]" || ch == "}" {
                break;
            }
            if ch == "#" {
                break;
            }
            value.push_str(&ch);
            self.advance();
        }

        // Trim whitespace
        value = value.trim().to_string();

        // Parse as appropriate type
        if value == "null" || value == "~" || value.is_empty() {
            return Result::Ok(YamlValue::Null);
        }

        if value == "true" {
            return Result::Ok(YamlValue::Bool(true));
        }

        if value == "false" {
            return Result::Ok(YamlValue::Bool(false));
        }

        // Try to parse as number
        let num_opt = value.parse_float();
        match num_opt {
            Option::Some(num) => {
                return Result::Ok(YamlValue::Number(num));
            }
            Option::None => {}
        }

        // Remove quotes if present
        if value.starts_with("\"") && value.ends_with("\"") {
            value = value.substring(1, value.len() - 1);
        } else if value.starts_with("'") && value.ends_with("'") {
            value = value.substring(1, value.len() - 1);
        }

        return Result::Ok(YamlValue::String(value));
    }

    fn parse_flow_sequence(self: &mut YamlParser) -> Result<YamlValue, String> {
        // Skip opening [
        self.advance();

        let mut sequence = Vec::new();

        loop {
            self.skip_whitespace();

            let ch = self.current_char();
            if ch == "]" {
                self.advance();
                break;
            }

            if ch.is_empty() {
                return Result::Err(String::from("Unexpected end of input in sequence"));
            }

            // Parse value
            let value = self.parse_value()?;
            sequence.push(value);

            self.skip_whitespace();

            // Check for comma or closing bracket
            let next = self.current_char();
            if next == "," {
                self.advance();
            } else if next == "]" {
                self.advance();
                break;
            } else if !next.is_empty() {
                return Result::Err(String::from("Expected ',' or ']' in sequence"));
            }
        }

        return Result::Ok(YamlValue::Sequence(sequence));
    }

    fn parse_flow_mapping(self: &mut YamlParser) -> Result<YamlValue, String> {
        // Skip opening {
        self.advance();

        let mut mapping = HashMap::new();

        loop {
            self.skip_whitespace();

            let ch = self.current_char();
            if ch == "}" {
                self.advance();
                break;
            }

            if ch.is_empty() {
                return Result::Err(String::from("Unexpected end of input in mapping"));
            }

            // Parse key
            let key_result = self.parse_scalar()?;
            let key = match key_result {
                YamlValue::String(s) => s,
                _ => key_result.to_string(),
            };

            self.skip_whitespace();

            // Expect colon
            if self.current_char() != ":" {
                return Result::Err(String::from("Expected ':' after key"));
            }
            self.advance();

            self.skip_whitespace();

            // Parse value
            let value = self.parse_value()?;
            mapping.insert(key, value);

            self.skip_whitespace();

            // Check for comma or closing brace
            let next = self.current_char();
            if next == "," {
                self.advance();
            } else if next == "}" {
                self.advance();
                break;
            } else if !next.is_empty() {
                return Result::Err(String::from("Expected ',' or '}' in mapping"));
            }
        }

        return Result::Ok(YamlValue::Mapping(mapping));
    }

    fn parse_block_sequence(self: &mut YamlParser) -> Result<YamlValue, String> {
        let mut sequence = Vec::new();
        let base_indent = self.column;

        loop {
            // Check if we're still at the same indent level with a dash
            if self.current_char() != "-" {
                break;
            }

            // Skip dash
            self.advance();
            self.skip_whitespace();

            // Parse value
            let value = self.parse_value()?;
            sequence.push(value);

            // Skip to next line
            if self.current_char() == "\n" {
                self.advance();
            }

            // Check indent of next line
            self.skip_whitespace();
            if self.column < base_indent {
                break;
            }
        }

        return Result::Ok(YamlValue::Sequence(sequence));
    }
}

// Parse YAML string to YamlValue
fn parse(yaml: String) -> Result<YamlValue, String> {
    let mut parser = YamlParser::new(yaml);
    return parser.parse_value();
}

// Serialize YamlValue to YAML string
fn stringify(value: YamlValue) -> String {
    return stringify_with_indent(&value, 0);
}

fn stringify_with_indent(value: &YamlValue, indent: i64) -> String {
    let indent_str = " ".repeat(indent as usize * 2);

    match value {
        YamlValue::Null => String::from("null"),
        YamlValue::Bool(b) => if *b { String::from("true") } else { String::from("false") },
        YamlValue::Number(n) => n.to_string(),
        YamlValue::String(s) => {
            // Quote strings that need it
            if s.contains(":") || s.contains("#") || s.contains("\"") {
                return format!("\"{}\"", s.replace("\"", "\\\""));
            }
            return s.clone();
        }
        YamlValue::Sequence(seq) => {
            if seq.is_empty() {
                return String::from("[]");
            }

            let mut result = String::new();
            for item in seq {
                result.push_str("\n");
                result.push_str(&indent_str);
                result.push_str("- ");
                result.push_str(&stringify_with_indent(item, indent + 1));
            }
            return result;
        }
        YamlValue::Mapping(map) => {
            if map.is_empty() {
                return String::from("{}");
            }

            let mut result = String::new();
            let keys = map.keys();
            for key in keys {
                result.push_str("\n");
                result.push_str(&indent_str);
                result.push_str(key);
                result.push_str(": ");
                let value_str = stringify_with_indent(&map.get(key).unwrap(), indent + 1);
                result.push_str(&value_str);
            }
            return result;
        }
    }
}

// Create YAML values

fn yaml_null() -> YamlValue {
    return YamlValue::Null;
}

fn yaml_bool(value: bool) -> YamlValue {
    return YamlValue::Bool(value);
}

fn yaml_number(value: f64) -> YamlValue {
    return YamlValue::Number(value);
}

fn yaml_string(value: String) -> YamlValue {
    return YamlValue::String(value);
}

fn yaml_sequence() -> YamlValue {
    return YamlValue::Sequence(Vec::new());
}

fn yaml_mapping() -> YamlValue {
    return YamlValue::Mapping(HashMap::new());
}
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_definition_exists() {
        assert!(!YAML_DEFINITION.is_empty());
    }

    #[test]
    fn test_yaml_definition_contains_enum() {
        assert!(YAML_DEFINITION.contains("enum YamlValue"));
        assert!(YAML_DEFINITION.contains("Null"));
        assert!(YAML_DEFINITION.contains("Bool"));
        assert!(YAML_DEFINITION.contains("Number"));
        assert!(YAML_DEFINITION.contains("String"));
        assert!(YAML_DEFINITION.contains("Sequence"));
        assert!(YAML_DEFINITION.contains("Mapping"));
    }

    #[test]
    fn test_yaml_definition_contains_parser() {
        assert!(YAML_DEFINITION.contains("struct YamlParser"));
        assert!(YAML_DEFINITION.contains("fn parse("));
        assert!(YAML_DEFINITION.contains("fn stringify("));
    }

    #[test]
    fn test_yaml_definition_contains_type_checks() {
        assert!(YAML_DEFINITION.contains("fn is_null("));
        assert!(YAML_DEFINITION.contains("fn is_bool("));
        assert!(YAML_DEFINITION.contains("fn is_number("));
        assert!(YAML_DEFINITION.contains("fn is_string("));
        assert!(YAML_DEFINITION.contains("fn is_sequence("));
        assert!(YAML_DEFINITION.contains("fn is_mapping("));
    }

    #[test]
    fn test_yaml_definition_contains_conversions() {
        assert!(YAML_DEFINITION.contains("fn as_bool("));
        assert!(YAML_DEFINITION.contains("fn as_number("));
        assert!(YAML_DEFINITION.contains("fn as_string("));
        assert!(YAML_DEFINITION.contains("fn as_sequence("));
        assert!(YAML_DEFINITION.contains("fn as_mapping("));
    }

    #[test]
    fn test_yaml_definition_contains_accessors() {
        assert!(YAML_DEFINITION.contains("fn get("));
        assert!(YAML_DEFINITION.contains("fn set("));
        assert!(YAML_DEFINITION.contains("fn keys("));
        assert!(YAML_DEFINITION.contains("fn get_index("));
        assert!(YAML_DEFINITION.contains("fn push("));
        assert!(YAML_DEFINITION.contains("fn len("));
    }

    #[test]
    fn test_yaml_definition_contains_constructors() {
        assert!(YAML_DEFINITION.contains("fn yaml_null("));
        assert!(YAML_DEFINITION.contains("fn yaml_bool("));
        assert!(YAML_DEFINITION.contains("fn yaml_number("));
        assert!(YAML_DEFINITION.contains("fn yaml_string("));
        assert!(YAML_DEFINITION.contains("fn yaml_sequence("));
        assert!(YAML_DEFINITION.contains("fn yaml_mapping("));
    }
}
