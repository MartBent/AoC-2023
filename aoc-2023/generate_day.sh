#!/bin/bash

# Check if the parameter is provided
if [ -z "$1" ]; then
  exit 1
fi

# Create the folder
mkdir "src/day$1"

# Create the Rust module file
MOD_FILE="src/day$1"/mod.rs
cp boilerplate_code.txt $MOD_FILE

INPUT_FILE="src/day$1"/input.txt
INPUT_LINK="https://adventofcode.com/2020/day/$1/input"
echo $INPUT_LINK
curl --cookie "session=$(cat .session)" $INPUT_LINK > $INPUT_FILE

echo "Module 'day$1' created successfully."
