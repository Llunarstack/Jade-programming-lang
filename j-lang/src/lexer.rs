use crate::error::JError;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Char(char),
    Emoji(String),
    Money(String, f64), // (currency_symbol, amount)
    Hex(String),
    Date(String),
    Time(String),
    DateTime(String),
    Infinity(bool), // true for +inf, false for -inf

    // Identifiers and Keywords
    Identifier(String),

    // Types
    Str,
    Int,
    FloatType,
    Bool,
    List,
    Dict,
    Tuple,
    Vec,
    Mat,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
    Set,
    Counter,
    Deque,
    PriorityQ,
    Graph,
    Tree,
    Grid,
    // New OP collection types
    Queue,
    Ring,
    Sorted,
    Bag,
    WindowType,
    ViewType,
    Prio,
    Diff,
    // Advanced array types
    Span,
    MutSpan,
    Chunk,
    Sparse,
    Stride,
    Flat,
    MutFlat,
    IntervalType,
    Union,
    CharType,
    EmojiType,
    Ascii,
    MoneyType,
    HexType,
    DateType,
    TimeType,
    DateTimeType,
    Any,
    Expr,
    EnumType,

    // Keywords
    Fn,
    Class,
    EnumKeyword,
    Trait,
    Static,
    Pub,
    Priv,
    If,
    Else,
    Match,
    Case,
    While,
    Loop,
    For,
    In,
    Break,
    Continue,
    Defer,
    Converge,
    // Advanced loop keywords
    Sweep,
    Shrink,
    Meet,
    Binary,
    Dp,
    WhileNonzero,
    WhileChange,
    WhileMatch,
    Return,
    Yield,
    Async,
    Await,
    Task,
    Gen,
    Use,
    Mod,
    Import,
    Module,
    Let,
    Mut,
    Ref,
    Try,
    Catch,
    Finally,
    Panic,
    Of,
    With,
    Cond,
    By,
    This,
    Self_,
    // jnew_features: extensions, loops, security, enterprise, tooling
    Extend,
    Phantom,
    Mirror,
    MemoVar,
    Fuzz,
    Within,
    Rollback,
    Retry,
    Race,
    Barrier,
    Pulse,
    Untrusted,
    Secret,
    Secure,
    Canary,
    Component,
    Contract,
    Workspace,
    Env,
    Packet,
    Gui,
    Sql,
    Embed,
    Triple,
    Shield,
    Deterministic,
    Audit,
    Layout,
    Fixed,
    Sequence,
    Pure,
    Effect,
    Invariant,
    Constraint,
    Solver,
    Flood,
    Ignite,
    Interval,
    ConstantTimeEq, // ~== operator
    Tilde,          // ~ (single)

    // Operators
    Arrow,    // ->
    FatArrow, // => (for arrow lambdas)
    Pipeline, // |>
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %
    Power,    // **

    // Comparison
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Logical
    And, // and
    Or,  // or
    Not, // not

    // Bitwise operators
    Ampersand,  // &
    BitwiseOr,  // | (already exists as Pipe, but we'll use for bitwise)
    Caret,      // ^
    LeftShift,  // <<
    RightShift, // >>

    // Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Dot,          // .
    DotDot,       // ..
    Colon,        // :
    Semicolon,    // ;
    Pipe,         // |
    Question,     // ?
    Exclamation,  // !
    At,           // @
    Hash,         // #
    Dollar,       // $

    // Special
    Newline,
    Eof,

    // Range operators
    Range,          // ..
    RangeExclusive, // ..<

    // Type conversion
    TypeConvert, // *type (e.g., str*count)

    // J-specific operators
    Bind,       // <- (for conversions)
    Underscore, // _ (anonymous variable)

    // Execute syntax
    Execute, // j; (execute command)

    // Generator keywords
    GenType,      // gen<T> type
    YieldKeyword, // yield
    Generic,      // generic type parameters
    TypeParam,    // <T> in generics

    // Additional operators
    Reverse, // rev (reverse iteration)
    Step,    // step (step iteration)
    Until,   // until (condition)
    Where,   // where (filter)
    Zip,     // zip (parallel iteration)

    // Advanced J features
    Auto,    // auto (auto-optimization)
    Cheat,   // cheat (aggressive optimization)
    Live,    // live (hot-reload variables)
    Why,     // why (error explanation)
    Blend,   // blend (probabilistic execution)
    Echo,    // echo (reactive variables)
    Trace,   // trace (deterministic replay)
    Guard,   // guard (contracts)
    Lens,    // lens (zero-copy views)
    Predict, // predict (profile-guided optimization)
    Forever, // forever (daemon mode)
    Quantum, // quantum (quantum computing hint)

    // Memory management
    Arena,   // arena (arena allocation)
    Stack,   // stack (stack allocation)
    Pool,    // pool (object pool)
    Tight,   // tight (packed structs)
    Recycle, // recycle (object recycling)

    // Concurrency
    Parallel, // parallel (parallel execution)
    Threaded, // threaded (explicit threading)
    Scope,    // scope (structured concurrency)
    Cancel,   // cancel (cancellation)

    // Testing
    Test,     // test (test case)
    Property, // property (property test)
    Assert,   // assert (assertion)

    // Macros
    Macro,       // macro (compile-time metaprogramming)
    CompileTime, // compile_time (compile-time execution)

    // FFI
    Ffi,     // ffi (foreign function interface)
    Include, // include (header inclusion)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    current: usize,
    start: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            start: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            self.start = self.current;
            let start_line = self.line;
            let start_column = self.column;

            match self.scan_token()? {
                Some(token_type) => {
                    let lexeme = self.get_lexeme_from_current();
                    tokens.push(Token::new(token_type, lexeme, start_line, start_column));
                }
                None => {} // Skip whitespace/comments
            }
        }

        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
        ));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<TokenType>, String> {
        let c = self.advance();

        match c {
            // Single character tokens
            '(' => Ok(Some(TokenType::LeftParen)),
            ')' => Ok(Some(TokenType::RightParen)),
            '{' => Ok(Some(TokenType::LeftBrace)),
            '}' => Ok(Some(TokenType::RightBrace)),
            '[' => Ok(Some(TokenType::LeftBracket)),
            ']' => Ok(Some(TokenType::RightBracket)),
            ',' => Ok(Some(TokenType::Comma)),
            ';' => Ok(Some(TokenType::Semicolon)),
            '?' => Ok(Some(TokenType::Question)),
            '@' => Ok(Some(TokenType::At)),
            '$' => {
                // Check if this is money ($ followed by digits) or string interpolation
                if self.peek().is_ascii_digit() {
                    self.scan_money('$')
                } else {
                    Ok(Some(TokenType::Dollar))
                }
            }

            // Operators that might be multi-character
            '+' => Ok(Some(TokenType::Plus)),
            '%' => Ok(Some(TokenType::Modulo)),
            '/' => {
                if self.match_char('/') {
                    // Line comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(TokenType::Divide))
                }
            }

            '-' => {
                if self.match_char('>') {
                    Ok(Some(TokenType::Arrow))
                } else {
                    Ok(Some(TokenType::Minus))
                }
            }

            '*' => {
                if self.match_char('*') {
                    Ok(Some(TokenType::Power))
                } else {
                    Ok(Some(TokenType::Multiply))
                }
            }

            '=' => {
                if self.match_char('=') {
                    Ok(Some(TokenType::Equal))
                } else if self.match_char('>') {
                    Ok(Some(TokenType::FatArrow))
                } else {
                    Ok(Some(TokenType::Assign))
                }
            }

            '!' => {
                if self.match_char('=') {
                    Ok(Some(TokenType::NotEqual))
                } else {
                    Ok(Some(TokenType::Exclamation))
                }
            }

            '<' => {
                if self.match_char('<') {
                    Ok(Some(TokenType::LeftShift))
                } else if self.match_char('-') {
                    Ok(Some(TokenType::Bind))
                } else if self.match_char('=') {
                    Ok(Some(TokenType::LessEqual))
                } else {
                    Ok(Some(TokenType::Less))
                }
            }

            '>' => {
                if self.match_char('>') {
                    Ok(Some(TokenType::RightShift))
                } else if self.match_char('=') {
                    Ok(Some(TokenType::GreaterEqual))
                } else {
                    Ok(Some(TokenType::Greater))
                }
            }

            '~' => {
                if self.match_char('=') && self.match_char('=') {
                    Ok(Some(TokenType::ConstantTimeEq))
                } else {
                    Ok(Some(TokenType::Tilde))
                }
            }

            '&' => Ok(Some(TokenType::Ampersand)),

            '^' => Ok(Some(TokenType::Caret)),

            '|' => {
                if self.match_char('>') {
                    Ok(Some(TokenType::Pipeline))
                } else {
                    Ok(Some(TokenType::Pipe))
                }
            }

            '.' => {
                if self.match_char('.') {
                    if self.match_char('<') {
                        Ok(Some(TokenType::RangeExclusive))
                    } else {
                        Ok(Some(TokenType::Range))
                    }
                } else {
                    Ok(Some(TokenType::Dot))
                }
            }

            ':' => Ok(Some(TokenType::Colon)),

            '#' => {
                // Check if it's a hex color or comment
                // Hex colors are #RGB, #RGBA, #RRGGBB, or #RRGGBBAA
                if self.looks_like_hex_color() {
                    self.scan_hex_color()
                } else {
                    // Line comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                }
            }

            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(Some(TokenType::Newline))
            }

            // String literals
            '"' => self.scan_string(),
            '\'' => self.scan_char(),

            // Numbers, dates, times, money
            c if c.is_ascii_digit() => self.scan_number_or_date_time(),

            // Currency symbols
            '€' | '¥' | '£' | '₿' | '₽' | '₺' | '₹' | '₴' => self.scan_money(c),

            // Identifiers, keywords, and underscore
            c if self.is_alpha(c) => self.scan_identifier_or_keyword(),
            '_' => Ok(Some(TokenType::Underscore)),

            // Execute command: j;
            'j' if self.peek() == ';' => {
                self.advance(); // consume ';'
                Ok(Some(TokenType::Execute))
            }

            // Emoji (Unicode) - only actual emoji, not all multi-byte chars
            c if is_emoji(c) => {
                let emoji = c.to_string();
                Ok(Some(TokenType::Emoji(emoji)))
            }

            _ => Err(JError::new(
                crate::error::ErrorKind::UnexpectedCharacter,
                format!("Unexpected character '{}'", c),
            )
            .with_location(self.line, self.column)
            .with_tip(format!("Character '{}' is not valid in this context", c))
            .with_solution(
                "Remove this character or check if you meant to use a different symbol".to_string(),
            )
            .to_string()),
        }
    }

    fn scan_string(&mut self) -> Result<Option<TokenType>, String> {
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if self.peek() == '\\' {
                self.advance(); // consume backslash
                match self.advance() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    'a' => value.push('\x07'), // bell
                    'b' => value.push('\x08'), // backspace
                    'f' => value.push('\x0C'), // form feed
                    'v' => value.push('\x0B'), // vertical tab
                    '0' => value.push('\0'),   // null
                    '?' => value.push('?'),
                    // Color shortcuts: \c{red}, \c{bold}, etc.
                    'c' if self.peek() == '{' => {
                        self.advance(); // consume '{'
                        let mut color_name = String::new();
                        while self.peek() != '}' && !self.is_at_end() {
                            color_name.push(self.advance());
                        }
                        if self.peek() == '}' {
                            self.advance(); // consume '}'
                                            // Convert color name to ANSI escape
                            let ansi_code = match color_name.as_str() {
                                "red" => "\x1b[31m",
                                "green" => "\x1b[32m",
                                "yellow" => "\x1b[33m",
                                "blue" => "\x1b[34m",
                                "magenta" => "\x1b[35m",
                                "cyan" => "\x1b[36m",
                                "white" => "\x1b[37m",
                                "bold" => "\x1b[1m",
                                "dim" => "\x1b[2m",
                                "ul" => "\x1b[4m",
                                "reset" => "\x1b[0m",
                                _ => "",
                            };
                            value.push_str(ansi_code);
                        } else {
                            return Err("Unterminated color escape sequence".to_string());
                        }
                    }
                    // Emoji shortcuts: \emoji{fire}, \emoji{heart}, etc.
                    // Check this BEFORE the simple 'e' escape to avoid unreachable pattern
                    'e' if self.peek_ahead(4) == Some("moji".to_string()) => {
                        // Skip "moji"
                        for _ in 0..4 {
                            self.advance();
                        }
                        if self.peek() == '{' {
                            self.advance(); // consume '{'
                            let mut emoji_name = String::new();
                            while self.peek() != '}' && !self.is_at_end() {
                                emoji_name.push(self.advance());
                            }
                            if self.peek() == '}' {
                                self.advance(); // consume '}'
                                                // Convert emoji name to actual emoji
                                let emoji = match emoji_name.as_str() {
                                    "fire" => "🔥",
                                    "heart" => "❤️",
                                    "sparkles" => "✨",
                                    "rocket" => "🚀",
                                    "star" => "⭐",
                                    "thumbs_up" => "👍",
                                    "smile" => "😊",
                                    "cool" => "😎",
                                    "party" => "🎉",
                                    "check" => "✅",
                                    "cross" => "❌",
                                    "warning" => "⚠️",
                                    _ => &emoji_name, // fallback to name
                                };
                                value.push_str(emoji);
                            } else {
                                return Err("Unterminated emoji escape sequence".to_string());
                            }
                        } else {
                            value.push('e'); // not an emoji escape
                        }
                    }
                    // Simple escape: \e
                    'e' => value.push('\x1B'), // escape
                    // Unicode escape: \U{HHHHHH}
                    'U' if self.peek() == '{' => {
                        self.advance(); // consume '{'
                        let mut hex = String::new();
                        while self.peek() != '}' && !self.is_at_end() && hex.len() < 6 {
                            if self.peek().is_ascii_hexdigit() {
                                hex.push(self.advance());
                            } else {
                                return Err("Invalid Unicode escape sequence".to_string());
                            }
                        }
                        if self.peek() == '}' {
                            self.advance(); // consume '}'
                            if let Ok(code_point) = u32::from_str_radix(&hex, 16) {
                                if let Some(ch) = char::from_u32(code_point) {
                                    value.push(ch);
                                } else {
                                    return Err("Invalid Unicode code point".to_string());
                                }
                            } else {
                                return Err("Invalid Unicode escape sequence".to_string());
                            }
                        } else {
                            return Err("Unterminated Unicode escape sequence".to_string());
                        }
                    }
                    // Hex escape: \xHH
                    'x' => {
                        let mut hex = String::new();
                        for _ in 0..2 {
                            if self.peek().is_ascii_hexdigit() {
                                hex.push(self.advance());
                            } else {
                                return Err("Invalid hex escape sequence".to_string());
                            }
                        }
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            value.push(byte as char);
                        } else {
                            return Err("Invalid hex escape sequence".to_string());
                        }
                    }
                    c => value.push(c),
                }
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(JError::unterminated_string(self.line, self.column).to_string());
        }

        self.advance(); // closing quote
        Ok(Some(TokenType::String(value)))
    }

    fn scan_char(&mut self) -> Result<Option<TokenType>, String> {
        if self.is_at_end() {
            return Err("Unterminated character literal".to_string());
        }

        let c = self.advance();

        if self.peek() != '\'' {
            return Err("Character literal must contain exactly one character".to_string());
        }

        self.advance(); // closing quote
        Ok(Some(TokenType::Char(c)))
    }

    fn scan_number_or_date_time(&mut self) -> Result<Option<TokenType>, String> {
        // Check if this might be a date (YYYY-MM-DD format)
        if self.current + 9 < self.input.len()
            && self.input[self.current + 4] == '-'
            && self.input[self.current + 7] == '-'
        {
            return self.scan_date_time();
        }

        // Check if this might be a time (HH:MM:SS format)
        if self.current + 7 < self.input.len()
            && self.input[self.current + 2] == ':'
            && self.input[self.current + 5] == ':'
        {
            return self.scan_time();
        }

        // Regular number parsing
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for decimal point
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume '.'

            while self.peek().is_ascii_digit() {
                self.advance();
            }

            let value: f64 = self
                .get_lexeme_from_current()
                .parse()
                .map_err(|_| "Invalid float literal")?;
            Ok(Some(TokenType::Float(value)))
        } else {
            let value: i64 = self
                .get_lexeme_from_current()
                .parse()
                .map_err(|_| "Invalid integer literal")?;
            Ok(Some(TokenType::Integer(value)))
        }
    }

    fn scan_date_time(&mut self) -> Result<Option<TokenType>, String> {
        // Scan YYYY-MM-DD
        for _ in 0..10 {
            if self.is_at_end() {
                return Err("Invalid date format".to_string());
            }
            self.advance();
        }

        // Check if there's a time component (space + HH:MM:SS)
        if self.peek() == ' ' && self.current + 9 < self.input.len() {
            self.advance(); // space

            // Scan HH:MM:SS
            for _ in 0..8 {
                if self.is_at_end() {
                    return Err("Invalid datetime format".to_string());
                }
                self.advance();
            }

            let datetime = self.get_lexeme_from_current();
            Ok(Some(TokenType::DateTime(datetime)))
        } else {
            let date = self.get_lexeme_from_current();
            Ok(Some(TokenType::Date(date)))
        }
    }

    fn scan_time(&mut self) -> Result<Option<TokenType>, String> {
        // Scan HH:MM:SS
        for _ in 0..8 {
            if self.is_at_end() {
                return Err("Invalid time format".to_string());
            }
            self.advance();
        }

        let time = self.get_lexeme_from_current();
        Ok(Some(TokenType::Time(time)))
    }

    fn scan_money(&mut self, currency_char: char) -> Result<Option<TokenType>, String> {
        let currency_symbol = currency_char.to_string();

        // Skip whitespace after currency symbol
        while self.peek() == ' ' {
            self.advance();
        }

        // Parse the number part
        let mut has_digits = false;
        let _has_decimal = false;

        // Handle negative amounts
        if self.peek() == '-' {
            self.advance();
        }

        while self.peek().is_ascii_digit() {
            self.advance();
            has_digits = true;
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume '.'
                            // has_decimal = true; // Not currently used

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        if !has_digits {
            return Err("Invalid money format - no digits found".to_string());
        }

        let amount_str = self.input[self.start + 1..self.current]
            .iter()
            .collect::<String>();
        let amount: f64 = amount_str.parse().map_err(|_| "Invalid money amount")?;

        Ok(Some(TokenType::Money(currency_symbol, amount)))
    }

    fn scan_identifier_or_keyword(&mut self) -> Result<Option<TokenType>, String> {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.get_lexeme_from_current();
        let token_type = match text.as_str() {
            // Types
            "str" => TokenType::Str,
            "int" => TokenType::Int,
            "float" => TokenType::FloatType,
            "bool" => TokenType::Bool,
            "list" => TokenType::List,
            "dict" => TokenType::Dict,
            "tuple" => TokenType::Tuple,
            "vec" => TokenType::Vec,
            "mat" => TokenType::Mat,
            "vec3" => TokenType::Vec3,
            "vec4" => TokenType::Vec4,
            "mat2" => TokenType::Mat2,
            "mat3" => TokenType::Mat3,
            "mat4" => TokenType::Mat4,
            "set" => TokenType::Set,
            "counter" => TokenType::Counter,
            "deque" => TokenType::Deque,
            "priorityq" => TokenType::PriorityQ,
            "graph" => TokenType::Graph,
            "tree" => TokenType::Tree,
            "grid" => TokenType::Grid,
            // New OP collection types
            "queue" => TokenType::Queue,
            "ring" => TokenType::Ring,
            "sorted" => TokenType::Sorted,
            "bag" => TokenType::Bag,
            "view" => TokenType::ViewType,
            "prio" => TokenType::Prio,
            "diff" => TokenType::Diff,
            // Advanced array types
            "span" => TokenType::Span,
            "mut_span" => TokenType::MutSpan,
            "chunk" => TokenType::Chunk,
            "sparse" => TokenType::Sparse,
            "stride" => TokenType::Stride,
            "flat" => TokenType::Flat,
            "mut_flat" => TokenType::MutFlat,
            "interval" => TokenType::IntervalType,
            "union" => TokenType::Union,
            "char" => TokenType::CharType,
            "emoji" => TokenType::EmojiType,
            "ascii" => TokenType::Ascii,
            "money" => TokenType::MoneyType,
            "hex" => TokenType::HexType,
            "date" => TokenType::DateType,
            "time" => TokenType::TimeType,
            "datetime" => TokenType::DateTimeType,
            "any" => TokenType::Any,
            "expr" => TokenType::Expr,

            // Keywords
            "fn" => TokenType::Fn,
            "enum" => TokenType::EnumKeyword,
            "static" => TokenType::Static,
            "class" => TokenType::Class,
            "trait" => TokenType::Trait,
            "pub" => TokenType::Pub,
            "priv" => TokenType::Priv,

            "if" => TokenType::If,
            "else" => TokenType::Else,
            "match" => TokenType::Match,
            "case" => TokenType::Case,
            "while" => TokenType::While,
            "loop" => TokenType::Loop,
            "for" => TokenType::For,
            // Advanced loop keywords
            "sweep" => TokenType::Sweep,
            "shrink" => TokenType::Shrink,
            "meet" => TokenType::Meet,
            "binary" => TokenType::Binary,
            "dp" => TokenType::Dp,
            "while_nonzero" => TokenType::WhileNonzero,
            "while_change" => TokenType::WhileChange,
            "while_match" => TokenType::WhileMatch,
            "in" => TokenType::In,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "defer" => TokenType::Defer,
            "converge" => TokenType::Converge,
            "memo" => TokenType::MemoVar,
            "extend" => TokenType::Extend,
            "phantom" => TokenType::Phantom,
            "mirror" => TokenType::Mirror,
            "fuzz" => TokenType::Fuzz,
            "within" => TokenType::Within,
            "rollback" => TokenType::Rollback,
            "retry" => TokenType::Retry,
            "race" => TokenType::Race,
            "barrier" => TokenType::Barrier,
            "pulse" => TokenType::Pulse,
            "untrusted" => TokenType::Untrusted,
            "secret" => TokenType::Secret,
            "secure" => TokenType::Secure,
            "canary" => TokenType::Canary,
            "component" => TokenType::Component,
            "contract" => TokenType::Contract,
            "workspace" => TokenType::Workspace,
            "env" => TokenType::Env,
            "packet" => TokenType::Packet,
            "gui" => TokenType::Gui,
            "sql" => TokenType::Sql,
            "embed" => TokenType::Embed,
            "triple" => TokenType::Triple,
            "shield" => TokenType::Shield,
            "deterministic" => TokenType::Deterministic,
            "audit" => TokenType::Audit,
            "layout" => TokenType::Layout,
            "fixed" => TokenType::Fixed,
            "sequence" => TokenType::Sequence,
            "pure" => TokenType::Pure,
            "effect" => TokenType::Effect,
            "invariant" => TokenType::Invariant,
            "constraint" => TokenType::Constraint,
            "solver" => TokenType::Solver,
            "flood" => TokenType::Flood,
            "ignite" => TokenType::Ignite,
            "by" => TokenType::By,
            "of" => TokenType::Of,
            "with" => TokenType::With,
            "cond" => TokenType::Cond,
            "this" => TokenType::This,
            "self" => TokenType::Self_,
            "rev" => TokenType::Reverse,
            "step" => TokenType::Step,
            "until" => TokenType::Until,
            "where" => TokenType::Where,
            "zip" => TokenType::Zip,
            "inf" => TokenType::Infinity(true),
            "-inf" => TokenType::Infinity(false),

            // Advanced J features
            "auto" => TokenType::Auto,
            "cheat" => TokenType::Cheat,
            "live" => TokenType::Live,
            "why" => TokenType::Why,
            "blend" => TokenType::Blend,
            "echo" => TokenType::Echo,
            "trace" => TokenType::Trace,
            "guard" => TokenType::Guard,
            "lens" => TokenType::Lens,
            "predict" => TokenType::Predict,
            "forever" => TokenType::Forever,
            "quantum" => TokenType::Quantum,

            // Memory management
            "arena" => TokenType::Arena,
            "stack" => TokenType::Stack,
            "pool" => TokenType::Pool,
            "tight" => TokenType::Tight,
            "recycle" => TokenType::Recycle,

            // Concurrency
            "parallel" => TokenType::Parallel,
            "threaded" => TokenType::Threaded,
            "scope" => TokenType::Scope,
            "cancel" => TokenType::Cancel,

            // Testing
            "test" => TokenType::Test,
            "property" => TokenType::Property,
            "assert" => TokenType::Assert,

            // Macros
            "macro" => TokenType::Macro,
            "compile_time" => TokenType::CompileTime,

            // FFI
            "ffi" => TokenType::Ffi,
            "include" => TokenType::Include,

            "return" => TokenType::Return,
            "yield" => TokenType::YieldKeyword,
            "async" => TokenType::Async,
            "await" => TokenType::Await,
            "task" => TokenType::Task,
            "gen" => TokenType::Gen,

            "use" => TokenType::Use,
            "import" => TokenType::Import,
            "module" => TokenType::Module,
            "mod" => TokenType::Mod,

            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "panic" => TokenType::Panic,

            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,

            "true" => TokenType::Boolean(true),
            "false" => TokenType::Boolean(false),

            _ => TokenType::Identifier(text),
        };

        Ok(Some(token_type))
    }

    fn scan_hex_color(&mut self) -> Result<Option<TokenType>, String> {
        let mut hex = String::from("#");

        while self.is_hex_digit(self.peek()) && hex.len() < 9 {
            // #RRGGBBAA max
            hex.push(self.advance());
        }

        if hex.len() != 4 && hex.len() != 7 && hex.len() != 9 {
            return Err("Invalid hex color format".to_string());
        }

        Ok(Some(TokenType::Hex(hex)))
    }

    fn looks_like_hex_color(&self) -> bool {
        // Check if the next characters form a valid hex color pattern
        let mut pos = self.current;
        let mut hex_count = 0;

        while pos < self.input.len() && hex_count < 8 {
            let c = self.input[pos];
            if !c.is_ascii_hexdigit() {
                break;
            }
            hex_count += 1;
            pos += 1;
        }

        // Valid hex colors are 3, 4, 6, or 8 hex digits
        matches!(hex_count, 3 | 4 | 6 | 8)
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            self.column += 1;
            let c = self.input[self.current];
            self.current += 1;
            c
        } else {
            '\0'
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.current + 1]
        }
    }

    fn peek_ahead(&self, n: usize) -> Option<String> {
        if self.current + n > self.input.len() {
            None
        } else {
            Some(self.input[self.current..self.current + n].iter().collect())
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn is_hex_digit(&self, c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    fn get_lexeme_from_current(&self) -> String {
        self.input[self.start..self.current].iter().collect()
    }
}

// Helper function to check if a character is an emoji
fn is_emoji(c: char) -> bool {
    matches!(c,
        '\u{1F600}'..='\u{1F64F}' | // Emoticons
        '\u{1F300}'..='\u{1F5FF}' | // Misc Symbols and Pictographs
        '\u{1F680}'..='\u{1F6FF}' | // Transport and Map
        '\u{1F700}'..='\u{1F77F}' | // Alchemical Symbols
        '\u{1F780}'..='\u{1F7FF}' | // Geometric Shapes Extended
        '\u{1F800}'..='\u{1F8FF}' | // Supplemental Arrows-C
        '\u{1F900}'..='\u{1F9FF}' | // Supplemental Symbols and Pictographs
        '\u{1FA00}'..='\u{1FA6F}' | // Chess Symbols
        '\u{1FA70}'..='\u{1FAFF}' | // Symbols and Pictographs Extended-A
        '\u{2600}'..='\u{26FF}'   | // Misc symbols
        '\u{2700}'..='\u{27BF}'   | // Dingbats
        '\u{FE00}'..='\u{FE0F}'   | // Variation Selectors
        '\u{1F1E6}'..='\u{1F1FF}'   // Regional Indicator Symbols
    )
}
