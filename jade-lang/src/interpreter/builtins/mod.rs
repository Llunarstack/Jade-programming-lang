//! Built-in functions grouped by category: math, algo, DSA, variables, string, io, numeric, enum, counter, crypto.

mod core;
mod math;
mod algo;
mod dsa;
mod variables;
mod string;
mod io;
mod numeric;
mod stats;
mod enum_builtins;
mod counter;
mod bits;
mod uf;
mod trie;
mod memo;
mod random;
mod crypto;
#[cfg(feature = "regex")]
pub(super) mod regex_builtins;

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

/// Try to dispatch a builtin by name. Returns `Ok(Some(value))` if handled, `Ok(None)` if not.
/// Dispatches via a loop to avoid deep call stacks (stack overflow on Windows with many modules).
pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    type TryFn = fn(&mut Interpreter, &str, &[AstNode]) -> Result<Option<Value>, String>;
    let modules: &[TryFn] = &[
        core::try_call,
        math::try_call,
        algo::try_call,
        dsa::try_call,
        variables::try_call,
        string::try_call,
        io::try_call,
        numeric::try_call,
        stats::try_call,
        enum_builtins::try_call,
        counter::try_call,
        bits::try_call,
        uf::try_call,
        trie::try_call,
        memo::try_call,
        random::try_call,
        crypto::try_call,
    ];
    for try_fn in modules {
        if let Some(v) = try_fn(interpreter, name, args)? {
            return Ok(Some(v));
        }
    }
    #[cfg(feature = "regex")]
    if let Some(v) = regex_builtins::try_call(interpreter, name, args)? {
        return Ok(Some(v));
    }
    Ok(None)
}
