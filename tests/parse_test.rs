#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use curl_parser_ruben4ick::{CurlParser, Rule};
    use pest::Parser;

    #[test]
    fn url_plain_parses() -> Result<()> {
        let input = r#"https://my.example.com"#;
        let pairs = CurlParser::parse(Rule::url, input).context("failed to parse url_plain")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn url_double_quoted_parses() -> Result<()> {
        let input = r#""https://my.example.com""#;
        let pairs =
            CurlParser::parse(Rule::url, input).context("failed to parse quoted url (double)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn url_single_quoted_parses() -> Result<()> {
        let input = r#"'https://my.example.com'"#;
        let pairs =
            CurlParser::parse(Rule::url, input).context("failed to parse quoted url (single)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn value_bare_word_parses() -> Result<()> {
        let input = r#"POST"#;
        let pairs = CurlParser::parse(Rule::value, input).context("failed to parse bare value")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn method_flag_short_parses() -> Result<()> {
        let input = r#"-X POST"#;
        let pairs =
            CurlParser::parse(Rule::method_flag, input).context("failed to parse -X flag")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn method_flag_long_quoted_parses() -> Result<()> {
        let input = r#"--request "post""#;
        let pairs = CurlParser::parse(Rule::method_flag, input)
            .context("failed to parse --request flag with quoted value")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn header_flag_double_quoted_parses() -> Result<()> {
        let input = r#"-H "Content-Type: application/json""#;
        let pairs = CurlParser::parse(Rule::header_flag, input)
            .context("failed to parse header flag (double-quoted)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn header_flag_single_quoted_parses() -> Result<()> {
        let input = r#"-H 'Authorization: Bearer 123'"#;
        let pairs = CurlParser::parse(Rule::header_flag, input)
            .context("failed to parse header flag (single-quoted)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn header_flag_bare_parses() -> Result<()> {
        let input = r#"-H Accept:application/json"#;
        let pairs = CurlParser::parse(Rule::header_flag, input)
            .context("Failed to parse header flag (bare)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn data_flag_double_quoted_parses() -> Result<()> {
        let input = r#"-d "{\"a\":1,\"b\":\"2\"}""#;
        let pairs = CurlParser::parse(Rule::data_flag, input)
            .context("Failed to parse data flag (double-quoted)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn data_flag_single_quoted_parses() -> Result<()> {
        let input = r#"-d '{"a":1,"b":"2"}'"#;
        let pairs = CurlParser::parse(Rule::data_flag, input)
            .context("failed to parse data flag (single-quoted)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn data_flag_bare_parses() -> Result<()> {
        let input = r#"-d a=1&b=2"#;
        let pairs = CurlParser::parse(Rule::data_flag, input)
            .context("failed to parse data flag (bare)")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }

    #[test]
    fn option_accepts_any_flag_variant() -> Result<()> {
        for input in [
            r#"-X POST"#,
            r#"--request "POST""#,
            r#"-H "Accept: application/json""#,
            r#"-d 'a=1'"#,
        ] {
            let pairs = CurlParser::parse(Rule::option, input).context("failed to parse option")?;
            assert_eq!(pairs.as_str(), input);
        }
        Ok(())
    }

    #[test]
    fn curl_minimal_with_plain_url() -> Result<()> {
        let input = r#"curl https://my.example.com"#;
        let pairs = CurlParser::parse(Rule::curl, input).context("failed to parse minimal curl")?;
        assert_eq!(pairs.as_str(), input);
        Ok(())
    }
}
