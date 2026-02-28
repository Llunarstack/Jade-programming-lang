//! Evaluation of control-flow AST nodes: block, defer, assignment, return, break, continue, try/catch, throw.

use crate::parser::AstNode;
use super::*;

impl Interpreter {
    pub(super) fn eval_block(&mut self, statements: &[AstNode]) -> Result<Value, String> {
        self.push_scope();
        self.defer_stack.push(Vec::new());

        let result = (|| {
            let mut last_val = Value::None;
            for stmt in statements {
                if let AstNode::Defer(expr) = stmt {
                    if let Some(defer_vec) = self.defer_stack.last_mut() {
                        defer_vec.push((*expr.clone(), None));
                    }
                } else {
                    last_val = self.eval_node(stmt)?;
                }
            }
            Ok(last_val)
        })();

        let defers = self.defer_stack.pop().unwrap_or_default();
        for (expr, value_for_underscore) in defers.into_iter().rev() {
            if let Some(v) = value_for_underscore {
                self.set_variable("_".to_string(), v);
            }
            let _ = self.eval_node(&expr);
        }

        self.pop_scope();
        result
    }

    pub(super) fn eval_assignment(&mut self, name: &str, value: &AstNode) -> Result<Value, String> {
        let val = self.eval_node(value)?;
        self.assign_variable(name, val.clone())?;
        Ok(val)
    }

    pub(super) fn eval_destructuring_assignment(
        &mut self,
        targets: &[String],
        value: &AstNode,
    ) -> Result<Value, String> {
        let val = self.eval_node(value)?;
        match val {
            Value::Tuple(tuple) => {
                if targets.len() != tuple.len() {
                    return Err(format!(
                        "Cannot destructure tuple of length {} into {} variables",
                        tuple.len(),
                        targets.len()
                    ));
                }
                for (target, value) in targets.iter().zip(tuple.iter()) {
                    self.set_variable(target.clone(), value.clone());
                }
                Ok(Value::Tuple(tuple))
            }
            Value::List(list) => {
                if targets.len() > list.len() {
                    return Err(format!(
                        "Cannot destructure list of length {} into {} variables",
                        list.len(),
                        targets.len()
                    ));
                }
                for (i, target) in targets.iter().enumerate() {
                    if i < list.len() {
                        self.set_variable(target.clone(), list[i].clone());
                    } else {
                        self.set_variable(target.clone(), Value::None);
                    }
                }
                Ok(Value::List(list))
            }
            _ => Err("Can only destructure tuples and lists".to_string()),
        }
    }

    pub(super) fn eval_return(&mut self, expr: &Option<Box<AstNode>>) -> Result<Value, String> {
        if let Some(expr) = expr {
            self.eval_node(expr)
        } else {
            Ok(Value::None)
        }
    }

    pub(super) fn eval_try_catch(
        &mut self,
        try_block: &AstNode,
        catch_var: &Option<String>,
        catch_block: &AstNode,
        finally_block: &Option<Box<AstNode>>,
    ) -> Result<Value, String> {
        let result = self.eval_node(try_block);

        let final_result = match result {
            Err(error_msg) => {
                if let Some(var_name) = catch_var {
                    self.set_variable(var_name.clone(), Value::String(error_msg));
                }
                self.eval_node(catch_block)?
            }
            Ok(val) => val,
        };

        if let Some(finally) = finally_block {
            self.eval_node(finally)?;
        }

        Ok(final_result)
    }

    pub(super) fn eval_throw(&mut self, expr: &AstNode) -> Result<Value, String> {
        let error_val = self.eval_node(expr)?;
        let error_msg = match error_val {
            Value::String(s) => s,
            other => format!("{}", other),
        };
        Err(error_msg)
    }
}
