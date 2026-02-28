//! Eval section: output — out(), print_table, print_progress_bar, print_animation, print_gradient, format_string.

use std::collections::HashMap;
use crate::parser::AstNode;
use super::*;

impl Interpreter {
    /// Main `out(...)` builtin: printing, tables, progress bar, animation, gradient, formatting.
    pub(super) fn call_out(&mut self, args: &[AstNode]) -> Result<Value, String> {
        if args.is_empty() {
            self.write_out_ln("");
            return Ok(Value::None);
        }

        if args.len() == 1 {
            let val = self.eval_node(&args[0])?;
            if let Value::List(rows) = &val {
                if !rows.is_empty() {
                    if let Value::List(_) = &rows[0] {
                        self.print_table(rows)?;
                        return Ok(Value::None);
                    }
                }
            }
            self.write_out_ln(&val.to_string());
        } else if args.len() == 2 {
            let first_val = self.eval_node(&args[0])?;
            let second_val = self.eval_node(&args[1])?;

            match second_val {
                Value::Dict(options) => {
                    if let Some(Value::Float(percent)) = options.get("progress") {
                        let width = options.get("width").and_then(|v| if let Value::Integer(i) = v { Some(*i as usize) } else { None }).unwrap_or(40);
                        let color = options.get("color").and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None }).unwrap_or_else(|| "green".to_string());
                        self.print_progress_bar(*percent, width, &color)?;
                        return Ok(Value::None);
                    }
                    if let Some(Value::String(anim_type)) = options.get("animate") {
                        let interval = options.get("interval").and_then(|v| if let Value::Float(f) = v { Some(*f) } else { None }).unwrap_or(0.15);
                        let count = options.get("count").and_then(|v| if let Value::Integer(i) = v { Some(*i as usize) } else { None });
                        let text = first_val.to_string();
                        self.print_animation(&text, anim_type, interval, count)?;
                        return Ok(Value::None);
                    }
                    if let Some(Value::List(gradient_colors)) = options.get("gradient") {
                        let text = first_val.to_string();
                        self.print_gradient(&text, gradient_colors)?;
                        return Ok(Value::None);
                    }
                    if let Value::List(rows) = &first_val {
                        if !rows.is_empty() {
                            if let Value::List(_) = &rows[0] {
                                self.print_table_with_options(rows, &options)?;
                                return Ok(Value::None);
                            }
                        }
                    }
                    let mut output = first_val.to_string();
                    if let Value::String(format_str) = &first_val {
                        if format_str.contains('{') {
                            output = self.format_string(format_str, &options)?;
                        }
                    }
                    let mut formatted_output = String::new();
                    let mut has_color = false;
                    let mut has_style = false;
                    if let Some(Value::String(color)) = options.get("color") {
                        match color.as_str() {
                            "red" => { formatted_output.push_str("\x1b[31m"); has_color = true; }
                            "green" => { formatted_output.push_str("\x1b[32m"); has_color = true; }
                            "yellow" => { formatted_output.push_str("\x1b[33m"); has_color = true; }
                            "blue" => { formatted_output.push_str("\x1b[34m"); has_color = true; }
                            "magenta" => { formatted_output.push_str("\x1b[35m"); has_color = true; }
                            "cyan" => { formatted_output.push_str("\x1b[36m"); has_color = true; }
                            "white" => { formatted_output.push_str("\x1b[37m"); has_color = true; }
                            _ => {}
                        }
                    }
                    if let Some(Value::String(style)) = options.get("style") {
                        match style.as_str() {
                            "bold" => { formatted_output.push_str("\x1b[1m"); has_style = true; }
                            "dim" => { formatted_output.push_str("\x1b[2m"); has_style = true; }
                            "underline" => { formatted_output.push_str("\x1b[4m"); has_style = true; }
                            "blink" => { formatted_output.push_str("\x1b[5m"); has_style = true; }
                            _ => {}
                        }
                    }
                    formatted_output.push_str(&output);
                    if has_color || has_style {
                        formatted_output.push_str("\x1b[0m");
                    }
                    let end = options.get("end").and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None }).unwrap_or_else(|| "\n".to_string());
                    self.write_out(&format!("{}{}", formatted_output, end));
                }
                _ => {
                    self.write_out_ln(&format!("{} {}", first_val, second_val));
                }
            }
        } else {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    self.write_out(" ");
                }
                let val = self.eval_node(arg)?;
                self.write_out(&val.to_string());
            }
            self.write_out_ln("");
        }
        Ok(Value::None)
    }

    pub(super) fn format_string(
        &self,
        format_str: &str,
        vars: &HashMap<String, Value>,
    ) -> Result<String, String> {
        let mut result = String::new();
        let mut i = 0;
        let chars: Vec<char> = format_str.chars().collect();

        while i < chars.len() {
            if chars[i] == '{' && i + 1 < chars.len() {
                if chars[i + 1] == '}' {
                    i += 2;
                    continue;
                }
                let mut j = i + 1;
                let mut found = false;
                while j < chars.len() {
                    if chars[j] == '}' {
                        found = true;
                        break;
                    }
                    j += 1;
                }
                if found {
                    let var_name: String = chars[i + 1..j].iter().collect();
                    let parts: Vec<&str> = var_name.split(':').collect();
                    let key = parts[0].trim();
                    let formatted = if let Some(value) = vars.get(key) {
                        if parts.len() > 1 {
                            let spec = parts[1].trim();
                            if let Some(stripped) = spec.strip_prefix('>') {
                                let width: usize = stripped.parse().unwrap_or(0);
                                format!("{:>width$}", value, width = width)
                            } else if let Some(stripped) = spec.strip_prefix('<') {
                                let width: usize = stripped.parse().unwrap_or(0);
                                format!("{:<width$}", value, width = width)
                            } else if spec.starts_with(".2f") || spec.starts_with(".1f") {
                                if let Value::Float(f) = value {
                                    format!("{:.2}", f)
                                } else {
                                    value.to_string()
                                }
                            } else {
                                value.to_string()
                            }
                        } else {
                            value.to_string()
                        }
                    } else {
                        match self.get_variable(key) {
                            Ok(val) => val.to_string(),
                            Err(_) => format!("{{{}}}", key),
                        }
                    };
                    result.push_str(&formatted);
                    i = j + 1;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        Ok(result)
    }

    pub(super) fn print_table(&mut self, rows: &[Value]) -> Result<(), String> {
        let mut table: Vec<Vec<String>> = Vec::new();
        for row in rows {
            if let Value::List(cells) = row {
                table.push(cells.iter().map(|c| c.to_string()).collect());
            } else {
                return Err("Table rows must be lists".to_string());
            }
        }
        if table.is_empty() {
            return Ok(());
        }
        let num_cols = table[0].len();
        let mut col_widths = vec![0; num_cols];
        for row in &table {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }
        for (row_idx, row) in table.iter().enumerate() {
            let mut line = String::new();
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx < col_widths.len() {
                    line.push_str(&format!("{:width$}", cell, width = col_widths[col_idx]));
                    if col_idx < row.len() - 1 {
                        line.push_str("  ");
                    }
                }
            }
            self.write_out_ln(&line);
            if row_idx == 0 {
                let mut sep = String::new();
                for (ii, &width) in col_widths.iter().enumerate() {
                    sep.push_str(&"-".repeat(width));
                    if ii < col_widths.len() {
                        sep.push_str("  ");
                    }
                }
                self.write_out_ln(&sep);
            }
        }
        Ok(())
    }

    pub(super) fn print_table_with_options(
        &mut self,
        rows: &[Value],
        options: &HashMap<String, Value>,
    ) -> Result<(), String> {
        let mut table: Vec<Vec<String>> = Vec::new();
        for row in rows {
            if let Value::List(cells) = row {
                table.push(cells.iter().map(|c| c.to_string()).collect());
            } else {
                return Err("Table rows must be lists".to_string());
            }
        }
        if table.is_empty() {
            return Ok(());
        }
        let num_cols = table[0].len();
        let mut col_widths = vec![0; num_cols];
        for row in &table {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }
        let align = options.get("align").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None }).unwrap_or("left");
        let color = options.get("color").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None });
        let header = options.get("header").and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None }).unwrap_or(false);

        if let Some(color_name) = color {
            match color_name {
                "red" => self.write_out("\x1b[31m"),
                "green" => self.write_out("\x1b[32m"),
                "yellow" => self.write_out("\x1b[33m"),
                "blue" => self.write_out("\x1b[34m"),
                "magenta" => self.write_out("\x1b[35m"),
                "cyan" => self.write_out("\x1b[36m"),
                "white" => self.write_out("\x1b[37m"),
                _ => {}
            }
        }
        for (row_idx, row) in table.iter().enumerate() {
            let mut line = String::new();
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx < col_widths.len() {
                    match align {
                        "right" => line.push_str(&format!("{:>width$}", cell, width = col_widths[col_idx])),
                        "center" => {
                            let padding = col_widths[col_idx].saturating_sub(cell.len());
                            let left_pad = padding / 2;
                            let right_pad = padding - left_pad;
                            line.push_str(&format!("{}{}{}", " ".repeat(left_pad), cell, " ".repeat(right_pad)));
                        }
                        _ => line.push_str(&format!("{:<width$}", cell, width = col_widths[col_idx])),
                    }
                    if col_idx < row.len() - 1 {
                        line.push_str("  ");
                    }
                }
            }
            self.write_out_ln(&line);
            if header && row_idx == 0 {
                let mut sep = String::new();
                for (ii, &width) in col_widths.iter().enumerate() {
                    sep.push_str(&"-".repeat(width));
                    if ii < col_widths.len() {
                        sep.push_str("  ");
                    }
                }
                self.write_out_ln(&sep);
            }
        }
        if color.is_some() {
            self.write_out("\x1b[0m");
        }
        Ok(())
    }

    pub(super) fn print_progress_bar(&mut self, percent: f64, width: usize, color: &str) -> Result<(), String> {
        let clamped_percent = percent.clamp(0.0, 100.0);
        let filled = ((clamped_percent / 100.0) * width as f64) as usize;
        let empty = width.saturating_sub(filled);

        match color {
            "red" => self.write_out("\x1b[31m"),
            "green" => self.write_out("\x1b[32m"),
            "yellow" => self.write_out("\x1b[33m"),
            "blue" => self.write_out("\x1b[34m"),
            "magenta" => self.write_out("\x1b[35m"),
            "cyan" => self.write_out("\x1b[36m"),
            "white" => self.write_out("\x1b[37m"),
            _ => {}
        }
        let mut bar = "[".to_string();
        for _ in 0..filled {
            bar.push('\u{2588}');
        }
        if filled < width {
            if (clamped_percent / 100.0 * width as f64) - filled as f64 > 0.5 {
                bar.push('\u{258C}');
                for _ in 0..empty.saturating_sub(1) {
                    bar.push(' ');
                }
            } else {
                for _ in 0..empty {
                    bar.push(' ');
                }
            }
        }
        bar.push_str(&format!("] {:.0}%", clamped_percent));
        self.write_out(&bar);
        self.write_out("\x1b[0m");
        self.write_out_ln("");
        Ok(())
    }

    pub(super) fn print_animation(
        &mut self,
        text: &str,
        anim_type: &str,
        interval: f64,
        count: Option<usize>,
    ) -> Result<(), String> {
        let frames: Vec<&str> = match anim_type {
            "spinner" => vec!["|", "/", "-", "\\"],
            "dots" => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            "bar" | "bounce" | "marquee" | "pulse" => return Err(format!("Animation type '{}' not fully implemented", anim_type)),
            _ => return Err(format!("Unknown animation type: {}", anim_type)),
        };
        let max_iterations = count.unwrap_or(10);
        let mut iteration = 0;
        while iteration < max_iterations {
            for frame in &frames {
                self.write_out(&format!("\r{} {}", text, frame));
                std::io::Write::flush(&mut std::io::stdout()).ok();
                std::thread::sleep(std::time::Duration::from_secs_f64(interval));
                iteration += 1;
                if iteration >= max_iterations {
                    break;
                }
            }
        }
        self.write_out_ln("");
        Ok(())
    }

    pub(super) fn print_gradient(&mut self, text: &str, gradient_colors: &[Value]) -> Result<(), String> {
        if gradient_colors.len() < 2 {
            return Err("Gradient requires at least 2 colors".to_string());
        }
        let mut colors: Vec<(u8, u8, u8)> = Vec::new();
        for color_val in gradient_colors {
            if let Value::String(hex_str) = color_val {
                let hex = hex_str.trim_start_matches('#');
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        colors.push((r, g, b));
                    }
                }
            }
        }
        if colors.len() < 2 {
            return Err("Gradient colors must be hex strings".to_string());
        }
        let chars: Vec<char> = text.chars().collect();
        for (i, ch) in chars.iter().enumerate() {
            let t = if chars.len() > 1 { i as f64 / (chars.len() - 1) as f64 } else { 0.0 };
            let segment_size = 1.0 / (colors.len() - 1) as f64;
            let segment = (t / segment_size).min((colors.len() - 2) as f64) as usize;
            let local_t = (t - segment as f64 * segment_size) / segment_size;
            let (r1, g1, b1) = colors[segment];
            let (r2, g2, b2) = colors[segment + 1];
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * local_t) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * local_t) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * local_t) as u8;
            self.write_out(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch));
        }
        self.write_out("\x1b[0m");
        self.write_out_ln("");
        Ok(())
    }
}
