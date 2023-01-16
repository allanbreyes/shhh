use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RuleFile {
    version: u32,
    rules: Vec<Rule>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "mode")]
pub enum Rule {
    #[serde(rename = "multiline-regex")]
    MultilineRegex(MultilineRegexSchema),

    #[serde(rename = "regex")]
    Regex(RegexSchema),
}

impl Rule {
    pub fn id(&self) -> &str {
        match self {
            Rule::MultilineRegex(schema) => &schema.id,
            Rule::Regex(schema) => &schema.id,
        }
    }

    pub fn is_match(&self, line: &str) -> bool {
        match self {
            Rule::MultilineRegex(_) => false,
            Rule::Regex(schema) => {
                schema.pattern.is_match(line)
            }
        }
    }

    pub fn is_start(&self, line: &str) -> bool {
        match self {
            Rule::MultilineRegex(schema) => {
                schema.pattern_start.is_match(line)
            }
            Rule::Regex(_) => {
                false
            }
        }
    }

    pub fn is_end(&self, line: &str) -> bool {
        match self {
            Rule::MultilineRegex(schema) => {
                schema.pattern_end.is_match(line)
            }
            Rule::Regex(_) => {
                false
            }
        }
    }

    pub fn replace(&self, line: &str) -> String {
        match self {
            Rule::MultilineRegex(schema) => {
                schema.pattern_mid.replace_all(line, format!("*** {} ***", schema.id)).to_string()
            }
            Rule::Regex(schema) => {
                schema.pattern.replace_all(line, format!("*** {} ***", schema.id)).to_string()
            }
        }
    }

    pub fn replace_start(&self, line: &str) -> String {
        match self {
            Rule::MultilineRegex(schema) => {
                schema.pattern_start.replace_all(line, format!("*** start({}) ***", schema.id)).to_string()
            }
            Rule::Regex(_) => {
                line.to_string()
            }
        }
    }

    pub fn replace_end(&self, line: &str) -> String {
        match self {
            Rule::MultilineRegex(schema) => {
                schema.pattern_end.replace_all(line, format!("*** end({}) ***", schema.id)).to_string()
            }
            Rule::Regex(_) => {
                line.to_string()
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MultilineRegexSchema {
    pub id: String,
    #[serde(with = "serde_regex")]
    pub pattern_start: Regex,
    #[serde(with = "serde_regex")]
    pub pattern_mid: Regex,
    #[serde(with = "serde_regex")]
    pub pattern_end: Regex,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RegexSchema {
    pub id: String,
    #[serde(with = "serde_regex")]
    pub pattern: Regex,
}

pub fn load_rules(yaml: &str) -> Result<Vec<Rule>, Box<dyn std::error::Error>> {
    // TODO: handle errors with more helpful messages
    let file: RuleFile = serde_yaml::from_str(yaml)?;
    if file.version != 1 {
        return Err("Invalid version".into());
    }
    Ok(file.rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_rules() {
        let yaml = include_str!("../rules.yml");
        let _rules = load_rules(yaml).unwrap();
    }
}