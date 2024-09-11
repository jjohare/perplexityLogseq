
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
