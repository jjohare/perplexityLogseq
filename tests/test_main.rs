use logseq_perplexity_integration::{clean_logseq_links, select_context_blocks, process_markdown_block, load_prompt, load_topics, process_markdown, call_perplexity_api};
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_process_markdown() {
        let markdown_content = "- Block 1 with [[link]]\n- Block 2\n- Block 3 with [[another link]]\n";
        fs::write("test_markdown.md", markdown_content).unwrap();

        let prompt = "Test prompt";
        let topics = vec!["Topic 1".to_string(), "Topic 2".to_string()];

        let result = process_markdown("test_markdown.md", prompt, &topics).await;

        assert!(result.is_ok(), "process_markdown failed: {:?}", result.err());
        let processed_content = result.unwrap();

        // Check that the output contains the expected structure
        assert!(processed_content.contains("- ```\nBlock 1 with link\n```\n"));
        assert!(processed_content.contains("- ```\nBlock 2\n```\n"));
        assert!(processed_content.contains("- ```\nBlock 3 with another link\n```\n"));

        fs::remove_file("test_markdown.md").unwrap();
    }

    #[tokio::test]
    async fn test_call_perplexity_api() {
        let prompt = "Analyze this text";
        let context = vec!["This is a test context".to_string()];
        let topics = vec!["Topic 1".to_string(), "Topic 2".to_string()];

        let result = call_perplexity_api(prompt, &context, &topics).await;

        assert!(result.is_ok(), "API call failed: {:?}", result.err());
        let response = result.unwrap();
        assert!(!response.is_empty(), "API response is empty");
    }

    // Test cleaning a single Logseq link with [[ ]]
    #[test]
    fn test_clean_logseq_links() {
        let input = "This is a test [[link]] to Logseq.";
        let expected = "This is a test link to Logseq.";
        let result = clean_logseq_links(input);
        assert_eq!(result, expected);
    }

    // Test cleaning multiple Logseq links in the same sentence
    #[test]
    fn test_clean_logseq_links_with_multiple() {
        let input = "Multiple [[links]] in [[one]] sentence.";
        let expected = "Multiple links in one sentence.";
        let result = clean_logseq_links(input);
        assert_eq!(result, expected);
    }

    // Test a string with no Logseq links to ensure it's returned unchanged
    #[test]
    fn test_clean_logseq_links_no_brackets() {
        let input = "No special links here.";
        let result = clean_logseq_links(input);
        assert_eq!(result, input);
    }

    // Test selecting blocks around an active block in the middle of the content
    #[test]
    fn test_select_context_blocks() {
        let content = "- Block 1\n- Block 2\n- Active Block\n- Block 4\n- Block 5\n- Block 6";
        let active_block = "Active Block";
        let expected = vec![
            "Block 2".to_string(),
            "Active Block".to_string(),
            "Block 4".to_string(),
            "Block 5".to_string(),
        ];

        let result = select_context_blocks(content, active_block);
        assert_eq!(result, expected);
    }

    // Test selecting blocks when the active block is at the start of the content
    #[test]
    fn test_select_context_blocks_start_of_file() {
        let content = "- Active Block\n- Block 2\n- Block 3\n- Block 4";
        let active_block = "Active Block";
        let expected = vec![
            "Active Block".to_string(),
            "Block 2".to_string(),
            "Block 3".to_string(),
        ];

        let result = select_context_blocks(content, active_block);
        assert_eq!(result, expected);
    }

    // Test selecting blocks when the active block is near the end of the content
    #[test]
    fn test_select_context_blocks_end_of_file() {
        let content = "- Block 1\n- Block 2\n- Block 3\n- Active Block";
        let active_block = "Active Block";
        let expected = vec![
            "Block 2".to_string(),
            "Block 3".to_string(),
            "Active Block".to_string(),
        ];

        let result = select_context_blocks(content, active_block);
        assert_eq!(result, expected);
    }

    // Test selecting blocks when the content is empty
    #[test]
    fn test_select_context_blocks_empty_content() {
        let content = "";  // Empty content
        let active_block = "Active Block";
        let expected: Vec<String> = vec![];

        let result = select_context_blocks(content, active_block);
        assert_eq!(result, expected);
    }

    // Test selecting blocks when the active block does not exist in the content
    #[test]
    fn test_select_context_blocks_no_active_block() {
        let content = "- Block 1\n- Block 2\n- Block 3";
        let active_block = "Nonexistent Block";
        let expected: Vec<String> = vec![];

        let result = select_context_blocks(content, active_block);
        assert_eq!(result, expected);
    }

    // Test process_markdown_block
    #[test]
    fn test_process_markdown_block() {
        let input = "- This is a test [[block]] with a [[link]].\n";
        let prompt = "Analyze this text";
        let topics = vec!["Topic 1".to_string(), "Topic 2".to_string()];
        let api_response = "API response goes here";

        let expected_output = "- ```\nThis is a test block with a link.\n```\nAPI response goes here";

        let result = process_markdown_block(input, prompt, &topics, api_response);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_load_prompt() {
        let prompt_content = "This is a test prompt\n";
        fs::write("test_prompt.md", prompt_content).unwrap();
        
        let result = load_prompt("test_prompt.md");
        assert_eq!(result.unwrap(), prompt_content);

        fs::remove_file("test_prompt.md").unwrap();
    }

    #[test]
    fn test_load_topics() {
        let topics_content = "Topic 1,Topic 2,Topic with spaces";
        fs::write("test_topics.csv", topics_content).unwrap();
        
        let expected = vec!["Topic 1".to_string(), "Topic 2".to_string(), "Topic with spaces".to_string()];
        let result = load_topics("test_topics.csv");
        assert_eq!(result.unwrap(), expected);

        fs::remove_file("test_topics.csv").unwrap();
    }
}
