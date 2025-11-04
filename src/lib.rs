use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./curl_grammar.pest"]
pub struct CurlParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Pest(String),
}

pub fn parse_curl(input: &str) -> Result<(), ParseError> {
    CurlParser::parse(Rule::curl, input)
        .map(|_| ())
        .map_err(|e| ParseError::Pest(e.to_string()))
}
