// Enhanced error messaging system with helpful tips and solutions

use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JError {
    pub kind: ErrorKind,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub source_line: Option<String>,
    pub tip: Option<String>,
    pub solution: Option<String>,
    pub context: Option<String>,
    pub similar_names: Vec<String>,
    pub help_url: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // Lexer errors
    UnexpectedCharacter,
    UnterminatedString,
    InvalidNumber,
    InvalidEscape,

    // Parser errors
    UnexpectedToken,
    ExpectedToken,
    InvalidSyntax,
    MissingOperand,

    // Runtime errors
    UndefinedVariable,
    TypeError,
    DivisionByZero,
    IndexOutOfBounds,
    KeyNotFound,
    InvalidOperation,
    StackOverflow,

    // Function errors
    UndefinedFunction,
    WrongArgumentCount,
    InvalidArgument,

    // Other
    FileNotFound,
    IOError,
}

impl JError {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            line: None,
            column: None,
            source_line: None,
            tip: None,
            solution: None,
            context: None,
            similar_names: Vec::new(),
            help_url: None,
        }
    }

    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    #[allow(dead_code)]
    pub fn with_source(mut self, source_line: String) -> Self {
        self.source_line = Some(source_line);
        self
    }

    pub fn with_tip(mut self, tip: String) -> Self {
        self.tip = Some(tip);
        self
    }

    pub fn with_solution(mut self, solution: String) -> Self {
        self.solution = Some(solution);
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_similar_names(mut self, names: Vec<String>) -> Self {
        self.similar_names = names;
        self
    }

    pub fn with_help_url(mut self, url: String) -> Self {
        self.help_url = Some(url);
        self
    }

    // Smart error constructors with automatic tips and solutions

    pub fn undefined_variable(name: &str, line: usize, column: usize) -> Self {
        let similar = find_similar_names(name, &get_common_variable_names());
        let mut error = Self::new(
            ErrorKind::UndefinedVariable,
            format!("Variable '{}' is not defined", name),
        )
        .with_location(line, column);

        if !similar.is_empty() {
            error = error.with_similar_names(similar.clone());
            error = error.with_tip(format!("Did you mean one of these? {}", similar.join(", ")));
        } else {
            error = error
                .with_tip("Make sure you've declared this variable before using it".to_string());
        }

        error = error.with_solution(format!(
            "Declare the variable first:\n  int | {} -> 0\n  str | {} -> \"value\"\n  list | {} -> []\n\nOr check for typos in the variable name",
            name, name, name
        ));

        error = error.with_context("Variable access".to_string());
        error = error.with_help_url("https://j-lang.dev/docs/variables".to_string());

        error
    }

    pub fn undefined_function(name: &str, line: usize, column: usize) -> Self {
        let similar = find_similar_names(name, &get_builtin_function_names());
        let mut error = Self::new(
            ErrorKind::UndefinedFunction,
            format!("Function '{}' is not defined", name),
        )
        .with_location(line, column);

        if !similar.is_empty() {
            error = error.with_similar_names(similar.clone());
            error = error.with_tip(format!("Did you mean one of these? {}", similar.join(", ")));
        } else {
            error = error.with_tip(
                "Check if the function name is spelled correctly or if it's a built-in function"
                    .to_string(),
            );
        }

        error = error.with_solution(format!(
            "Define the function first:\n  fn | {}(params) > {{\n    // function body\n    return result\n  }}\n\nOr check the built-in functions list",
            name
        ));

        error = error.with_context("Function call".to_string());
        error = error.with_help_url("https://j-lang.dev/docs/functions".to_string());

        error
    }

    #[allow(dead_code)]
    pub fn type_error(expected: &str, got: &str, line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::TypeError,
            format!("Type mismatch: expected {}, got {}", expected, got),
        )
        .with_location(line, column);

        error = error.with_tip(format!("The operation requires a {} value", expected));

        // Provide conversion suggestions
        let conversion_tip = match (expected, got) {
            ("int", "float") => Some("Use int(value) to convert float to int"),
            ("float", "int") => Some("Use float(value) to convert int to float"),
            ("str", _) => Some("Use str(value) to convert to string"),
            ("list", "str") => Some("Use list(value) to convert string to list of characters"),
            _ => None,
        };

        if let Some(tip) = conversion_tip {
            error = error.with_solution(tip.to_string());
        }

        error
    }

    #[allow(dead_code)]
    pub fn wrong_argument_count(
        func_name: &str,
        expected: usize,
        got: usize,
        line: usize,
        column: usize,
    ) -> Self {
        let mut error = Self::new(
            ErrorKind::WrongArgumentCount,
            format!(
                "Function '{}' expects {} argument(s), but {} were provided",
                func_name, expected, got
            ),
        )
        .with_location(line, column);

        if got < expected {
            error = error.with_tip(format!(
                "Add {} more argument(s) to the function call",
                expected - got
            ));
        } else {
            error = error.with_tip(format!(
                "Remove {} argument(s) from the function call",
                got - expected
            ));
        }

        error = error.with_solution(format!(
            "Check the function definition to see what arguments it expects"
        ));

        error
    }

    pub fn division_by_zero(line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::DivisionByZero,
            "Cannot divide by zero".to_string(),
        )
        .with_location(line, column);

        error = error.with_tip("Check if the divisor could be zero".to_string());
        error = error.with_solution(
            "Add a check before division:\n  if divisor != 0 {\n    result = numerator / divisor\n  }".to_string()
        );

        error
    }

    pub fn index_out_of_bounds(index: i64, length: usize, line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::IndexOutOfBounds,
            format!(
                "Index {} is out of bounds for list of length {}",
                index, length
            ),
        )
        .with_location(line, column);

        error = error.with_tip(format!(
            "Valid indices are 0 to {}",
            length.saturating_sub(1)
        ));
        error = error.with_solution(
            "Check the index before accessing:\n  if index < len(list) {\n    value = list[index]\n  }".to_string()
        );

        error
    }

    pub fn key_not_found(key: &str, line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::KeyNotFound,
            format!("Key '{}' not found in dictionary", key),
        )
        .with_location(line, column);

        error = error.with_tip("Check if the key exists before accessing it".to_string());
        error = error.with_solution(format!(
            "Use the 'in' operator to check:\n  if \"{}\" in dict {{\n    value = dict[\"{}\"]\n  }}",
            key, key
        ));

        error
    }

    pub fn unexpected_token(expected: &str, got: &str, line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::UnexpectedToken,
            format!("Expected {}, but got {}", expected, got),
        )
        .with_location(line, column);

        error = error.with_tip(format!("Add {} here", expected));

        // Provide syntax-specific solutions
        let solution = match expected {
            "'->' after variable name" => "Variable declarations use: type | name -> value",
            "'|' after type" => "Variable declarations use: type | name -> value",
            "')' after parameters" => "Function declarations use: fn | name(params) > { body }",
            "'>' before function body" => "Function declarations use: fn | name(params) > { body }",
            _ => "Check the J-Lang syntax guide for correct syntax",
        };

        error = error.with_solution(solution.to_string());

        error
    }

    pub fn unterminated_string(line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::UnterminatedString,
            "String literal is not terminated".to_string(),
        )
        .with_location(line, column);

        error = error.with_tip("Add a closing quote (\") to end the string".to_string());
        error =
            error.with_solution("Strings must be enclosed in double quotes: \"text\"".to_string());

        error
    }

    #[allow(dead_code)]
    pub fn invalid_syntax(context: &str, line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::InvalidSyntax,
            format!("Invalid syntax in {}", context),
        )
        .with_location(line, column);

        error = error.with_tip("Review the syntax for this construct".to_string());

        let solution = match context {
            "variable declaration" => "Use: type | name -> value\nExample: int | x -> 42",
            "function declaration" => "Use: fn | name(type|param) > { body }\nExample: fn | add(int|a, int|b) > { return a + b }",
            "for loop" => "Use: for var in iterable { body }\nExample: for i in 0..10 { out(i) }",
            "if statement" => "Use: if condition { body }\nExample: if x > 0 { out(\"positive\") }",
            _ => "Check the J-Lang documentation for correct syntax",
        };

        error = error.with_solution(solution.to_string());

        error
    }

    pub fn stack_overflow(depth: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::StackOverflow,
            format!(
                "Stack overflow: too many recursive calls (depth: {})",
                depth
            ),
        );

        error = error.with_tip("Your function is calling itself too many times".to_string());
        error = error.with_solution(
            "Add a base case to stop recursion:\n  fn | factorial(int|n) > {\n    if n <= 1 { return 1 }\n    return n * factorial(n - 1)\n  }".to_string()
        );
        error = error.with_context("Recursive function call".to_string());
        error = error.with_help_url("https://j-lang.dev/docs/recursion".to_string());

        error
    }

    #[allow(dead_code)]
    pub fn invalid_operation(
        operation: &str,
        type1: &str,
        type2: &str,
        line: usize,
        column: usize,
    ) -> Self {
        let mut error = Self::new(
            ErrorKind::InvalidOperation,
            format!("Cannot perform '{}' on {} and {}", operation, type1, type2),
        )
        .with_location(line, column);

        error = error.with_tip(format!(
            "The '{}' operation is not supported for these types",
            operation
        ));

        let solution = match (operation, type1, type2) {
            ("+", "str", "int") | ("+", "int", "str") => {
                "Convert to string first: str(number) + text\nOr convert to int: number + int(text)"
            }
            ("+", "list", _) | ("+", _, "list") => {
                "Use append() or extend() to add items to a list"
            }
            ("-" | "*" | "/", "str", _) | ("-" | "*" | "/", _, "str") => {
                "Convert strings to numbers first: int(text) or float(text)"
            }
            _ => "Check that both operands are compatible types",
        };

        error = error.with_solution(solution.to_string());
        error = error.with_context(format!("Binary operation: {}", operation));

        error
    }

    pub fn parser_error(
        message: &str,
        expected: &str,
        _got: &str,
        line: usize,
        column: usize,
    ) -> Self {
        let mut error =
            Self::new(ErrorKind::UnexpectedToken, message.to_string()).with_location(line, column);

        // Provide context-specific tips
        let (tip, solution): (&str, &str) = match expected {
            "'->' after variable name" | "'|' after type" => (
                "Variable declarations follow the pattern: type | name -> value",
                "Examples:\n  int | count -> 0\n  str | name -> \"John\"\n  list | items -> [1, 2, 3]"
            ),
            "')' after parameters" | "'>' before function body" => (
                "Function declarations follow the pattern: fn | name(params) > { body }",
                "Examples:\n  fn | greet(str|name) > { out(\"Hello\", name) }\n  fn | add(int|a, int|b) > { return a + b }"
            ),
            "'{' to start block" => (
                "Blocks must be enclosed in curly braces { }",
                "Examples:\n  if condition { statements }\n  for i in range { statements }"
            ),
            _ => (
                "Check the expected token",
                "Check the J-Lang syntax guide for correct syntax"
            ),
        };

        error = error.with_tip(tip.to_string());
        error = error.with_solution(solution.to_string());
        error = error.with_context("Parsing".to_string());
        error = error.with_help_url("https://j-lang.dev/docs/syntax".to_string());

        error
    }

    #[allow(dead_code)]
    pub fn keyword_as_identifier(keyword: &str, line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::InvalidSyntax,
            format!(
                "'{}' is a reserved keyword and cannot be used as an identifier",
                keyword
            ),
        )
        .with_location(line, column);

        error = error.with_tip(format!("'{}' has special meaning in J-Lang", keyword));
        error = error.with_solution(format!(
            "Choose a different name:\n  {} -> my_{}\n  {} -> {}_value\n  {} -> user_{}",
            keyword, keyword, keyword, keyword, keyword, keyword
        ));

        let keywords = vec![
            "if",
            "else",
            "for",
            "while",
            "fn",
            "return",
            "break",
            "continue",
            "int",
            "str",
            "bool",
            "float",
            "list",
            "dict",
            "true",
            "false",
            "sweep",
            "shrink",
            "meet",
            "binary",
            "dp",
            "while_nonzero",
            "while_change",
        ];

        error = error.with_context(format!("Reserved keywords: {}", keywords.join(", ")));

        error
    }

    #[allow(dead_code)]
    pub fn immutable_assignment(name: &str, line: usize, column: usize) -> Self {
        let mut error = Self::new(
            ErrorKind::InvalidOperation,
            format!("Cannot assign to immutable variable '{}'", name),
        )
        .with_location(line, column);

        error = error.with_tip("This variable was declared as immutable (const)".to_string());
        error = error.with_solution(format!(
            "Either:\n  1. Declare as mutable: int | {} -> value\n  2. Create a new variable: int | new_{} -> new_value",
            name, name
        ));
        error = error.with_context("Variable assignment".to_string());

        error
    }

    #[allow(dead_code)]
    pub fn file_not_found(filename: &str) -> Self {
        let mut error = Self::new(
            ErrorKind::FileNotFound,
            format!("File '{}' not found", filename),
        );

        error =
            error.with_tip("Check that the file path is correct and the file exists".to_string());
        error = error.with_solution(format!(
            "Verify:\n  1. The file exists at: {}\n  2. The file extension is correct (.j)\n  3. You have read permissions",
            filename
        ));
        error = error.with_context("File operation".to_string());

        error
    }
}

