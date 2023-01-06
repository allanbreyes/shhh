mod rules;

use clap::clap_derive::Parser;
use regex::Regex;
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
    // TODO: actually do something with the loaded rules
    let _rules = rules::load_rules(include_str!("../rules.yml"))?;

    // TODO: refactor to threads and channels to handle concurrent streams
    let mut buffer = Vec::new();
    let mut lines_redacted = 0;

    // TODO: evaluate multiple rules
    let mut in_sensitive_block = false;
    let ssh_key_start = Regex::new(r"-----BEGIN .* PRIVATE KEY-----")?;
    let ssh_key_end = Regex::new(r"-----END .* PRIVATE KEY-----")?;

    for line in stdout.lines() {
        let line = line?;
        if ssh_key_start.is_match(line.as_str()) {
            buffer.push(String::from("*** SSH private key ***"));
            lines_redacted += 1;
            in_sensitive_block = true;
        } else if in_sensitive_block {
            buffer.push(String::from("*** SSH private key ***"));
            lines_redacted += 1;
            if ssh_key_end.is_match(line.as_str()) {
                for line in &buffer {
                    println!("{}", line);
                }
                in_sensitive_block = false;
            }
        } else {
            println!("{}", line);
        }
    }

    Ok(Summary { lines_redacted })
}
