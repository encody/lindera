use serde::{Deserialize, Serialize};

use lindera_core::token_filter::TokenFilter;

use crate::{error::LinderaErrorKind, LinderaResult, Token};

pub const LENGTH_TOKEN_FILTER_NAME: &str = "length";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LengthTokenFilterConfig {
    min: Option<usize>,
    max: Option<usize>,
}

impl LengthTokenFilterConfig {
    pub fn new(min: Option<usize>, max: Option<usize>) -> Self {
        Self { min, max }
    }

    pub fn from_slice(data: &[u8]) -> LinderaResult<Self> {
        serde_json::from_slice(data).map_err(|err| LinderaErrorKind::Deserialize.with_error(err))
    }
}

#[derive(Clone, Debug)]
pub struct LengthTokenFilter {
    config: LengthTokenFilterConfig,
}

impl LengthTokenFilter {
    pub fn new(config: LengthTokenFilterConfig) -> Self {
        Self { config }
    }

    pub fn from_slice(data: &[u8]) -> LinderaResult<Self> {
        Ok(Self::new(LengthTokenFilterConfig::from_slice(data)?))
    }
}

impl TokenFilter for LengthTokenFilter {
    fn name(&self) -> &'static str {
        LENGTH_TOKEN_FILTER_NAME
    }

    fn apply<'a>(&self, tokens: &mut Vec<Token<'a>>) -> LinderaResult<()> {
        tokens.retain(|token| {
            let len = token.text.chars().count();
            if let Some(min) = self.config.min {
                if len < min {
                    return false;
                }
            }
            if let Some(max) = self.config.max {
                if len > max {
                    return false;
                }
            }
            true
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use lindera_core::token_filter::TokenFilter;

    use crate::{
        token_filter::length::{LengthTokenFilter, LengthTokenFilterConfig},
        Token,
    };

    #[test]
    fn test_length_token_filter_config_from_slice() {
        let config_str = r#"
        {
            "min": 1,
            "max": 3
        }
        "#;
        let config = LengthTokenFilterConfig::from_slice(config_str.as_bytes()).unwrap();

        assert_eq!(config.min.unwrap(), 1);
        assert_eq!(config.max.unwrap(), 3);

        let config_str = r#"
        {
            "min": 1
        }
        "#;
        let config = LengthTokenFilterConfig::from_slice(config_str.as_bytes()).unwrap();

        assert_eq!(config.min.unwrap(), 1);
        assert_eq!(config.max, None);

        let config_str = r#"
        {
            "max": 2
        }
        "#;
        let config = LengthTokenFilterConfig::from_slice(config_str.as_bytes()).unwrap();

        assert_eq!(config.min, None);
        assert_eq!(config.max.unwrap(), 2);
    }

    #[test]
    fn test_length_token_filter_from_slice() {
        let config_str = r#"
        {
            "min": 1,
            "max": 3
        }
        "#;
        let result = LengthTokenFilter::from_slice(config_str.as_bytes());

        assert_eq!(result.is_ok(), true);

        let config_str = r#"
        {
            "min": 1
        }
        "#;
        let result = LengthTokenFilter::from_slice(config_str.as_bytes());

        assert_eq!(result.is_ok(), true);

        let config_str = r#"
        {
            "max": 2
        }
        "#;
        let result = LengthTokenFilter::from_slice(config_str.as_bytes());

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_length_token_filter_apply() {
        let config_str = r#"
        {
            "min": 3,
            "max": 3
        }
        "#;
        let filter = LengthTokenFilter::from_slice(config_str.as_bytes()).unwrap();

        let mut tokens: Vec<Token> = vec![
            Token {
                text: Cow::Borrowed("to"),
                details: None,
                byte_start: 0,
                byte_end: 2,
            },
            Token {
                text: Cow::Borrowed("be"),
                details: None,
                byte_start: 3,
                byte_end: 5,
            },
            Token {
                text: Cow::Borrowed("or"),
                details: None,
                byte_start: 6,
                byte_end: 8,
            },
            Token {
                text: Cow::Borrowed("not"),
                details: None,
                byte_start: 9,
                byte_end: 12,
            },
            Token {
                text: Cow::Borrowed("to"),
                details: None,
                byte_start: 13,
                byte_end: 15,
            },
            Token {
                text: Cow::Borrowed("be"),
                details: None,
                byte_start: 16,
                byte_end: 18,
            },
            Token {
                text: Cow::Borrowed("this"),
                details: None,
                byte_start: 19,
                byte_end: 23,
            },
            Token {
                text: Cow::Borrowed("is"),
                details: None,
                byte_start: 24,
                byte_end: 26,
            },
            Token {
                text: Cow::Borrowed("the"),
                details: None,
                byte_start: 27,
                byte_end: 30,
            },
            Token {
                text: Cow::Borrowed("question"),
                details: None,
                byte_start: 31,
                byte_end: 39,
            },
        ];

        filter.apply(&mut tokens).unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "not");
        assert_eq!(tokens[1].text, "the");
    }
}