impl fmt::Display for JError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Compact error format
        let (emoji, severity) = match self.kind {
            ErrorKind::UnexpectedCharacter | ErrorKind::UnexpectedToken => ("❌", "SYNTAX"),
            ErrorKind::UndefinedVariable | ErrorKind::UndefinedFunction => ("🔍", "NAME"),
            ErrorKind::TypeError => ("🔧", "TYPE"),
            ErrorKind::DivisionByZero | ErrorKind::IndexOutOfBounds => ("⚠️", "RUNTIME"),
            ErrorKind::StackOverflow => ("💥", "STACK"),
            ErrorKind::WrongArgumentCount | ErrorKind::InvalidArgument => ("📝", "ARGUMENT"),
            ErrorKind::FileNotFound | ErrorKind::IOError => ("📁", "FILE"),
            _ => ("❗", "ERROR"),
        };

        // Single line header
        write!(f, "\n{} {} ERROR: {}", emoji, severity, self.message)?;

        // Location on same line if available
        if let (Some(line), Some(column)) = (self.line, self.column) {
            write!(f, " (line {}, col {})", line, column)?;
        }
        writeln!(f)?;

        // Show source line with pointer (compact)
        if let (Some(line), Some(column)) = (self.line, self.column) {
            if let Some(ref source) = self.source_line {
                writeln!(f, "  {} │ {}", line, source)?;
                writeln!(
                    f,
                    "  {} │ {}^",
                    " ".repeat(line.to_string().len()),
                    " ".repeat(column.saturating_sub(1))
                )?;
            }
        }

        // Similar names (compact)
        if !self.similar_names.is_empty() {
            write!(f, "  💡 Did you mean: {}", self.similar_names.join(", "))?;
            writeln!(f)?;
        }

        // Tip (compact, single line if possible)
        if let Some(ref tip) = self.tip {
            if tip.lines().count() == 1 {
                writeln!(f, "  💡 {}", tip)?;
            } else {
                writeln!(f, "  💡 Tip:")?;
                for line in tip.lines() {
                    writeln!(f, "     {}", line)?;
                }
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

// Helper function to find similar names using Levenshtein distance
fn find_similar_names(name: &str, candidates: &[&str]) -> Vec<String> {
    let mut matches: Vec<(String, usize)> = candidates
        .iter()
        .map(|&candidate| {
            let distance = levenshtein_distance(name, candidate);
            (candidate.to_string(), distance)
        })
        .filter(|(_, distance)| *distance <= 3 && *distance > 0)
        .collect();

    matches.sort_by_key(|(_, distance)| *distance);
    matches.into_iter().take(3).map(|(name, _)| name).collect()
}

// Get common variable names for suggestions
fn get_common_variable_names() -> Vec<&'static str> {
    vec![
        "i", "j", "k", "x", "y", "z", "count", "index", "value", "result", "temp", "left", "right",
        "mid", "start", "end", "sum", "product", "max", "min", "total", "arr", "list", "nums",
        "items", "data", "key", "val", "name", "id", "size", "length",
    ]
}

// Get built-in function names for suggestions
fn get_builtin_function_names() -> Vec<&'static str> {
    vec![
        "out", "len", "sum", "min", "max", "abs", "pow", "range", "map", "filter", "reduce", "zip",
        "sort", "reverse", "unique", "flatten", "split", "join", "replace", "trim", "push", "pop",
        "append", "insert", "remove", "keys", "values", "items", "get", "set", "int", "float",
        "str", "bool", "list", "dict", "type", "print", "input", "read", "write",
    ]
}

