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