## Fork Cracker: Uncovering Sensitive Information Leaks in GitHub Forks

**Fork Cracker** is a powerful tool designed to help security researchers and developers identify potential sensitive information leaks in GitHub repositories by scanning forks for specific files.

### Key Features:

* **Simple Scan:** Quickly validate the existence of potential commit URLs.
* **Advanced Scan:**  Pinpoint commits containing specific files, revealing potential leaks of sensitive data like API keys, credentials, or proprietary code.
* **Hexadecimal Generation:** Employs a robust sequential hexadecimal generation algorithm to efficiently check a wide range of potential commit IDs.
* **Targeted Output:** Saves discovered URLs to a text file for further analysis and investigation.

### How It Works:

Fork Cracker generates a series of hexadecimal strings representing potential commit IDs. It then leverages the GitHub API to verify the validity of these IDs within the specified repository.

During advanced scans, the tool goes a step further by checking if the identified commits modified the target file, providing a more focused search for potential leaks.

### Installation:

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/RobinHirst11/fork-cracker.git
   ```

2. **Navigate to the Directory:**

   ```bash
   cd fork-cracker
   ```

3. **Run the Tool:**

   ```bash
   cargo run
   ```

### Usage:

1. **Launch the Tool:** Run the executable and select the "Run" option.
2. **Choose Scan Type:** Select either "Simple" for a quick check or "Advanced" for a targeted file search.
3. **Enter Repository URL:** Provide the GitHub repository URL (e.g., `username/repo`).
4. **Specify File Name (Advanced Scan Only):** For advanced scans, enter the name of the file you are looking for.
5. **Analyze Results:** The tool will output a list of URLs to commits that match your criteria, allowing you to investigate potential leaks.

**Example:**

To find commits that modified a file named `config.js` in the repository `user/repo`:

1. Choose "Advanced" scan.
2. Enter `user/repo` as the repository.
3. Enter `config.js` as the file name.

### Disclaimer:

Fork Cracker is provided for educational and security research purposes only. Use this tool responsibly and ethically. The creator is not liable for any misuse or damage resulting from its use.

### Contributing:

We welcome contributions! Please submit pull requests for bug fixes, feature enhancements, or improvements.

<a href="https://github.com/RobinHirst11/fork-cracker/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=RobinHirst11/fork-cracker" />
</a>

### License:

This project is licensed under the [Poblic](LICENSE) license.
