#!/bin/bash

# Set the Perplexity API key
export PERPLEXITY_API_KEY="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"

# Check if all arguments are provided
if [ "$#" -ne 4 ]; then
    echo "Usage: $0 <input_markdown> <prompt_file> <topics_file> <output_file>"
    exit 1
fi

# Run the application
cargo run -- "$1" "$2" "$3" "$4"
