#!/bin/bash

# URL to fetch crates data
BASE_URL="https://crates.io/api/v1/crates"
OUTPUT_FILE="top_1000_recent_crates.txt"

# Initialize variables
URL="$BASE_URL?per_page=100&sort=recent-downloads"

true > "$OUTPUT_FILE"

# hack for getting around kurt's custom jq version
JQ_ARGS="-r"
if [[ "$(which jq)" = "$HOME/.cargo/bin/jq" ]]; then
    JQ_ARGS=""
fi

# Loop to fetch and process each page
for _ in {1..10}; do
    echo Requesting "$URL"
    RES=$(curl -s "$URL")
    NEXT_PAGE=$(echo "$RES" | jq $JQ_ARGS '.meta.next_page')
    URL="$BASE_URL$NEXT_PAGE"
    echo "$RES" | jq  $JQ_ARGS '.crates[].name' >> "$OUTPUT_FILE"
done
