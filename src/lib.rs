use std::fs;
use std::io::{self, Write};
use regex::Regex;
use serde::{Serialize, Deserialize};

pub fn process_markdown() {
    // Placeholder for markdown processing function
}

pub fn call_perplexity_api() {
    // Placeholder for API call function
}

/// Function to select context blocks around an active block.
/// Assumes content is structured with hyphens ("- ") at the beginning of each block.
pub fn select_context_blocks(content: &str, active_block: &str) -> Vec<String> {
    // If content is empty, return an empty vector
    if content.trim().is_empty() {
        return vec![];
    }

    // Split the content by lines, remove leading "- ", and trim any extra spaces
    let blocks: Vec<&str> = content
        .lines()
        .map(|block| block.trim_start_matches("- ").trim())
        .filter(|block| !block.is_empty())
        .collect();

    let mut selected_blocks = Vec::new();

    // Find the active block's index
    if let Some(index) = blocks.iter().position(|&block| block == active_block) {
        // Determine the number of blocks to include before the active block
        let prev_blocks = if index == blocks.len() - 1 {
            // If active block is the last one, include up to two previous blocks
            2.min(index)
        } else {
            // Otherwise, include one previous block
            1.min(index)
        };

        // Add previous blocks
        selected_blocks.extend(blocks[index - prev_blocks..index].iter().map(|&s| s.to_string()));

        // Add the active block itself
        selected_blocks.push(active_block.to_string());

        // Add up to two following blocks if they exist
        let end = (index + 3).min(blocks.len());
        selected_blocks.extend(blocks[index+1..end].iter().map(|&s| s.to_string()));
    }

    selected_blocks
}

pub fn clean_logseq_links(input: &str) -> String {
    let re = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    re.replace_all(input, "$1").to_string()
}

pub fn process_markdown_block(input: &str, _prompt: &str, _topics: &[String], api_response: &str) -> String {
    let cleaned_input = clean_logseq_links(input);
    let mut output = Vec::new();
    writeln!(output, "- ```").unwrap();
    writeln!(output, "{}", cleaned_input.trim_start_matches("- ").trim_end()).unwrap();
    writeln!(output, "```").unwrap();
    write!(output, "{}", api_response).unwrap();
    String::from_utf8(output).unwrap()
}

pub fn load_prompt(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn load_topics(file_path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content.split(',').map(|s| s.trim().to_string()).collect())
}

// Perplexity API request structure
#[derive(Debug, Serialize)]
pub struct PerplexityRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub return_citations: Option<bool>,
    pub search_domain_filter: Option<Vec<String>>,
    pub return_images: Option<bool>,
    pub return_related_questions: Option<bool>,
    pub search_recency_filter: Option<String>,
    pub top_k: Option<u32>,
    pub stream: Option<bool>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// Perplexity API response structure
#[derive(Debug, Deserialize)]
pub struct PerplexityResponse {
    pub id: String,
    pub model: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub finish_reason: Option<String>,
    pub message: Message,
    pub delta: Option<Delta>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
