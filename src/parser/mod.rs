use regex::{Error as RegexError, Regex};
use std::{fs::File, io::Read, result::Result};
use thiserror::Error;

// Permits nodes to return values from function output
#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    None,
}

// Errors specific to the Oak language
#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] RegexError),
}

pub trait Node {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value;
}

// Evaluates math expressions in code
pub(crate) struct EvalMathExp {
    pub expr: String,
}

impl EvalMathExp {
    pub fn parse(expr_tokens: Vec<&str>) -> Self {
        Self {
            expr: expr_tokens.join(" "),
        }
    }
}

impl Node for EvalMathExp {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_eval_math_exp(self)
    }
}

// Parses binary operations
pub(crate) struct BinOp {
    pub left: Box<dyn Node>,
    pub op: String,
    pub right: Box<dyn Node>,
}

impl BinOp {
    pub fn parse(left: Box<dyn Node>, op: String, right: Box<dyn Node>) -> Self {
        Self { left, op, right }
    }
}

impl Node for BinOp {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_bin_op(self)
    }
}

// Evaluates and parse numerical types
pub(crate) struct Number {
    pub value: f64,
}

impl Number {
    pub fn parse(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Node for Number {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_number(self)
    }
}

// Parses variables
pub(crate) struct Var {
    pub name: String,
}

impl Var {
    pub fn parse(name: String) -> Self {
        Self { name }
    }
}

impl Node for Var {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_var(self)
    }
}

// Evaluates variables and functions assignation
pub(crate) struct Assign {
    pub name: String,
    pub expr: Box<dyn Node>,
}

impl Assign {
    pub fn parse(name: String, expr: Box<dyn Node>) -> Self {
        Self { name, expr }
    }
}

impl Node for Assign {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_assign(self)
    }
}

// Parses strings
pub(crate) struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_string_literal(self)
    }
}

// Parses function calls
pub(crate) struct FunctionCall {
    pub name: String,
    pub args: Vec<Box<dyn Node>>,
}

impl FunctionCall {
    pub fn parse(name: String, args: Vec<Box<dyn Node>>) -> Self {
        Self { name, args }
    }
}

impl Node for FunctionCall {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_function_call(self)
    }
}

// Non-evaluative comment parsing
pub(crate) struct Comment {
    pub value: String,
}

impl Comment {
    pub fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for Comment {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_comment(self)
    }
}

// Visitor trait pattern for each node
pub trait Visitor {
    fn visit_eval_math_exp(&mut self, node: &EvalMathExp) -> Value;
    fn visit_bin_op(&mut self, node: &BinOp) -> Value;
    fn visit_number(&mut self, node: &Number) -> Value;
    fn visit_var(&mut self, node: &Var) -> Value;
    fn visit_assign(&mut self, node: &Assign) -> Value;
    fn visit_string_literal(&mut self, node: &StringLiteral) -> Value;
    fn visit_function_call(&mut self, node: &FunctionCall) -> Value;
    fn visit_comment(&mut self, node: &Comment) -> Value;
}

pub fn parse_script(source: String) -> Result<(), ScriptError> {
    let mut file = File::open(source)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut sections: Vec<&str> = Vec::new();
    let mut in_section = String::new();
    let mut project_name = String::new();

    // The tokens vector stores all the words of each section of the .oak script to tokenize them
    let tokens: Vec<&str> = content.split_whitespace().collect();
    let project_regex = Regex::new(r#""([^"]+)\.project""#)?;
    let section_full_regex =
        Regex::new(r#"(?s)BEGIN SECTION "([^"]+)"\s*(.*?)\s*END SECTION "([^"]+)""#)?;

    // Parses sections
    for line in content.lines() {
        in_section = "".to_string();
        if line.contains("BEGIN") && line.contains(".project") {
            in_section = "project".to_string();
            sections.push("project");

            if let Some(captures) = project_regex.captures(line) {
                if let Some(project_name_match) = captures.get(1) {
                    project_name = project_name_match.as_str().to_string();
                }
            }
        }

        for caps in section_full_regex.captures_iter(&content) {
            if let Some(section_name_match) = caps.get(1) {
                let section_name = section_name_match.as_str();
                // The values printed here are for debugging purposes.
                // After the development stage they will be removed or moved to a "debug-mode" section.
                println!("\n--- Found Section: '{}' ---", section_name);

                if let Some(section_content_match) = caps.get(2) {
                    let section_content = section_content_match.as_str().trim();
                    println!("Section Content:\n{}", section_content);

                    let section_tokens: Vec<&str> = section_content.split_whitespace().collect();
                    println!("Section Tokens: {:?}", section_tokens);
                } else {
                    println!("Warning: No content found for section '{}'", section_name);
                }
            }
        }
    }

    Ok(())
}
