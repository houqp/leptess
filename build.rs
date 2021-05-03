extern crate anyhow;
use anyhow::{Context, Result};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const VARIABLES_FILE: &str = "set_variables_list.txt";
const RAW_VARIABLES_FILE: &str = include_str!("set_variables_list.txt");

// Convert inputs like chop_x_y_weight to ChopXyWeight.
// This makes them suitable for use as enum variant names.
fn to_variant_case(input: &str) -> String {
    let mut in_acronym = false;
    let mut capitalize_next = true;
    let mut output = String::new();

    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '_' => {
                capitalize_next = true;
            }
            _ => {
                if capitalize_next {
                    // Clippy doesn't like acronyms to have subsequent characters capitalized.
                    if !in_acronym {
                        output.extend(c.to_uppercase());
                        in_acronym = true;
                    // But we should capitalize the first word after the acronym finishes
                    } else if iter.peek() != Some(&'_') && iter.peek().is_some() {
                        output.extend(c.to_uppercase());
                        in_acronym = false;
                    } else {
                        output.push(c);
                        in_acronym = true;
                    }
                } else {
                    in_acronym = false;
                    output.push(c);
                }
                capitalize_next = false;
            }
        }
    }

    output
}

fn escape_markdown(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.'
            | '!' => {
                output.push('\\');
            }
            _ => {}
        }
        output.push(c);
    }
    output
}

fn write_variant(
    line: &'static str,
    encountered_variables: &mut HashMap<&'static str, String>,
    stream: &mut impl Write,
) -> Result<()> {
    let mut split = line.split('\t');
    let variable_name = split
        .next()
        .context("Expected variable to be defined in the line")?;
    let example_value = split
        .next()
        .context("Expected example value to be defined in the line")?;
    let comment = split
        .next()
        .context("Expected comment to be defined in the line")?;
    if let Entry::Vacant(vacant_entry) = encountered_variables.entry(variable_name) {
        writeln!(stream, "    /// {}", escape_markdown(comment))
            .context("Expected to write comment")?;
        if !example_value.is_empty() {
            writeln!(
                stream,
                "    ///\n    /// Example value: `{:?}`",
                example_value
            )
            .context("Expected to write example value")?;
        }
        let variant_name = to_variant_case(variable_name);
        writeln!(stream, "    {},", variant_name).context("Expected to write variant")?;
        vacant_entry.insert(variant_name);
    }
    Ok(())
}

// To view the output run
// ```
// cargo test
// find target/debug/build/leptess-*/out -name variable.rs
// ```
// And open the file.
fn create_variable_rs_file() -> Result<()> {
    println!("cargo:rerun-if-changed={}", VARIABLES_FILE);

    let out_dir = std::env::var("OUT_DIR").context("Couldn't read OUT_DIR")?;
    let mut file_name = Path::new(&out_dir).to_path_buf();
    file_name.push("variable.rs");

    let file = File::create(&file_name)
        .with_context(|| format!("Couldn't create file {}", file_name.to_string_lossy()))?;
    let mut stream = BufWriter::new(file);

    writeln!(
        stream,
        r#"/// Enum representing different variable options accepted by Tesseract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variable {{"#
    )
    .context("Expected to write header")?;

    // Key - variable name
    // Value - enum variant name
    let mut encountered_variables = HashMap::new();

    for (line, line_no) in RAW_VARIABLES_FILE.lines().skip(1).zip(2..) {
        write_variant(line, &mut encountered_variables, &mut stream)
            .with_context(|| format!("Error on line {} of {}", line_no, VARIABLES_FILE))?;
    }

    writeln!(
        stream,
        r#"}}
    
impl Variable {{
    /// Get the variable's name as used by Tesseract
    pub fn as_cstr(&self) -> &'static std::ffi::CStr {{
        match self {{"#
    )
    .context("Expected to write middle")?;

    for (variable_name, variant_name) in encountered_variables {
        writeln!(
            stream,
            r#"            Variable::{} => std::ffi::CStr::from_bytes_with_nul(b"{}\0").unwrap(),"#,
            variant_name,
            variable_name.escape_debug().collect::<String>()
        )
        .with_context(|| format!("Failed to write match arm for {}", variable_name))?
    }

    writeln!(
        stream,
        r#"        }}
    }}
}}"#
    )
    .context("Expected to write footer")?;

    Ok(())
}

fn main() -> Result<()> {
    create_variable_rs_file().context("Error in `create_variable_rs_file`")?;
    Ok(())
}
