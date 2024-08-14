# Fork Cracker

This tool helps you find potentially sensitive information leaks on GitHub by scanning commit history for specific files.

## Features

* **Simple Scan:** Checks the validity of generated commit URLs.
* **Advanced Scan:** Allows you to specify a file name to search for within commits.
* **Hexadecimal Generation:** Uses a sequential hexadecimal generation algorithm to check a range of potential commit IDs.
* **Output:** Saves found URLs to a text file.

## How it Works

The tool generates a series of hexadecimal strings representing potential commit IDs. It then checks if these IDs correspond to valid commits on the specified GitHub repository.

For advanced scans, it further checks if the commit modified the specified file.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/RobinHirst11/fork-cracker.git
   ```

2. Navigate to the directory:

   ```bash
   cd fork-cracker
   ```

3. Run the tool:

   ```bash
   cargo run
   ```

## Usage

1. Run the tool and choose the "Run" option.
2. Select either "Simple" or "Advanced" scan.
3. Enter the GitHub repository URL (e.g., `username/repo`).
4. For advanced scans, enter the file name you are looking for.
5. The tool will output a list of URLs to commits that match your criteria.

**Example:**

To find commits that modified a file named `config.js` in the repository `user/repo`:

1. Choose "Advanced" scan.
2. Enter `user/repo` as the repository.
3. Enter `config.js` as the file name.

## Disclaimer

This tool is provided as-is and should be used responsibly and ethically. The creator is not responsible for any misuse or damage caused by this tool.

## Contributing

Contributions are welcome! Please submit pull requests for bug fixes, feature requests, or improvements.

## License

This project is licensed under the [HPOG](LICENSE) license
