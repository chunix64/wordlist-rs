# wordlist-cli

A fast CLI tool for extracting and analyzing word frequency statistics from files.

## Features

- Multiple file input support
- Recursive directory scanning
- Configurable sorting (none / asc / desc)
- Output formats: plain text and JSON
- Fast and lightweight

## Installation

### From source

```bash
cargo build --release
````

Binary will be in:

```bash
./target/release/wordlist-cli
```

## Usage

```bash
wordlist-cli [OPTIONS] <PATHS>...
```

### Basic example

```bash
wordlist-cli file.txt
```

### Multiple files

```bash
wordlist-cli file1.txt file2.txt file3.txt
```

### Recursive directory scan

```bash
wordlist-cli ./docs --recursive
```

## Options

```
-o, --output <OUTPUT>
      Output file path

--output-format <OUTPUT_FORMAT>
      Output format (default: txt)
      Possible values: txt, json

-r, --recursive
      Scan directories recursively

--sort <SORT>
      Sort order (default: desc)
      Possible values: none, asc, desc

-h, --help
      Print help

-V, --version
      Print version
```

## Sorting behavior

| Mode | Description                |
| ---- | -------------------------- |
| none | No ordering guarantee      |
| asc  | Lowest frequency → highest |
| desc | Highest frequency → lowest |

Tie-breaker: alphabetical order of the word.

## Output formats

### TXT (default)

```
word 42
example 15
rust 10
```

### JSON

```json
[
  { "word": "the", "frequency": 68068 },
  { "word": "to", "frequency": 41559 },
  { "word": "a", "frequency": 28997 },
]
```

## Example workflows

### Top words in a file

```bash
wordlist-cli text.txt --sort desc
```

### Export as JSON

```bash
wordlist-cli text.txt --output-format json -o result
```
To generate `result.json`

### Scan a directory

```bash
wordlist-cli ./notes -r --sort asc
```

## License

MIT license