// Helper function to suggest similar variable names (simple Levenshtein distance)
#[allow(dead_code)]
fn suggest_similar_name(name: &str) -> Option<String> {
    let candidates = get_common_variable_names();
    let similar = find_similar_names(name, &candidates);
    similar.first().cloned()
}

// Simple Levenshtein distance implementation
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

// Convert string errors to JError
impl From<String> for JError {
    fn from(s: String) -> Self {
        // Try to parse error messages and enhance them
        if s.contains("not defined") || s.contains("not found") {
            if let Some(name) = extract_identifier(&s) {
                if s.contains("Variable") {
                    return JError::undefined_variable(&name, 0, 0);
                } else if s.contains("Function") {
                    return JError::undefined_function(&name, 0, 0);
                }
            }
        }

        if s.contains("divide by zero") || s.contains("division by zero") {
            return JError::division_by_zero(0, 0);
        }

        if s.contains("Stack overflow") {
            return JError::stack_overflow(50);
        }

        // Default error
        JError::new(ErrorKind::InvalidOperation, s)
    }
}

fn extract_identifier(s: &str) -> Option<String> {
    // Extract identifier from error message like "Variable 'x' not defined"
    if let Some(start) = s.find('\'') {
        if let Some(end) = s[start + 1..].find('\'') {
            return Some(s[start + 1..start + 1 + end].to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_variable_error() {
        let error = JError::undefined_variable("count", 5, 10);
        assert_eq!(error.kind, ErrorKind::UndefinedVariable);
        assert!(error.tip.is_some());
        assert!(error.solution.is_some());
    }

    #[test]
    fn test_type_error() {
        let error = JError::type_error("int", "str", 3, 5);
        assert_eq!(error.kind, ErrorKind::TypeError);
        assert!(error.tip.is_some());
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("count", "cont"), 1);
        assert_eq!(levenshtein_distance("index", "indx"), 1);
        assert_eq!(levenshtein_distance("hello", "world"), 4);
    }
}
