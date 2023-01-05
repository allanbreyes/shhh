use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = shhh::Config::parse();
    let output = shhh::run(config)?;
    eprintln!("{} lines redacted", output.lines_redacted);
    Ok(())
}
