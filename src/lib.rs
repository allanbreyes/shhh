mod rules;

use clap::clap_derive::Parser;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio, ChildStdout};

#[derive(Debug, Default, Parser)]
#[clap(version, about)]
/// A simple shell wrapper that keeps secrets out of build and CI logs.
pub struct Config {
    /// The file to run
    file: String,

    /// Path of the shell to use
    #[arg(short, long, default_value_t = String::from(env!("SHELL")))]
    shell: String,
}

pub fn run(config: Config) -> Result<Summary, Box<dyn std::error::Error>> {
    let mut child = Command::new(config.shell)
        .arg(config.file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    // TODO: handle errors and stderr
    let stdout = BufReader::new(child.stdout.take().unwrap());
    let summary = filter_streams(stdout)?;

    child.wait()?;

    Ok(summary)
}

pub struct Summary {
    pub lines_redacted: usize,
}

pub fn filter_streams(stdout: BufReader<ChildStdout>) -> Result<Summary, Box<dyn std::error::Error>> {
    let rules = rules::load_rules(include_str!("../rules.yml"))?;

    // TODO: refactor to threads and channels to handle concurrent streams
    let mut buffer = Vec::new();
    let mut lines_redacted = 0;

    let mut activated_multiline_rules: Vec<&str> = Vec::new();

    for line in stdout.lines() {
        let mut line = line?;
        let mut redacted = false;
        for rule in &rules {
            if rule.is_start(&line) {
                activated_multiline_rules.push(rule.id());
                line = rule.replace_start(&line);
                redacted = true;
            }
            if rule.is_end(&line) {
                let mut popped = false;
                activated_multiline_rules.retain(|&x| {
                    if popped || x != rule.id() {
                        true
                    } else {
                        popped = true;
                        false
                    }
                });
                line = rule.replace_end(&line);
                redacted = true;
            }
            if activated_multiline_rules.contains(&rule.id()) {
                line = rule.replace(&line);
                redacted = true;
            } else if rule.is_match(&line) {
                line = rule.replace(&line);
                redacted = true;
            }
        }
        buffer.push(line);

        if redacted {
            lines_redacted += 1;
        }

        if activated_multiline_rules.is_empty() {
            for line in &buffer {
                println!("{}", line);
            }
            buffer.clear();
        }
    }

    Ok(Summary { lines_redacted })
}
