use crate::lexer::{Token, TokenType};
use crate::error::JError;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassField {
    pub name: String,
    pub field_type: String,
    pub default_value: Option<AstNode>,
    pub is_public: bool,
    pub is_readonly: bool,
    pub is_static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    StringInterpolation {
        parts: Vec<AstNode>, // alternating string literals and expressions
    },
    Boolean(bool),
    Char(char),
    Emoji(String),
    Money(String, f64), // (currency_symbol, amount)
    Hex(String),
    Date(String),
    Time(String),
    DateTime(String),
    Infinity(bool), // true for +inf, false for -inf
    List(Vec<AstNode>),
    Dict(Vec<(AstNode, AstNode)>),
    Tuple(Vec<AstNode>),
    Vector(Vec<AstNode>), // 1D vector literal
    Matrix(Vec<Vec<AstNode>>), // 2D matrix literal
    
    // Variables and declarations
    Identifier(String),
    VarDeclaration {
        var_type: String,
        name: String,
        value: Box<AstNode>,
        immutable: bool,
        is_static: bool,
    },
    TypeConversion {
        target_type: String,
        name: String,
    },
    
    // Functions
    FunctionDeclaration {
        name: String,
        params: Vec<(String, String)>, // (type, name)
        return_type: Option<String>,
        body: Box<AstNode>,
        decorators: Vec<Decorator>, // @decorator syntax
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    /// Call on any expression (e.g. instance.method(args) or Class.new(args))
    Call {
        callee: Box<AstNode>,
        args: Vec<AstNode>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<AstNode>,
    },
    
    // Control flow
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Option<Box<AstNode>>,
    },
    Match {
        expr: Box<AstNode>,
        arms: Vec<MatchArm>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    For {
        var: String,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
    },
    ForIndexed {
        index_var: String,
        value_var: String,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
    },
    
    // Expressions
    Binary {
        left: Box<AstNode>,
        operator: BinaryOp,
        right: Box<AstNode>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<AstNode>,
    },
    Pipeline {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Index {
        object: Box<AstNode>,
        index: Box<AstNode>,
    },
    DotAccess {
        object: Box<AstNode>,
        field: String,
    },
    Slice {
        object: Box<AstNode>,
        start: Option<Box<AstNode>>,
        end: Option<Box<AstNode>>,
        step: Option<Box<AstNode>>,
    },
    
    // Statements
    Block(Vec<AstNode>),
    Expression(Box<AstNode>),
    Assignment {
        name: String,
        value: Box<AstNode>,
    },
    DestructuringAssignment {
        targets: Vec<String>,
        value: Box<AstNode>,
    },
    Return(Option<Box<AstNode>>),
    Break,
    Continue,
    Defer(Box<AstNode>),
    ConvergeLoop { body: Box<AstNode> },
    
    // Error handling
    TryExpression(Box<AstNode>), // expr?
    TryCatch {
        try_block: Box<AstNode>,
        catch_var: Option<String>,
        catch_block: Box<AstNode>,
        finally_block: Option<Box<AstNode>>,
    },
    Throw(Box<AstNode>),
    
    // Ranges
    Range {
        start: Box<AstNode>,
        end: Box<AstNode>,
        inclusive: bool,
        step: Option<Box<AstNode>>,
    },
    
    // J-specific constructs
    EnumDeclaration {
        name: String,
        backing_type: Option<String>,
        variants: Vec<(String, Option<AstNode>)>,
    },
    ClassDeclaration {
        name: String,
        parent: Option<String>,
        traits: Vec<String>,
        fields: Vec<ClassField>,
        methods: Vec<AstNode>, // FunctionDeclaration nodes
        static_fields: Vec<ClassField>,
        static_methods: Vec<AstNode>,
    },
    
    // Anonymous variable
    Underscore,
    
    // Execute command
    ExecuteFile {
        filename: String,
    },
    
    // Enhanced for loops
    ForReverse {
        var: String,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
    },
    ForStep {
        var: String,
        start: Box<AstNode>,
        step: Box<AstNode>,
        condition: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    ForZip {
        vars: Vec<String>,
        iterables: Vec<AstNode>,
        body: Box<AstNode>,
    },
    ForParallel {
        var: String,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
        workers: Option<Box<AstNode>>,
        ordered: bool,
    },
    ForChunked {
        var: String,
        iterable: Box<AstNode>,
        chunk_size: Box<AstNode>,
        body: Box<AstNode>,
    },
    ForFiltered {
        var: String,
        iterable: Box<AstNode>,
        filter: Box<AstNode>,
        body: Box<AstNode>,
    },
    ForWindowed {
        var: String,
        iterable: Box<AstNode>,
        window_size: Box<AstNode>,
        body: Box<AstNode>,
    },
    
    // Advanced J features
    AutoFunction {
        name: String,
        params: Vec<(String, String)>,
        body: Box<AstNode>,
    },
    CheatBlock {
        body: Box<AstNode>,
    },
    LiveVariable {
        var_type: String,
        name: String,
        value: Box<AstNode>,
    },
    WhyExpression {
        expr: Box<AstNode>,
    },
    BlendExpression {
        branches: Vec<(f64, AstNode)>, // (probability, expression)
    },
    EchoVariable {
        var_type: String,
        name: String,
        dependency: Box<AstNode>,
    },
    TraceBlock {
        body: Box<AstNode>,
    },
    GuardExpression {
        pre_conditions: Vec<AstNode>,
        post_conditions: Vec<AstNode>,
        body: Box<AstNode>,
    },
    LensView {
        target: Box<AstNode>,
        transform: Box<AstNode>,
    },
    
    // Memory management
    ArenaAllocation {
        arena: String,
        expr: Box<AstNode>,
    },
    StackAllocation {
        var_type: String,
        name: String,
        value: Box<AstNode>,
    },
    
    // Concurrency
    TaskSpawn {
        body: Box<AstNode>,
    },
    ChannelSend {
        channel: Box<AstNode>,
        value: Box<AstNode>,
    },
    ChannelReceive {
        channel: Box<AstNode>,
    },
    ScopeBlock {
        workers: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    
    // Testing
    TestCase {
        name: String,
        body: Box<AstNode>,
    },
    PropertyTest {
        name: String,
        var_type: String,
        var_name: String,
        body: Box<AstNode>,
    },
    Assertion {
        condition: Box<AstNode>,
        message: Option<String>,
    },
    
    // Macros
    MacroDefinition {
        name: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
    MacroCall {
        name: String,
        args: Vec<AstNode>,
    },
    
    // Generators and comprehensions
    Generator {
        params: Vec<String>,
        body: Box<AstNode>,
    },
    Yield {
        value: Box<AstNode>,
    },
    ListComprehension {
        expr: Box<AstNode>,
        var: String,
        iterable: Box<AstNode>,
        condition: Option<Box<AstNode>>,
    },
    DictComprehension {
        key_expr: Box<AstNode>,
        value_expr: Box<AstNode>,
        var: String,
        iterable: Box<AstNode>,
        condition: Option<Box<AstNode>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<AstNode>,
    pub body: AstNode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decorator {
    pub name: String,
    pub args: Vec<AstNode>, // Arguments for parameterized decorators like @retry(3)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(AstNode),
    Identifier(String),
    Wildcard,
    Range { start: AstNode, end: AstNode },
    List(Vec<Pattern>),
    Tuple(Vec<Pattern>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    And, Or,
    Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus, Not,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<AstNode, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue; // Skip newlines
            }
            
            statements.push(self.statement()?);
        }
        
        Ok(AstNode::Block(statements))
    }
    
    fn statement(&mut self) -> Result<AstNode, String> {
        // Execute command: j; -> filename
        if self.match_token(&TokenType::Execute) {
            return self.execute_statement();
        }
        
        // Type conversion: type*variable (parsed as type * variable)
        if self.is_type_token() && self.check_ahead(&TokenType::Multiply) {
            return self.type_conversion_statement();
        }
        
        // Static variable declaration: static type | name -> value
        if self.check(&TokenType::Static) {
            return self.var_declaration();
        }
        
        // Variable declaration: type | name -> value or static type | name -> value
        if self.is_type_token() || self.check(&TokenType::Static) {
            return self.var_declaration();
        }
        
        // Enum declaration: enum | name { variant = value, ... }
        if self.match_token(&TokenType::EnumKeyword) {
            return self.enum_declaration();
        }
        
        // Class declaration: class | name { ... }
        if self.match_token(&TokenType::Class) {
            return self.class_declaration();
        }
        
        // Check for decorators before function declaration
        if self.check(&TokenType::At) {
            // Look ahead to see if next non-decorator token is Fn
            let saved_pos = self.current;
            let mut found_fn = false;
            while self.current < self.tokens.len() && self.check(&TokenType::At) {
                self.advance(); // consume @
                if self.current < self.tokens.len() {
                    if matches!(self.tokens[self.current].token_type, TokenType::Identifier(_)) {
                        self.advance(); // consume decorator name
                    }
                    if self.current < self.tokens.len() && self.check(&TokenType::LeftParen) {
                        self.advance(); // consume (
                        let mut depth = 1;
                        while depth > 0 && self.current < self.tokens.len() {
                            match self.tokens[self.current].token_type {
                                TokenType::LeftParen => depth += 1,
                                TokenType::RightParen => depth -= 1,
                                _ => {}
                            }
                            if depth > 0 {
                                self.advance();
                            }
                        }
                        if self.current < self.tokens.len() {
                            self.advance(); // consume )
                        }
                    }
                }
            }
            if self.current < self.tokens.len() && matches!(self.tokens[self.current].token_type, TokenType::Fn) {
                found_fn = true;
            }
            self.current = saved_pos; // restore position
            
            if found_fn {
                return self.function_declaration();
            }
        }
        
        // Function or Lambda: fn | ...
        if self.match_token(&TokenType::Fn) {
            // Look ahead to see if this is a function declaration or lambda
            if self.check(&TokenType::Pipe) {
                let saved_pos = self.current;
                self.advance(); // consume |
                
                // Check if we have identifier followed by (
                if matches!(self.peek().token_type, TokenType::Identifier(_)) || 
                   matches!(self.peek().token_type, TokenType::Test) {
                    self.advance(); // consume identifier
                    
                    if self.check(&TokenType::LeftParen) {
                        // This is a function declaration: fn | name(params)
                        self.current = saved_pos; // restore position
                        return self.function_declaration();
                    }
                }
                
                // If we get here, it's a lambda
                self.current = saved_pos; // restore position
                return self.lambda_statement();
            }
            
            // If no pipe after fn, it's probably a function declaration
            return self.function_declaration();
        }
        
        // Control flow
        if self.match_token(&TokenType::If) {
            return self.if_statement();
        }
        
        if self.match_token(&TokenType::Match) {
            return self.match_statement();
        }
        
        if self.match_token(&TokenType::While) {
            return self.while_statement();
        }
        
        // Error handling
        if self.match_token(&TokenType::Try) {
            return self.try_catch_statement();
        }
        
        if self.match_token(&TokenType::Panic) {
            return self.throw_statement();
        }
        
        // FOR LOOPS - Check this BEFORE expression statement
        if self.match_token(&TokenType::For) || 
           (matches!(self.peek().token_type, TokenType::Identifier(_)) && self.check_ahead(&TokenType::In)) ||
           self.is_indexed_for_loop() ||
           self.check(&TokenType::In) {
            return self.for_statement();
        }
        
        if self.match_token(&TokenType::Return) {
            return self.return_statement();
        }
        
        if self.match_token(&TokenType::YieldKeyword) {
            return self.yield_statement();
        }
        
        if self.match_token(&TokenType::Break) {
            return Ok(AstNode::Break);
        }
        
        if self.match_token(&TokenType::Continue) {
            return Ok(AstNode::Continue);
        }

        if self.match_token(&TokenType::Defer) {
            let expr = self.expression()?;
            return Ok(AstNode::Defer(Box::new(expr)));
        }

        if self.match_token(&TokenType::Converge) {
            let body = self.block()?;
            return Ok(AstNode::ConvergeLoop { body: Box::new(body) });
        }
        
        // Check for destructuring assignment: (a, b, c) = expression
        if self.check(&TokenType::LeftParen) {
            let _start_pos = self.current;
            
            // Look ahead to see if this is a destructuring pattern
            let mut temp_pos = self.current + 1; // Skip the opening paren
            let mut is_destructuring = false;
            let mut paren_depth = 1;
            
            while temp_pos < self.tokens.len() && paren_depth > 0 {
                match &self.tokens[temp_pos].token_type {
                    TokenType::LeftParen => paren_depth += 1,
                    TokenType::RightParen => paren_depth -= 1,
                    TokenType::Assign if paren_depth == 0 => {
                        is_destructuring = true;
                        break;
                    }
                    _ => {}
                }
                temp_pos += 1;
            }
            
            // Check if we found an assignment after the closing paren
            if temp_pos < self.tokens.len() && matches!(self.tokens[temp_pos].token_type, TokenType::Assign) {
                is_destructuring = true;
            }
            
            if is_destructuring {
                return self.destructuring_assignment();
            }
        }
        
        // Block statement: { statements } (so { defer ... } is a block, not dict literal)
        if self.check(&TokenType::LeftBrace) {
            return self.block();
        }
        
        // Check for assignment statement: identifier = expression
        if matches!(self.peek().token_type, TokenType::Identifier(_)) && self.check_ahead(&TokenType::Assign) {
            return self.assignment_statement();
        }
        
        // Expression statement
        self.expression_statement()
    }
    
    fn type_conversion_statement(&mut self) -> Result<AstNode, String> {
        // Parse "type * variable" format
        let target_type = match &self.advance().token_type {
            TokenType::Str => "str".to_string(),
            TokenType::Int => "int".to_string(),
            TokenType::FloatType => "float".to_string(),
            TokenType::Bool => "bool".to_string(),
            TokenType::List => "list".to_string(),
            TokenType::Vec => "vec".to_string(),
            TokenType::Mat => "mat".to_string(),
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected type name in type conversion".to_string()),
        };
        
        self.consume(&TokenType::Multiply, "Expected '*' after type in type conversion")?;
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected variable name after '*' in type conversion".to_string()),
        };
        
        Ok(AstNode::TypeConversion { target_type, name })
    }
    
    fn execute_statement(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::Arrow, "Expected '->' after 'j;'")?;
        
        let token = self.advance().clone();
        let filename = match &token.token_type {
            TokenType::Identifier(name) => {
                // Handle filename with extension
                if self.match_token(&TokenType::Dot) {
                    let ext_token = self.advance().clone();
                    let extension = match &ext_token.token_type {
                        TokenType::Identifier(ext) => ext.clone(),
                        _ => return Err("Expected file extension after '.'".to_string()),
                    };
                    format!("{}.{}", name, extension)
                } else {
                    name.clone()
                }
            }
            TokenType::String(filename) => filename.clone(),
            _ => return Err("Expected filename after 'j; ->'".to_string()),
        };
        
        Ok(AstNode::ExecuteFile { filename })
    }
    
    fn var_declaration(&mut self) -> Result<AstNode, String> {
        let mut immutable = false;
        let mut is_static = false;
        
        // Check for static prefix
        if self.match_token(&TokenType::Static) {
            is_static = true;
        }
        
        // Check for immutable prefix !
        if self.match_token(&TokenType::Exclamation) {
            immutable = true;
        }
        
        let var_type = match &self.advance().token_type {
            TokenType::Str => "str".to_string(),
            TokenType::Int => "int".to_string(),
            TokenType::FloatType => "float".to_string(),
            TokenType::Bool => "bool".to_string(),
            TokenType::List => "list".to_string(),
            TokenType::Dict => "dict".to_string(),
            TokenType::Tuple => "tuple".to_string(),
            TokenType::Vec => "vec".to_string(),
            TokenType::Mat => "mat".to_string(),
            TokenType::Vec3 => "vec3".to_string(),
            TokenType::Vec4 => "vec4".to_string(),
            TokenType::Mat2 => "mat2".to_string(),
            TokenType::Mat3 => "mat3".to_string(),
            TokenType::Mat4 => "mat4".to_string(),
            TokenType::Set => "set".to_string(),
            TokenType::Counter => "counter".to_string(),
            TokenType::Deque => "deque".to_string(),
            TokenType::PriorityQ => "priorityq".to_string(),
            TokenType::Graph => "graph".to_string(),
            TokenType::Tree => "tree".to_string(),
            TokenType::Grid => "grid".to_string(),
            TokenType::CharType => "char".to_string(),
            TokenType::EmojiType => "emoji".to_string(),
            TokenType::Ascii => "ascii".to_string(),
            TokenType::MoneyType => "money".to_string(),
            TokenType::HexType => "hex".to_string(),
            TokenType::DateType => "date".to_string(),
            TokenType::TimeType => "time".to_string(),
            TokenType::DateTimeType => "datetime".to_string(),
            TokenType::Any => "any".to_string(),
            TokenType::Expr => "expr".to_string(),
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected type name".to_string()),
        };
        
        self.consume(&TokenType::Pipe, "Expected '|' after type")?;
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected variable name".to_string()),
        };
        
        self.consume(&TokenType::Arrow, "Expected '->' after variable name")?;
        
        let value = self.expression()?;
        
        Ok(AstNode::VarDeclaration {
            var_type,
            name,
            value: Box::new(value),
            immutable,
            is_static,
        })
    }
    
    fn parse_decorator(&mut self) -> Result<Decorator, String> {
        self.consume(&TokenType::At, "Expected '@' for decorator")?;
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected decorator name after '@'".to_string()),
        };
        
        let args = if self.check(&TokenType::LeftParen) {
            self.advance(); // consume '('
            let mut args = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    args.push(self.expression()?);
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }
            self.consume(&TokenType::RightParen, "Expected ')' after decorator arguments")?;
            args
        } else {
            Vec::new()
        };
        
        Ok(Decorator { name, args })
    }
    
    fn function_declaration(&mut self) -> Result<AstNode, String> {
        // Parse decorators (can be multiple, applied bottom-to-top)
        let mut decorators = Vec::new();
        while self.check(&TokenType::At) {
            decorators.push(self.parse_decorator()?);
        }
        

        
        // Consume Fn token if present
        if self.check(&TokenType::Fn) {
            self.advance();
        }

        // Optional return type
        let return_type = if self.is_type_token() { // Now checks for all known type tokens
            // Advance and clone the lexeme of the type token
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        


        self.consume(&TokenType::Pipe, "Expected '|' after 'fn'")?;
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            TokenType::Test => "test".to_string(),
            _ => return Err("Expected function name".to_string()),
        };
        
        self.consume(&TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param_type = self.advance().lexeme.clone();
                self.consume(&TokenType::Pipe, "Expected '|' after parameter type")?;
                let param_name = match &self.advance().token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected parameter name".to_string()),
                };
                params.push((param_type, param_name));
                
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;
        self.consume(&TokenType::Greater, "Expected '>' before function body")?;
        
        let body = if self.check(&TokenType::LeftBrace) {
            self.block()?
        } else {
            self.expression()?
        };
        
        Ok(AstNode::FunctionDeclaration {
            name,
            params,
            return_type,
            body: Box::new(body),
            decorators,
        })
    }
    
    fn if_statement(&mut self) -> Result<AstNode, String> {
        let condition = self.expression()?;
        
        let then_branch = if self.check(&TokenType::LeftBrace) {
            self.block()?
        } else {
            self.statement()?
        };
        
        let else_branch = if self.match_token(&TokenType::Else) {
            Some(Box::new(if self.check(&TokenType::LeftBrace) {
                self.block()?
            } else {
                self.statement()?
            }))
        } else {
            None
        };
        
        Ok(AstNode::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }
    
    fn match_statement(&mut self) -> Result<AstNode, String> {
        let expr = self.expression()?;
        
        self.consume(&TokenType::LeftBrace, "Expected '{' after match expression")?;
        
        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            // Skip newlines before pattern
            while self.match_token(&TokenType::Newline) {}
            
            if self.check(&TokenType::RightBrace) {
                break;
            }
            
            let pattern = self.pattern()?;
            
            let guard = if self.match_token(&TokenType::If) {
                Some(self.expression()?)
            } else {
                None
            };
            
            self.consume(&TokenType::Colon, "Expected ':' after match pattern")?;
            
            let body = if self.check(&TokenType::LeftBrace) {
                self.block()?
            } else {
                self.expression()?
            };
            
            arms.push(MatchArm { pattern, guard, body });
            
            if !self.check(&TokenType::RightBrace) {
                self.match_token(&TokenType::Newline); // Optional newline
            }
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after match arms")?;
        
        Ok(AstNode::Match {
            expr: Box::new(expr),
            arms,
        })
    }
    
    fn while_statement(&mut self) -> Result<AstNode, String> {
        let condition = self.expression()?;
        let body = if self.check(&TokenType::LeftBrace) {
            self.block()?
        } else {
            self.statement()?
        };
        
        Ok(AstNode::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }
    
    fn try_catch_statement(&mut self) -> Result<AstNode, String> {
        // Parse try block
        let try_block = self.block()?;
        
        // Parse catch block
        self.consume(&TokenType::Catch, "Expected 'catch' after try block")?;
        
        // Optional catch variable: catch e { ... } or catch { ... }
        let catch_var = if self.check(&TokenType::Identifier("".to_string())) {
            let var_name = match &self.advance().token_type {
                TokenType::Identifier(name) => Some(name.clone()),
                _ => None,
            };
            var_name
        } else {
            None
        };
        
        let catch_block = self.block()?;
        
        // Optional finally block
        let finally_block = if self.match_token(&TokenType::Finally) {
            Some(Box::new(self.block()?))
        } else {
            None
        };
        
        Ok(AstNode::TryCatch {
            try_block: Box::new(try_block),
            catch_var,
            catch_block: Box::new(catch_block),
            finally_block,
        })
    }
    
    fn throw_statement(&mut self) -> Result<AstNode, String> {
        // panic "error message" or panic(expr)
        let error_expr = self.expression()?;
        Ok(AstNode::Throw(Box::new(error_expr)))
    }
    
    fn for_statement(&mut self) -> Result<AstNode, String> {
        // Handle different for loop patterns:
        // 1. i in nums { ... }
        // 2. (i, v) in nums { ... }
        // 3. i in nums : expression
        // 4. i in reverse(nums) { ... } or i in nums rev { ... }
        // 5. i in zip(a, b) { ... }
        // 6. i in parallel(nums) { ... }
        
        if self.match_token(&TokenType::LeftParen) {
            // Indexed iteration: (i, v) in nums
            let index_var = match &self.advance().token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected index variable name".to_string()),
            };
            
            self.consume(&TokenType::Comma, "Expected ',' after index variable")?;
            
            let value_var = match &self.advance().token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected value variable name".to_string()),
            };
            
            self.consume(&TokenType::RightParen, "Expected ')' after variables")?;
            self.consume(&TokenType::In, "Expected 'in' after variables")?;
            
            let iterable = self.expression()?;
            
            let body = if self.match_token(&TokenType::Colon) {
                // Single expression body: (i,v) in nums : out(i v)
                self.expression()?
            } else {
                // Block body
                self.block()?
            };
            
            Ok(AstNode::ForIndexed {
                index_var,
                value_var,
                iterable: Box::new(iterable),
                body: Box::new(body),
            })
        } else {
            // Simple iteration: i in nums or in nums (anonymous)
            let var = if self.check(&TokenType::In) {
                // Anonymous: in nums : out(_)
                self.advance(); // consume 'in'
                "_".to_string()
            } else {
                // Named: i in nums
                let var_name = match &self.advance().token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected variable name in for loop".to_string()),
                };
                
                self.consume(&TokenType::In, "Expected 'in' after for loop variable")?;
                var_name
            };
            
            let iterable = self.expression()?;
            
            // Check for enhanced for loop variants after iterable
            // Check for 'rev' keyword: i in nums rev
            if self.match_token(&TokenType::Reverse) {
                let body = if self.match_token(&TokenType::Colon) {
                    self.expression()?
                } else {
                    self.block()?
                };
                return Ok(AstNode::ForReverse {
                    var,
                    iterable: Box::new(iterable),
                    body: Box::new(body),
                });
            }
            
            // Check for 'if' filter: i in nums if condition
            if self.match_token(&TokenType::If) {
                let filter = self.expression()?;
                let body = if self.match_token(&TokenType::Colon) {
                    self.expression()?
                } else {
                    self.block()?
                };
                return Ok(AstNode::ForFiltered {
                    var,
                    iterable: Box::new(iterable),
                    filter: Box::new(filter),
                    body: Box::new(body),
                });
            }
            
            // Check for enhanced for loop variants in function calls
            let enhanced_loop = match &iterable {
                AstNode::FunctionCall { name, args } => {
                    match name.as_str() {
                        "reverse" => {
                            if args.len() == 1 {
                                Some(AstNode::ForReverse {
                                    var: var.clone(),
                                    iterable: Box::new(args[0].clone()),
                                    body: Box::new(AstNode::Integer(0)), // Placeholder, will be replaced
                                })
                            } else {
                                None
                            }
                        }
                        "zip" => {
                            if args.len() >= 2 {
                                // For zip, we need multiple variables
                                // This is a simplified version - in practice you'd want (a,b) in zip(x,y)
                                Some(AstNode::ForZip {
                                    vars: vec![var.clone(), format!("{}_2", var)],
                                    iterables: args.clone(),
                                    body: Box::new(AstNode::Integer(0)), // Placeholder
                                })
                            } else {
                                None
                            }
                        }
                        "parallel" => {
                            if args.len() == 1 {
                                Some(AstNode::ForParallel {
                                    var: var.clone(),
                                    iterable: Box::new(args[0].clone()),
                                    body: Box::new(AstNode::Integer(0)), // Placeholder
                                    workers: None,
                                    ordered: false,
                                })
                            } else {
                                None
                            }
                        }
                        "chunks" => {
                            if args.len() == 2 {
                                Some(AstNode::ForChunked {
                                    var: var.clone(),
                                    iterable: Box::new(args[0].clone()),
                                    chunk_size: Box::new(args[1].clone()),
                                    body: Box::new(AstNode::Integer(0)), // Placeholder
                                })
                            } else {
                                None
                            }
                        }
                        _ => None
                    }
                }
                _ => None
            };
            
            let body = if self.match_token(&TokenType::Colon) {
                // Single expression body: i in nums : out(i)
                self.expression()?
            } else {
                // Block body
                self.block()?
            };
            
            // If we detected an enhanced loop, update its body
            if let Some(mut enhanced) = enhanced_loop {
                match &mut enhanced {
                    AstNode::ForReverse { body: ref mut b, .. } => *b = Box::new(body),
                    AstNode::ForZip { body: ref mut b, .. } => *b = Box::new(body),
                    AstNode::ForParallel { body: ref mut b, .. } => *b = Box::new(body),
                    AstNode::ForChunked { body: ref mut b, .. } => *b = Box::new(body),
                    _ => {}
                }
                Ok(enhanced)
            } else {
                Ok(AstNode::For {
                    var,
                    iterable: Box::new(iterable),
                    body: Box::new(body),
                })
            }
        }
    }
    
    fn return_statement(&mut self) -> Result<AstNode, String> {
        let value = if self.check(&TokenType::Newline) || self.is_at_end() {
            None
        } else {
            Some(Box::new(self.expression()?))
        };
        
        Ok(AstNode::Return(value))
    }
    
    fn yield_statement(&mut self) -> Result<AstNode, String> {
        let value = if self.check(&TokenType::Newline) || self.is_at_end() {
            return Err("yield requires a value".to_string());
        } else {
            Box::new(self.expression()?)
        };
        
        Ok(AstNode::Yield { value })
    }
    
    fn assignment_statement(&mut self) -> Result<AstNode, String> {
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected variable name in assignment".to_string()),
        };
        
        self.consume(&TokenType::Assign, "Expected '=' in assignment")?;
        
        let value = self.expression()?;
        
        Ok(AstNode::Assignment {
            name,
            value: Box::new(value),
        })
    }
    
    fn destructuring_assignment(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::LeftParen, "Expected '(' in destructuring assignment")?;
        
        let mut targets = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let target = match &self.advance().token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected variable name in destructuring assignment".to_string()),
                };
                targets.push(target);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' in destructuring assignment")?;
        self.consume(&TokenType::Assign, "Expected '=' in destructuring assignment")?;
        
        let value = self.expression()?;
        
        Ok(AstNode::DestructuringAssignment {
            targets,
            value: Box::new(value),
        })
    }
    
    fn expression_statement(&mut self) -> Result<AstNode, String> {
        let expr = self.expression()?;
        Ok(AstNode::Expression(Box::new(expr)))
    }
    
    fn expression(&mut self) -> Result<AstNode, String> {
        self.pipeline()
    }
    
    fn pipeline(&mut self) -> Result<AstNode, String> {
        let mut expr = self.arrow_lambda()?;
        
        while self.match_token(&TokenType::Pipeline) {
            let right = self.arrow_lambda()?;
            expr = AstNode::Pipeline {
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn arrow_lambda(&mut self) -> Result<AstNode, String> {
        // Check for arrow lambda: x => expr or (x, y) => expr
        let start_pos = self.current;
        
        // Try to parse as arrow lambda
        if let TokenType::Identifier(param) = &self.peek().token_type {
            let param_name = param.clone();
            self.advance();
            
            if self.match_token(&TokenType::FatArrow) {
                // Single parameter arrow lambda: x => body
                return Ok(AstNode::Lambda {
                    params: vec![param_name],
                    body: Box::new(self.or()?),
                });
            } else {
                // Not an arrow lambda, backtrack
                self.current = start_pos;
            }
        } else if self.check(&TokenType::LeftParen) {
            // Could be multi-param arrow lambda: (x, y) => body
            self.advance(); // consume (
            
            let mut params = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    if let TokenType::Identifier(name) = &self.advance().token_type {
                        params.push(name.clone());
                    } else {
                        self.current = start_pos;
                        return self.or();
                    }
                    
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }
            
            if self.match_token(&TokenType::RightParen) && self.match_token(&TokenType::FatArrow) {
                // Multi-parameter arrow lambda
                return Ok(AstNode::Lambda {
                    params,
                    body: Box::new(self.or()?),
                });
            } else {
                // Not an arrow lambda, backtrack
                self.current = start_pos;
            }
        }
        
        self.or()
    }
    
    fn or(&mut self) -> Result<AstNode, String> {
        let mut expr = self.and()?;
        
        while self.match_token(&TokenType::Or) {
            let operator = BinaryOp::Or;
            let right = self.and()?;
            expr = AstNode::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn and(&mut self) -> Result<AstNode, String> {
        let mut expr = self.equality()?;
        
        while self.match_token(&TokenType::And) {
            let operator = BinaryOp::And;
            let right = self.equality()?;
            expr = AstNode::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<AstNode, String> {
        let mut expr = self.comparison()?;
        
        while let Some(op) = self.match_equality_op() {
            let right = self.comparison()?;
            expr = AstNode::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<AstNode, String> {
        let mut expr = self.range_expr()?;
        
        while let Some(op) = self.match_comparison_op() {
            let right = self.range_expr()?;
            expr = AstNode::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn range_expr(&mut self) -> Result<AstNode, String> {
        let mut expr = self.term()?;
        
        // Check for range operators
        if self.match_token(&TokenType::Range) || self.match_token(&TokenType::RangeExclusive) {
            let inclusive = self.previous().token_type == TokenType::Range;
            
            let end = if self.is_at_end() || self.check(&TokenType::By) || 
                         self.check(&TokenType::RightParen) || self.check(&TokenType::Newline) ||
                         self.check(&TokenType::RightBrace) || self.check(&TokenType::RightBracket) ||
                         self.check(&TokenType::Comma) || self.check(&TokenType::Colon) {
                AstNode::Integer(i64::MAX) // Infinite range
            } else {
                self.term()?
            };
            
            let step = if self.match_token(&TokenType::By) {
                Some(Box::new(self.term()?))
            } else {
                None
            };
            
            expr = AstNode::Range {
                start: Box::new(expr),
                end: Box::new(end),
                inclusive,
                step,
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<AstNode, String> {
        let mut expr = self.factor()?;
        
        while let Some(op) = self.match_term_op() {
            let right = self.factor()?;
            expr = AstNode::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<AstNode, String> {
        let mut expr = self.unary()?;
        
        while let Some(op) = self.match_factor_op() {
            let right = self.unary()?;
            expr = AstNode::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<AstNode, String> {
        if let Some(op) = self.match_unary_op() {
            let expr = self.unary()?;
            return Ok(AstNode::Unary {
                operator: op,
                operand: Box::new(expr),
            });
        }
        
        self.postfix()
    }
    
    fn postfix(&mut self) -> Result<AstNode, String> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&TokenType::Question) {
                // Try expression: expr?
                expr = AstNode::TryExpression(Box::new(expr));
            } else if self.match_token(&TokenType::LeftParen) {
                // Function call
                let mut args = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&TokenType::RightParen, "Expected ')' after arguments")?;
                
                if let AstNode::Identifier(name) = expr {
                    expr = AstNode::FunctionCall { name, args };
                } else {
                    expr = AstNode::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
            } else if self.match_token(&TokenType::Dot) {
                // Dot access: expr.field
                let field = match &self.advance().token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected field name after '.'".to_string()),
                };
                expr = AstNode::DotAccess {
                    object: Box::new(expr),
                    field,
                };
            } else if self.match_token(&TokenType::LeftBracket) {
                // Check if this is slicing (contains ..) or indexing
                let _start_pos = self.current;
                
                // Look ahead to see if we have a slice pattern
                let mut has_slice = false;
                let mut temp_pos = self.current;
                let mut paren_depth = 0;
                
                while temp_pos < self.tokens.len() {
                    match &self.tokens[temp_pos].token_type {
                        TokenType::LeftBracket => paren_depth += 1,
                        TokenType::RightBracket => {
                            paren_depth -= 1;
                            if paren_depth < 0 {
                                break;
                            }
                        }
                        TokenType::Range => {
                            has_slice = true;
                            break;
                        }
                        _ => {}
                    }
                    temp_pos += 1;
                }
                
                if has_slice {
                    // Parse slice: [start .. end] or [start .. end by step]
                    let mut start = None;
                    let mut end = None;
                    let mut step = None;
                    
                    // Parse start (optional)
                    if !self.check(&TokenType::Range) {
                        start = Some(Box::new(self.term()?));
                    }
                    
                    // Consume ".."
                    self.consume(&TokenType::Range, "Expected '..' in slice")?;
                    
                    // Parse end (optional)
                    if !self.check(&TokenType::RightBracket) && !self.check(&TokenType::By) {
                        end = Some(Box::new(self.term()?));
                    }
                    
                    // Parse step (optional, after "by")
                    if self.match_token(&TokenType::By) {
                        step = Some(Box::new(self.term()?));
                    }
                    
                    self.consume(&TokenType::RightBracket, "Expected ']' after slice")?;
                    expr = AstNode::Slice {
                        object: Box::new(expr),
                        start,
                        end,
                        step,
                    };
                } else {
                    // Regular indexing: expr[index]
                    let index = self.expression()?;
                    self.consume(&TokenType::RightBracket, "Expected ']' after index")?;
                    expr = AstNode::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn primary(&mut self) -> Result<AstNode, String> {
        match &self.advance().token_type {
            TokenType::Boolean(b) => Ok(AstNode::Boolean(*b)),
            TokenType::Integer(i) => Ok(AstNode::Integer(*i)),
            TokenType::Float(f) => Ok(AstNode::Float(*f)),
            TokenType::String(s) => {
                let s_clone = s.clone();
                // Check for string interpolation patterns
                if s_clone.contains('$') || s_clone.contains('{') {
                    self.parse_string_interpolation(&s_clone)
                } else {
                    Ok(AstNode::String(s_clone))
                }
            }
            TokenType::Char(c) => Ok(AstNode::Char(*c)),
            TokenType::Emoji(e) => Ok(AstNode::Emoji(e.clone())),
            TokenType::Money(symbol, amount) => Ok(AstNode::Money(symbol.clone(), *amount)),
            TokenType::Hex(hex) => Ok(AstNode::Hex(hex.clone())),
            TokenType::Date(date) => Ok(AstNode::Date(date.clone())),
            TokenType::Time(time) => Ok(AstNode::Time(time.clone())),
            TokenType::DateTime(datetime) => Ok(AstNode::DateTime(datetime.clone())),
            TokenType::Infinity(positive) => Ok(AstNode::Infinity(*positive)),
            TokenType::Identifier(name) => Ok(AstNode::Identifier(name.clone())),
            TokenType::Underscore => Ok(AstNode::Underscore),
            
            TokenType::Pipe => {
                // Lambda expression: |param1, param2| body
                let mut params = Vec::new();
                if !self.check(&TokenType::Pipe) {
                    loop {
                        let param = match &self.advance().token_type {
                            TokenType::Identifier(name) => name.clone(),
                            _ => return Err("Expected parameter name in lambda".to_string()),
                        };
                        params.push(param);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(&TokenType::Pipe, "Expected '|' to close lambda parameters")?;
                let body = self.expression()?;
                
                Ok(AstNode::Lambda {
                    params,
                    body: Box::new(body),
                })
            }
            
            TokenType::Fn => {
                // Lambda expression: fn | param1, param2 > body
                self.consume(&TokenType::Pipe, "Expected '|' after 'fn'")?;
                
                let mut params = Vec::new();
                if !self.check(&TokenType::Greater) {
                    loop {
                        let param = match &self.advance().token_type {
                            TokenType::Identifier(name) => name.clone(),
                            _ => return Err("Expected parameter name in lambda".to_string()),
                        };
                        params.push(param);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(&TokenType::Greater, "Expected '>' in lambda expression")?;
                let body = self.expression()?;
                
                Ok(AstNode::Lambda {
                    params,
                    body: Box::new(body),
                })
            }
            
            TokenType::LeftParen => {
                // Could be tuple or grouped expression
                if self.check(&TokenType::RightParen) {
                    // Empty tuple
                    self.advance(); // consume )
                    Ok(AstNode::Tuple(Vec::new()))
                } else {
                    let first_expr = self.expression()?;
                    
                    if self.match_token(&TokenType::Comma) {
                        // Tuple with multiple elements
                        let mut elements = vec![first_expr];
                        
                        if !self.check(&TokenType::RightParen) {
                            loop {
                                elements.push(self.expression()?);
                                if !self.match_token(&TokenType::Comma) {
                                    break;
                                }
                            }
                        }
                        
                        self.consume(&TokenType::RightParen, "Expected ')' after tuple elements")?;
                        Ok(AstNode::Tuple(elements))
                    } else {
                        // Grouped expression
                        self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
                        Ok(first_expr)
                    }
                }
            }
            
            TokenType::LeftBracket => {
                // Could be list, vector, matrix, or list comprehension
                let mut elements = Vec::new();
                if !self.check(&TokenType::RightBracket) {
                    // Check if this is a list comprehension by looking ahead
                    let _start_pos = self.current;
                    let first_expr = self.expression()?;
                    
                    // Check for "for" keyword indicating comprehension
                    if self.match_token(&TokenType::For) {
                        // This is a list comprehension: [expr for var in iterable]
                        let var = match &self.advance().token_type {
                            TokenType::Identifier(name) => name.clone(),
                            _ => return Err("Expected variable name in list comprehension".to_string()),
                        };
                        
                        self.consume(&TokenType::In, "Expected 'in' in list comprehension")?;
                        let iterable = self.expression()?;
                        
                        // Optional condition with "if"
                        let condition = if self.match_token(&TokenType::If) {
                            Some(Box::new(self.expression()?))
                        } else {
                            None
                        };
                        
                        self.consume(&TokenType::RightBracket, "Expected ']' after list comprehension")?;
                        
                        return Ok(AstNode::ListComprehension {
                            expr: Box::new(first_expr),
                            var,
                            iterable: Box::new(iterable),
                            condition,
                        });
                    } else {
                        // Regular list - add the first expression and continue
                        elements.push(first_expr);
                        
                        while self.match_token(&TokenType::Comma) {
                            // Check if this element is itself a list (for matrix)
                            if self.check(&TokenType::LeftBracket) {
                                // This might be a matrix row
                                let row_expr = self.expression()?;
                                elements.push(row_expr);
                            } else {
                                elements.push(self.expression()?);
                            }
                        }
                    }
                }
                self.consume(&TokenType::RightBracket, "Expected ']' after elements")?;
                
                // Determine if this is a matrix (all elements are lists of same length)
                let is_matrix = elements.len() > 0 && elements.iter().all(|elem| {
                    matches!(elem, AstNode::List(_))
                });
                
                if is_matrix {
                    // Convert to matrix
                    let mut matrix_rows = Vec::new();
                    for elem in elements {
                        if let AstNode::List(row) = elem {
                            matrix_rows.push(row);
                        }
                    }
                    Ok(AstNode::Matrix(matrix_rows))
                } else {
                    // Regular list
                    Ok(AstNode::List(elements))
                }
            }
            
            TokenType::LeftBrace => {
                // Dictionary literal: {key: value, key2: value2} or dict comprehension
                let mut pairs = Vec::new();
                if !self.check(&TokenType::RightBrace) {
                    self.match_token(&TokenType::Newline); // Allow newline after opening brace
                    let first_key = self.dict_key()?;
                    self.consume(&TokenType::Colon, "Expected ':' after dictionary key")?;
                    let first_value = self.expression()?;
                    
                    // Check for "for" keyword indicating comprehension
                    if self.match_token(&TokenType::For) {
                        // This is a dict comprehension: {key: value for var in iterable}
                        let var = match &self.advance().token_type {
                            TokenType::Identifier(name) => name.clone(),
                            _ => return Err("Expected variable name in dict comprehension".to_string()),
                        };
                        
                        self.consume(&TokenType::In, "Expected 'in' in dict comprehension")?;
                        let iterable = self.expression()?;
                        
                        // Optional condition with "if"
                        let condition = if self.match_token(&TokenType::If) {
                            Some(Box::new(self.expression()?))
                        } else {
                            None
                        };
                        
                        self.consume(&TokenType::RightBrace, "Expected '}' after dict comprehension")?;
                        
                        return Ok(AstNode::DictComprehension {
                            key_expr: Box::new(first_key),
                            value_expr: Box::new(first_value),
                            var,
                            iterable: Box::new(iterable),
                            condition,
                        });
                    } else {
                        // Regular dictionary - add the first pair and continue
                        pairs.push((first_key, first_value));
                        
                        // Allow comma OR newline as separator
                        while self.match_token(&TokenType::Comma) || self.match_token(&TokenType::Newline) {
                            // Skip any additional newlines
                            while self.match_token(&TokenType::Newline) {}
                            
                            // Check if we've reached the end
                            if self.check(&TokenType::RightBrace) {
                                break;
                            }
                            
                            let key = self.dict_key()?;
                            self.consume(&TokenType::Colon, "Expected ':' after dictionary key")?;
                            let value = self.expression()?;
                            pairs.push((key, value));
                        }
                    }
                }
                self.match_token(&TokenType::Newline); // Allow newline before closing brace
                self.consume(&TokenType::RightBrace, "Expected '}' after dictionary elements")?;
                Ok(AstNode::Dict(pairs))
            }
            
            _ => {
                Err("Unexpected token".to_string())
            }
        }
    }
    
    fn enum_declaration(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::Pipe, "Expected '|' after 'enum'")?;
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected enum name".to_string()),
        };

        let backing_type = if self.match_token(&TokenType::Colon) {
            let type_name = match &self.advance().token_type {
                TokenType::Identifier(name) => name.clone(),
                TokenType::Str => "str".to_string(),
                TokenType::Int => "int".to_string(),
                TokenType::FloatType => "float".to_string(),
                TokenType::Bool => "bool".to_string(),
                _ => return Err("Expected type name after ':' for enum backing type".to_string()),
            };
            Some(type_name)
        } else {
            None
        };
        
        self.consume(&TokenType::LeftBrace, "Expected '{' after enum name or backing type")?;
        
        let mut variants = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            
            let variant_name = match &self.advance().token_type {
                TokenType::Identifier(name) => name.clone(),
                _ => return Err("Expected variant name".to_string()),
            };
            
            let value = if self.match_token(&TokenType::Assign) {
                Some(self.expression()?)
            } else {
                None
            };
            
            variants.push((variant_name, value));
            
            if !self.check(&TokenType::RightBrace) {
                self.match_token(&TokenType::Comma); // Optional comma
                self.match_token(&TokenType::Newline); // Optional newline
            }
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after enum variants")?;
        
        Ok(AstNode::EnumDeclaration { name, backing_type, variants })
    }
    
    fn class_declaration(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::Pipe, "Expected '|' after 'class'")?;
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected class name".to_string()),
        };
        
        // Check for parent class: class | Child : Parent
        let parent = if self.match_token(&TokenType::Colon) {
            match &self.advance().token_type {
                TokenType::Identifier(parent_name) => Some(parent_name.clone()),
                _ => return Err("Expected parent class name after ':'".to_string()),
            }
        } else {
            None
        };
        
        // Check for traits: class | Name : Parent + Trait1 + Trait2
        let mut traits = Vec::new();
        while self.match_token(&TokenType::Plus) {
            match &self.advance().token_type {
                TokenType::Identifier(trait_name) => traits.push(trait_name.clone()),
                _ => return Err("Expected trait name after '+'".to_string()),
            }
        }
        
        self.consume(&TokenType::LeftBrace, "Expected '{' after class declaration")?;
        
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut static_fields = Vec::new();
        let mut static_methods = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            
            // Check for static keyword
            let is_static = self.match_token(&TokenType::Static);
            
            // Check for visibility modifiers
            let is_public = if self.match_token(&TokenType::Pub) {
                true
            } else {
                self.match_token(&TokenType::Priv);
                false
            };
            
            // Check if it's a method (fn) or field (type)
            if self.check(&TokenType::Fn) {
                self.advance(); // consume fn
                let method = self.function_declaration()?;
                if is_static {
                    static_methods.push(method);
                } else {
                    methods.push(method);
                }
            } else if self.is_type_token() {
                // Field declaration
                let field_type = self.advance().token_type.clone();
                let type_str = match field_type {
                    TokenType::Str => "str".to_string(),
                    TokenType::Int => "int".to_string(),
                    TokenType::FloatType => "float".to_string(),
                    TokenType::Bool => "bool".to_string(),
                    TokenType::List => "list".to_string(),
                    TokenType::Dict => "dict".to_string(),
                    TokenType::Identifier(name) => name,
                    _ => return Err("Expected type for field".to_string()),
                };
                
                self.consume(&TokenType::Pipe, "Expected '|' after field type")?;
                
                let field_name = match &self.advance().token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected field name".to_string()),
                };
                
                let default_value = if self.match_token(&TokenType::Arrow) {
                    Some(self.expression()?)
                } else {
                    None
                };
                
                let field = ClassField {
                    name: field_name,
                    field_type: type_str,
                    default_value,
                    is_public,
                    is_readonly: false,
                    is_static,
                };
                
                if is_static {
                    static_fields.push(field);
                } else {
                    fields.push(field);
                }
            } else {
                return Err("Expected field or method declaration in class".to_string());
            }
            
            self.match_token(&TokenType::Newline);
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after class body")?;
        
        Ok(AstNode::ClassDeclaration {
            name,
            parent,
            traits,
            fields,
            methods,
            static_fields,
            static_methods,
        })
    }
    
    fn lambda_statement(&mut self) -> Result<AstNode, String> {
        // Lambda function: fn | name > params { body }
        self.consume(&TokenType::Pipe, "Expected '|' after 'fn'")?;
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected function name in lambda".to_string()),
        };
        
        self.consume(&TokenType::Greater, "Expected '>' in lambda")?;
        
        let mut params = Vec::new();
        if !self.check(&TokenType::LeftBrace) {
            loop {
                let param = match &self.advance().token_type {
                    TokenType::Identifier(name) => name.clone(),
                    _ => return Err("Expected parameter name in lambda".to_string()),
                };
                params.push(param);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
                if self.check(&TokenType::LeftBrace) {
                    break;
                }
            }
        }
        
        let body = if self.check(&TokenType::LeftBrace) {
            self.block()?
        } else {
            self.expression()?
        };
        
        // Create a lambda and assign it to the variable
        let lambda = AstNode::Lambda {
            params,
            body: Box::new(body),
        };
        
        Ok(AstNode::Assignment {
            name,
            value: Box::new(lambda),
        })
    }
    
    fn pattern(&mut self) -> Result<Pattern, String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::Integer(i) => Ok(Pattern::Literal(AstNode::Integer(*i))),
            TokenType::Float(f) => Ok(Pattern::Literal(AstNode::Float(*f))),
            TokenType::String(s) => Ok(Pattern::Literal(AstNode::String(s.clone()))),
            TokenType::Boolean(b) => Ok(Pattern::Literal(AstNode::Boolean(*b))),
            TokenType::Underscore => Ok(Pattern::Wildcard),
            TokenType::Identifier(name) if name == "_" => Ok(Pattern::Wildcard),
            TokenType::Identifier(name) => Ok(Pattern::Identifier(name.clone())),
            _ => Err(format!("Invalid pattern: {:?} at line {}", token.token_type, token.line)),
        }
    }
    
    fn block(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            statements.push(self.statement()?);
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}'")?;
        Ok(AstNode::Block(statements))
    }
    
    // Helper methods
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn check_ahead(&self, token_type: &TokenType) -> bool {
        if self.current + 1 >= self.tokens.len() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current + 1].token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn dict_key(&mut self) -> Result<AstNode, String> {
        let key_token = self.advance().clone(); // Consume the key token first
        match &key_token.token_type {
            TokenType::Identifier(name) => {
                Ok(AstNode::String(name.clone()))
            }
            TokenType::String(s) => {
                Ok(AstNode::String(s.clone()))
            }
            _ => {
                Err("Dictionary keys must be identifiers or string literals".to_string())
            }
        }
    }
    
    fn check_ahead_n(&self, n: usize, token_type: &TokenType) -> bool {
        if self.current + n >= self.tokens.len() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current + n].token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn is_indexed_for_loop(&self) -> bool {
        // Check for pattern: ( identifier , identifier ) in
        if !self.check(&TokenType::LeftParen) {
            return false;
        }
        
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        
        // Check for identifier after (
        if !matches!(self.tokens[self.current + 1].token_type, TokenType::Identifier(_)) {
            return false;
        }
        
        if self.current + 2 >= self.tokens.len() {
            return false;
        }
        
        // Check for comma
        if self.tokens[self.current + 2].token_type != TokenType::Comma {
            return false;
        }
        
        if self.current + 3 >= self.tokens.len() {
            return false;
        }
        
        // Check for second identifier
        if !matches!(self.tokens[self.current + 3].token_type, TokenType::Identifier(_)) {
            return false;
        }
        
        if self.current + 4 >= self.tokens.len() {
            return false;
        }
        
        // Check for )
        if self.tokens[self.current + 4].token_type != TokenType::RightParen {
            return false;
        }
        
        if self.current + 5 >= self.tokens.len() {
            return false;
        }
        
        // Check for in
        if self.tokens[self.current + 5].token_type != TokenType::In {
            return false;
        }
        
        true
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume(&mut self, token_type: &TokenType, _message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let current_token = self.peek();
            let expected = format!("{:?}", token_type);
            let got = format!("{:?}", current_token.token_type);
            Err(JError::unexpected_token(&expected, &got, current_token.line, current_token.column).to_string())
        }
    }
    
    fn is_type_token(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::Str | TokenType::Int | TokenType::FloatType | TokenType::Bool |
            TokenType::List | TokenType::Dict | TokenType::Tuple | TokenType::Vec | TokenType::Mat |
            TokenType::Vec3 | TokenType::Vec4 | TokenType::Mat2 | TokenType::Mat3 | TokenType::Mat4 |
            TokenType::Set | TokenType::Counter | TokenType::Deque | TokenType::PriorityQ | TokenType::Graph | TokenType::Tree | TokenType::Grid |
            TokenType::CharType | TokenType::EmojiType | TokenType::Ascii | TokenType::MoneyType |
            TokenType::HexType | TokenType::DateType | TokenType::TimeType | TokenType::DateTimeType |
            TokenType::Any | TokenType::Expr | TokenType::Exclamation
        )
    }
    
    fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::Equal) {
            Some(BinaryOp::Equal)
        } else if self.match_token(&TokenType::NotEqual) {
            Some(BinaryOp::NotEqual)
        } else {
            None
        }
    }
    
    fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::Greater) {
            Some(BinaryOp::Greater)
        } else if self.match_token(&TokenType::GreaterEqual) {
            Some(BinaryOp::GreaterEqual)
        } else if self.match_token(&TokenType::Less) {
            Some(BinaryOp::Less)
        } else if self.match_token(&TokenType::LessEqual) {
            Some(BinaryOp::LessEqual)
        } else {
            None
        }
    }
    
    fn match_term_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::Minus) {
            Some(BinaryOp::Subtract)
        } else if self.match_token(&TokenType::Plus) {
            Some(BinaryOp::Add)
        } else {
            None
        }
    }
    
    fn match_factor_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(&TokenType::Divide) {
            Some(BinaryOp::Divide)
        } else if self.match_token(&TokenType::Multiply) {
            Some(BinaryOp::Multiply)
        } else if self.match_token(&TokenType::Modulo) {
            Some(BinaryOp::Modulo)
        } else if self.match_token(&TokenType::Power) {
            Some(BinaryOp::Power)
        } else {
            None
        }
    }
    
    fn match_unary_op(&mut self) -> Option<UnaryOp> {
        if self.match_token(&TokenType::Not) {
            Some(UnaryOp::Not)
        } else if self.match_token(&TokenType::Minus) {
            Some(UnaryOp::Minus)
        } else {
            None
        }
    }
    
    fn parse_string_interpolation(&mut self, s: &str) -> Result<AstNode, String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut chars = s.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '$' {
                // Add current string part if not empty
                if !current.is_empty() {
                    parts.push(AstNode::String(current.clone()));
                    current.clear();
                }
                
                // Parse variable name
                let mut var_name = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                if !var_name.is_empty() {
                    parts.push(AstNode::Identifier(var_name));
                }
            } else if ch == '{' {
                // Add current string part if not empty
                if !current.is_empty() {
                    parts.push(AstNode::String(current.clone()));
                    current.clear();
                }
                
                // Parse expression inside braces
                let mut expr = String::new();
                let mut brace_count = 1;
                
                while let Some(next_ch) = chars.next() {
                    if next_ch == '{' {
                        brace_count += 1;
                    } else if next_ch == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                    }
                    expr.push(next_ch);
                }
                
                if !expr.is_empty() {
                    // Parse the expression
                    let mut expr_lexer = crate::lexer::Lexer::new(&expr);
                    let expr_tokens = expr_lexer.tokenize().map_err(|e| format!("Error tokenizing interpolated expression: {}", e))?;
                    let mut expr_parser = Parser::new(expr_tokens);
                    let expr_ast = expr_parser.expression()?;
                    parts.push(expr_ast);
                }
            } else {
                current.push(ch);
            }
        }
        
        // Add final string part if not empty
        if !current.is_empty() {
            parts.push(AstNode::String(current));
        }
        
        Ok(AstNode::StringInterpolation { parts })
    }
}