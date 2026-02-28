//! Eval section: primitives and literals — Integer, Float, String, collections, Identifier.

use std::collections::HashMap;
use crate::parser::AstNode;
use super::*;

/// Evaluate primitive literals and identifier lookup.
pub(super) fn eval_primitive(interpreter: &mut Interpreter, node: &AstNode) -> Result<Value, String> {
    match node {
            AstNode::Integer(i) => Ok(Value::Integer(*i)),
            AstNode::Float(f) => Ok(Value::Float(*f)),
            AstNode::String(s) => Ok(Value::String(s.clone())),
            AstNode::StringInterpolation { parts } => {
                let mut result = String::new();
                for part in parts {
                    let val = interpreter.eval_node(part)?;
                    result.push_str(&val.to_string());
                }
                Ok(Value::String(result))
            }
            AstNode::Boolean(b) => Ok(Value::Boolean(*b)),
            AstNode::Char(c) => Ok(Value::Char(*c)),

            AstNode::Infinity(positive) => Ok(Value::Infinity(*positive)),
            AstNode::Emoji(e) => Ok(Value::Emoji(e.clone())),
            AstNode::Money(symbol, amount) => Ok(Value::Money(symbol.clone(), *amount)),
            AstNode::Hex(hex) => Ok(Value::Hex(hex.clone())),
            AstNode::Date(date) => Ok(Value::Date(date.clone())),
            AstNode::Time(time) => Ok(Value::Time(time.clone())),
            AstNode::DateTime(datetime) => Ok(Value::DateTime(datetime.clone())),

            AstNode::Tuple(elements) => {
                let mut tuple = Vec::new();
                for element in elements {
                    tuple.push(interpreter.eval_node(element)?);
                }
                Ok(Value::Tuple(tuple))
            }

            AstNode::List(elements) => {
                let mut list = Vec::new();
                for element in elements {
                    list.push(interpreter.eval_node(element)?);
                }
                Ok(Value::List(list))
            }

            AstNode::Vector(elements) => {
                let mut vector = Vec::new();
                for element in elements {
                    let val = interpreter.eval_node(element)?;
                    match val {
                        Value::Integer(i) => vector.push(i as f64),
                        Value::Float(f) => vector.push(f),
                        _ => return Err("Vector elements must be numeric".to_string()),
                    }
                }
                Ok(Value::Vector(vector))
            }

            AstNode::Matrix(rows) => {
                let mut matrix = Vec::new();
                for row in rows {
                    let mut matrix_row = Vec::new();
                    for element in row {
                        let val = interpreter.eval_node(element)?;
                        match val {
                            Value::Integer(i) => matrix_row.push(i as f64),
                            Value::Float(f) => matrix_row.push(f),
                            _ => return Err("Matrix elements must be numeric".to_string()),
                        }
                    }
                    matrix.push(matrix_row);
                }
                interpreter.validate_matrix(&matrix)?;
                Ok(Value::Matrix(matrix))
            }

            AstNode::Dict(pairs) => {
                let mut dict = HashMap::new();
                for (key_node, value_node) in pairs {
                    let key = match interpreter.eval_node(key_node)? {
                        Value::String(s) => s,
                        Value::Integer(i) => i.to_string(),
                        _ => return Err("Dictionary keys must be strings or integers".to_string()),
                    };
                    let value = interpreter.eval_node(value_node)?;
                    dict.insert(key, value);
                }
                Ok(Value::Dict(dict))
            }

            AstNode::Identifier(name) => interpreter.get_variable(name),

            _ => Err("eval_primitive: expected primitive or identifier".to_string()),
        }
}
