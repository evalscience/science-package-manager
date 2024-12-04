# Science Package Manager (spm)

A command-line tool for managing scientific paper references and funding information for research projects. SPM helps you maintain a structured record of cited papers and configure blockchain addresses for receiving funding.

## Features

- Track scientific paper references with DOIs and URLs
- Configure cryptocurrency addresses for research funding
- Simple command-line interface
- JSON-based storage format

## Installation

### Prerequisites

- Rust and Cargo (1.70.0 or newer)
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/evalscience/spm
cd spm

# Build and install
cargo build --release
cargo install --path .
```

## Usage

### Initialize a New Project

Create a new science project in your directory:

```bash
spm init
```

This creates two files:
- `FUNDING.json`: Stores your cryptocurrency wallet addresses
- `SCIENCE.json`: Stores your paper references

### Adding Papers

Add a scientific paper reference:

```bash
spm add "Author et al. (2024)"
```

You'll be prompted to optionally provide:
- DOI (Digital Object Identifier)
- URL to the paper

### Configuring Funding

Set up your funding wallet addresses:

```bash
spm fund
```

You can configure addresses for:
- Ethereum
- Optimism
- Celo

## File Formats

### FUNDING.json

```json
{
  "ethereum_address": "0x...",
  "optimism_address": "0x...",
  "celo_address": "0x..."
}
```

### SCIENCE.json

```json
{
  "papers": [
    {
      "citation": "Author et al. (2024)",
      "doi": "10.1234/example",
      "url": "https://example.com/paper"
    }
  ]
}
```

## Contributing

Contributions are welcome! Here are some ways you can contribute:

- Report bugs
- Suggest new features
- Submit pull requests
- Improve documentation

## Planned Features

- Paper validation
- DOI automatic lookup
- Citation format checking
- Export to various citation formats
- Multiple funding addresses per chain
- Integration with paper databases

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

If you find this tool useful, consider supporting the development by sending funds to any of the addresses in your FUNDING.json file.