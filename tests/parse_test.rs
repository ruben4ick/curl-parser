use anyhow::Result;
use curl_parser::parse_curl;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_curl_parse() -> Result<()> {
        parse_curl("curl https://crates.io/")?;
        Ok(())
    }
}
