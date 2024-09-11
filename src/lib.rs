use std::fs;
use std::io::{self, Write};
use regex::Regex;
use serde::{Serialize, Deserialize};
use reqwest::Client;

pub async fn process_markdown(file_path: &str, prompt: &str, topics: &[String]) -> io::Result<String> {
    let content = fs::read_to_string(file_path)?;
    let blocks: Vec<&str> = content.split("\n- ").filter(|s| !s.trim().is_empty()).collect();
    let mut processed_content = String::new();

    for block in blocks {
        let trimmed_block = block.trim();
        let context = select_context_blocks(&content, trimmed_block);
        let api_response = call_perplexity_api(prompt, &context, topics).await?;
        let processed_block = process_markdown_block(trimmed_block, prompt, topics, &api_response);
        processed_content.push_str(&processed_block);
        processed_content.push('\n');
    }

    Ok(processed_content)
}

pub async fn call_perplexity_api(prompt: &str, context: &[String], topics: &[String]) -> io::Result<String> {
    let client = Client::new();
    let api_key = std::env::var("PERPLEXITY_API_KEY").map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

    let request = PerplexityRequest {
        model: "mistral-7b-instruct".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: format!("You are an AI assistant analyzing Logseq markdown blocks. The relevant topics are: {}.", topics.join(", ")),
            },
            Message {
                role: "user".to_string(),
                content: format!("Prompt: {}\n\nContext:\n{}", prompt, context.join("\n")),
            },
        ],
        max_tokens: Some(150),
        temperature: Some(0.7),
        top_p: None,
        return_citations: Some(false),
        search_domain_filter: None,
        return_images: Some(false),
        return_related_questions: Some(false),
        search_recency_filter: None,
        top_k: None,
        stream: Some(false),
        presence_penalty: None,
        frequency_penalty: None,
    };

    let response = client
        .post("https://api.perplexity.ai/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let response_text = response.text().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    match serde_json::from_str::<PerplexityResponse>(&response_text) {
        Ok(parsed_response) => Ok(parsed_response.choices[0].message.content.clone()),
        Err(e) => {
            eprintln!("Failed to parse API response: {}", e);
            eprintln!("Raw response: {}", response_text);
            if response_text.contains("error") {
                let error: serde_json::Value = serde_json::from_str(&response_text)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                if let Some(error_message) = error["error"]["message"].as_str() {
                    return Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()));
                }
            }
            Err(io::Error::new(io::ErrorKind::Other, "Failed to parse API response"))
        }
    }
}

// The rest of the file remains unchanged
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

#[derive(Debug, Deserialize)]
pub struct PerplexityResponse {
    pub id: Option<String>,
    pub model: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
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
