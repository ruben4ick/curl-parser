use pest::Parser;
use pest::error::Error as PestError;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./curl_grammar.pest"]
pub struct CurlParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Pest(#[from] Box<PestError<Rule>>),

    #[error("missing URL in curl command")]
    MissingUrl,

    #[error("missing value for flag {0}")]
    MissingValue(&'static str),
}

#[derive(Debug)]
pub struct CurlRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Display for CurlRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "method: {}", self.method)?;
        writeln!(f, "url: {}", self.url)?;
        writeln!(f, "headers:")?;
        for (k, v) in &self.headers {
            writeln!(f, "  {}: {}", k, v)?;
        }
        writeln!(f, "body:\n{}", self.body)
    }
}

impl CurlRequest {
    pub fn parse_input(input: &str) -> Result<Self, ParseError> {
        let pairs =
            CurlParser::parse(Rule::curl, input).map_err(|e| ParseError::Pest(Box::new(e)))?;

        let mut method = String::new();
        let mut url = String::new();
        let mut headers = HashMap::new();
        let mut body = String::new();

        for pair in pairs {
            if pair.as_rule() != Rule::curl {
                continue;
            }
            for p in pair.into_inner() {
                match p.as_rule() {
                    Rule::url => {
                        if url.is_empty() {
                            url = strip_quotes(p.as_str()).to_string();
                        }
                    }
                    Rule::method_flag => {
                        let v = p
                            .into_inner()
                            .next()
                            .ok_or(ParseError::MissingValue("-X/--request"))?;
                        method = strip_quotes(v.as_str()).to_uppercase();
                    }
                    Rule::header_flag => {
                        let v = p
                            .into_inner()
                            .next()
                            .ok_or(ParseError::MissingValue("-H/--header"))?;
                        if let Some((k, v)) = split_header(strip_quotes(v.as_str())) {
                            headers.insert(k, v);
                        }
                    }
                    Rule::data_flag => {
                        let v = p
                            .into_inner()
                            .next()
                            .ok_or(ParseError::MissingValue("-d/--data"))?;
                        body = strip_quotes(v.as_str()).to_string();
                    }
                    _ => {}
                }
            }
        }

        if url.is_empty() {
            return Err(ParseError::MissingUrl);
        }

        Ok(CurlRequest {
            method,
            url,
            headers,
            body,
        })
    }
}

pub fn parse(input: &str) -> Result<CurlRequest, ParseError> {
    CurlRequest::parse_input(input)
}

fn strip_quotes(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn split_header(raw: &str) -> Option<(String, String)> {
    let (k, v) = raw.split_once(':')?;
    Some((k.trim().to_string(), v.trim().to_string()))
}
