
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
