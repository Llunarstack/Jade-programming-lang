//! Evaluation of conditional AST nodes: if, match, cond, when, unless, either, guard, switch.

use crate::parser::{AstNode, CondBranch, MatchArm, SwitchCase};
use super::*;

impl Interpreter {
    pub(super) fn eval_if(
        &mut self,
        condition: &AstNode,
        then_branch: &AstNode,
        else_branch: &Option<Box<AstNode>>,
    ) -> Result<Value, String> {
        let cond_val = self.eval_node(condition)?;
        if self.is_truthy(&cond_val) {
            self.eval_node(then_branch)
        } else if let Some(else_node) = else_branch {
            self.eval_node(else_node)
        } else {
            Ok(Value::None)
        }
    }

    pub(super) fn eval_match(&mut self, expr: &AstNode, arms: &[MatchArm]) -> Result<Value, String> {
        let expr_val = self.eval_node(expr)?;
        for arm in arms {
            if self.pattern_matches(&arm.pattern, &expr_val)? {
                if let Some(guard) = &arm.guard {
                    let guard_val = self.eval_node(guard)?;
                    if !self.is_truthy(&guard_val) {
                        continue;
                    }
                }
                return self.eval_node(&arm.body);
            }
        }
        Err("No matching pattern found".to_string())
    }

    pub(super) fn eval_cond(
        &mut self,
        value: &AstNode,
        branches: &[CondBranch],
    ) -> Result<Value, String> {
        let cond_value = self.eval_node(value)?;
        self.set_variable("_".to_string(), cond_value.clone());
        for branch in branches {
            if branch.is_else {
                return self.eval_node(&branch.body);
            }
            let condition_result = self.eval_node(&branch.condition)?;
            if self.is_truthy(&condition_result) {
                return self.eval_node(&branch.body);
            }
        }
        Ok(Value::None)
    }

    pub(super) fn eval_when(
        &mut self,
        value: &AstNode,
        branches: &[CondBranch],
    ) -> Result<Value, String> {
        let cond_value = self.eval_node(value)?;
        self.set_variable("_".to_string(), cond_value.clone());
        for branch in branches {
            if branch.is_else {
                return self.eval_node(&branch.body);
            }
            let condition_result = self.eval_node(&branch.condition)?;
            if self.is_truthy(&condition_result) {
                return self.eval_node(&branch.body);
            }
        }
        Ok(Value::None)
    }

    pub(super) fn eval_unless(
        &mut self,
        condition: &AstNode,
        then_branch: &AstNode,
        else_branch: &Option<Box<AstNode>>,
    ) -> Result<Value, String> {
        let cond_val = self.eval_node(condition)?;
        if self.is_truthy(&cond_val) {
            self.eval_node(then_branch)
        } else if let Some(else_node) = else_branch {
            self.eval_node(else_node)
        } else {
            Ok(Value::None)
        }
    }

    pub(super) fn eval_either(
        &mut self,
        expr: &AstNode,
        true_body: &AstNode,
        false_body: &AstNode,
    ) -> Result<Value, String> {
        let val = self.eval_node(expr)?;
        if self.is_truthy(&val) {
            self.eval_node(true_body)
        } else {
            self.eval_node(false_body)
        }
    }

    pub(super) fn eval_guard_return(
        &mut self,
        condition: &AstNode,
        return_value: &AstNode,
    ) -> Result<Value, String> {
        let cond_val = self.eval_node(condition)?;
        if !self.is_truthy(&cond_val) {
            return self.eval_node(return_value);
        }
        Ok(Value::None)
    }

    pub(super) fn eval_switch(
        &mut self,
        expr: &AstNode,
        cases: &[(SwitchCase, AstNode)],
    ) -> Result<Value, String> {
        let val = self.eval_node(expr)?;
        for (case, body) in cases {
            let matches = match case {
                SwitchCase::Else => true,
                SwitchCase::Literal(lit) => {
                    let lit_val = self.eval_node(lit)?;
                    self.values_equal(&lit_val, &val)
                }
                SwitchCase::Range { start, end } => {
                    let start_val = self.eval_node(start)?;
                    let end_val = self.eval_node(end)?;
                    let to_f = |v: &Value| -> Option<f64> {
                        match v {
                            Value::Integer(i) => Some(*i as f64),
                            Value::Float(f) => Some(*f),
                            _ => None,
                        }
                    };
                    match (to_f(&val), to_f(&start_val), to_f(&end_val)) {
                        (Some(v), Some(s), Some(e)) => v >= s && v <= e,
                        _ => false,
                    }
                }
            };
            if matches {
                return self.eval_node(body);
            }
        }
        Ok(Value::None)
    }
}
