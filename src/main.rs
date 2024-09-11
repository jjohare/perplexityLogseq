use std::env;
use std::fs;
use std::io::{self, Write};
use logseq_perplexity_integration::{process_markdown, load_prompt, load_topics};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <input_markdown> <prompt_file> <topics_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let prompt_file = &args[2];
    let topics_file = &args[3];
    let output_file = &args[4];

    // Load prompt and topics
    let prompt = load_prompt(prompt_file)?;
    let topics = load_topics(topics_file)?;

    // Process markdown
    let processed_content = process_markdown(input_file, &prompt, &topics).await?;

    // Write result to output file
    fs::write(output_file, processed_content)?;

    println!("Processing complete. Output written to {}", output_file);

    Ok(())
}
