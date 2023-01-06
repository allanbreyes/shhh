use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RuleFile {
    version: u32,
    rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "mode")]
pub enum Rule {
    #[serde(rename = "multiline-regex")]
    MultilineRegex(MultilineRegexSchema),

    #[serde(rename = "regex")]
    Regex(RegexSchema),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MultilineRegexSchema {
    pub id: String,
    pub pattern_start: String,
    pub pattern_mid: String,
    pub pattern_end: String,
}

#[derive(Debug, Deserialize)]
pub struct RegexSchema {
    pub id: String,
    pub pattern: String,
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
        let rules = load_rules(yaml).unwrap();
    }
}