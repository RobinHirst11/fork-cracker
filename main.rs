use reqwest;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::fs;
use chrono::Local;

fn check_commit(client: &reqwest::blocking::Client, commit_id: &str, repo: &str) -> Result<bool, reqwest::Error> {
    let url = format!("https://github.com/{}/commit/{}", repo, commit_id);
    let response = client.head(&url).send()?;
    Ok(response.status().is_success())
}

fn check_repo_exists(client: &reqwest::blocking::Client, repo: &str) -> Result<bool, reqwest::Error> {
    let url = format!("https://github.com/{}", repo);
    let response = client.head(&url).send()?;
    Ok(response.status().is_success())
}

fn clear_terminal() {
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    std::io::stdout().flush().unwrap();
}

fn print_header() {
    println!("-----------------------------------------------------------------");
    println!("GitHub Fork Hex Collector");
    println!("Made by RobinHirst11");
    println!("-----------------------------------------------------------------");
}

fn generate_sequential_hex(length: usize, index: u64) -> String {
    let charset: Vec<char> = "abcdef0123456789".chars().collect();
    let charset_size = charset.len() as u64;

    let mut hex = String::new();
    let mut remaining = index;

    for _ in 0..length {
        let digit = remaining % charset_size;
        hex.push(charset[digit as usize]);
        remaining /= charset_size;
    }

    hex.chars().rev().collect()
}

fn check_url_for_phrase(client: &reqwest::blocking::Client, url: &str, target_file: &str) -> Result<bool, reqwest::Error> {
    let response = client.get(url).send()?;
    if response.status().is_success() {
        let html = response.text()?;
        Ok(html.contains(&format!("<a title=\"{}\" class=\"Link--primary Truncate-text\" href=\"", target_file)))
    } else {
        Ok(false)
    }
}

fn main() {
    let client = reqwest::blocking::Client::new();
    clear_terminal();
    print_header();

    let mut hex_length = 6;

    loop {
        println!("1. Run");
        println!("2. Instructions");
        println!("3. About");
        println!("4. Hex Config");
        println!("5. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        clear_terminal();
        print_header();

        match choice.trim() {
            "1" => {
                loop {
                    println!("1. Simple");
                    println!("2. Advanced");
                    println!("3. Help");
                    println!("4. Back");

                    let mut run_choice = String::new();
                    std::io::stdin().read_line(&mut run_choice).unwrap();
                    clear_terminal();
                    print_header();

                    match run_choice.trim() {
                        "1" => {
                            let mut repo = String::new();
                            loop {
                                if !repo.is_empty() {
                                    println!("Error: Repository not found. Please check the username/repo and try again. Press Enter to continue.");
                                }

                                println!("Enter your GitHub repo that you wish to scan (username/repo):");
                                repo.clear();
                                std::io::stdin().read_line(&mut repo).unwrap();
                                clear_terminal();
                                print_header();
                                repo = repo.trim().to_string();

                                if repo.is_empty() {
                                    println!("Repository cannot be empty. Please try again. Press Enter to continue.");
                                    let _ = std::io::stdin().read_line(&mut String::new());
                                    clear_terminal();
                                    print_header();
                                    continue;
                                }

                                match check_repo_exists(&client, &repo) {
                                    Ok(exists) => {
                                        if exists {
                                            clear_terminal();
                                            print_header();
                                            break;
                                        } else {
                                            println!("Error: Repository not found. Please check the username/repo and try again. Press Enter to continue.");
                                            let _ = std::io::stdin().read_line(&mut String::new());
                                            clear_terminal();
                                            print_header();
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error checking repository: {}. Press Enter to continue.", e);
                                        let _ = std::io::stdin().read_line(&mut String::new());
                                        clear_terminal();
                                        print_header();
                                    }
                                }
                            }

                            let charset_size: u32 = 36;
                            let num_hexs = charset_size.pow(hex_length as u32);

                            println!("Enter your output file name:");
                            let mut output_file = String::new();
                            std::io::stdin().read_line(&mut output_file).unwrap();
                            clear_terminal();
                            print_header();

                            let now = Local::now();
                            let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();

                            let output_path =
                                if output_file.trim().is_empty() {
                                    Path::new("output_files").join(format!("untitled-{}.txt", timestamp))
                                } else {
                                    Path::new("output_files").join(output_file.trim())
                                };

                            if !Path::new("output_files").exists() {
                                fs::create_dir("output_files").unwrap();
                            }

                            let mut found_count = 0;
                            let mut forks_file = match File::create(&output_path) {
                                Ok(file) => file,
                                Err(e) => {
                                    println!("Error creating {}: {}. Press Enter to continue.", output_path.display(), e);
                                    let _ = std::io::stdin().read_line(&mut String::new());
                                    clear_terminal();
                                    print_header();
                                    return;
                                }
                            };

                            println!("found {} so far...", found_count);
                            println!("trying: {}", generate_sequential_hex(hex_length, 0));
                            for i in 0..num_hexs {
                                let commit_id = generate_sequential_hex(hex_length, i as u64);

                                match check_commit(&client, &commit_id, &repo) {
                                    Ok(found) => {
                                        if found {
                                            let url = format!("https://github.com/{}/commit/{}", &repo, commit_id);
                                            found_count += 1;
                                            writeln!(forks_file, "{}", url).unwrap();
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error checking commit: {}. Press Enter to continue.", e);
                                        let _ = std::io::stdin().read_line(&mut String::new());
                                        clear_terminal();
                                        print_header();
                                    }
                                }
                                clear_terminal();
                                print_header();
                                println!("found {} so far...", found_count);
                                println!("trying: {}", commit_id);
                            }

                            clear_terminal();
                            print_header();

                            println!("found {} GitHub forks in total, saved to {}", found_count, output_path.display());
                            println!("have a nice day :)");

                            println!("1. Continue");
                            println!("2. Exit");
                            let mut continue_choice = String::new();
                            std::io::stdin().read_line(&mut continue_choice).unwrap();
                            clear_terminal();

                            match continue_choice.trim() {
                                "1" => {
                                    clear_terminal();
                                    print_header();
                                },
                                "2" => {
                                    clear_terminal();
                                    break;
                                },
                                _ => {
                                    println!("Invalid choice. Press Enter to continue.");
                                    let _ = std::io::stdin().read_line(&mut String::new());
                                    clear_terminal();
                                    print_header();
                                }
                            }

                        }
                        "2" => {
                            let mut repo = String::new();
                            loop {
                                if !repo.is_empty() {
                                    println!("Error: Repository not found. Please check the username/repo and try again. Press Enter to continue.");
                                }

                                println!("Enter your GitHub repo that you wish to scan (username/repo):");
                                repo.clear();
                                std::io::stdin().read_line(&mut repo).unwrap();
                                clear_terminal();
                                print_header();
                                repo = repo.trim().to_string();

                                if repo.is_empty() {
                                    println!("Repository cannot be empty. Please try again. Press Enter to continue.");
                                    let _ = std::io::stdin().read_line(&mut String::new());
                                    clear_terminal();
                                    print_header();
                                    continue;
                                }

                                match check_repo_exists(&client, &repo) {
                                    Ok(exists) => {
                                        if exists {
                                            clear_terminal();
                                            print_header();
                                            break;
                                        } else {
                                            println!("Error: Repository not found. Please check the username/repo and try again. Press Enter to continue.");
                                            let _ = std::io::stdin().read_line(&mut String::new());
                                            clear_terminal();
                                            print_header();
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error checking repository: {}. Press Enter to continue.", e);
                                        let _ = std::io::stdin().read_line(&mut String::new());
                                        clear_terminal();
                                        print_header();
                                    }
                                }
                            }

                            let mut target_file = String::new();

                            println!("Do you want to check for a specific file? [y/n]");
                            let mut check_specific_file = String::new();
                            std::io::stdin().read_line(&mut check_specific_file).unwrap();
                            clear_terminal();
                            print_header();

                            if check_specific_file.trim().to_lowercase() == "y" {
                                loop {
                                    println!("Enter the file name to check for (e.g., .env), or type 'cancel' to go back:");
                                    std::io::stdin().read_line(&mut target_file).unwrap();
                                    clear_terminal();
                                    print_header();

                                    target_file = target_file.trim().to_string();

                                    if target_file.to_lowercase() == "cancel" {
                                        break;
                                    } else if target_file.is_empty() {
                                        println!("File name cannot be empty. Please try again. Press Enter to continue.");
                                        let _ = std::io::stdin().read_line(&mut String::new());
                                        clear_terminal();
                                        print_header();
                                        continue;
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                println!("Please enter a file name:");
                                std::io::stdin().read_line(&mut target_file).unwrap();
                                clear_terminal();
                                print_header();
                            }

                            let charset_size: u32 = 36;
                            let num_hexs = charset_size.pow(hex_length as u32);

                            println!("Enter your output file name:");
                            let mut output_file = String::new();
                            std::io::stdin().read_line(&mut output_file).unwrap();
                            clear_terminal();
                            print_header();

                            let now = Local::now();
                            let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();

                            let output_path =
                                if output_file.trim().is_empty() {
                                    Path::new("output_files").join(format!("untitled-{}.txt", timestamp))
                                } else {
                                    Path::new("output_files").join(output_file.trim())
                                };

                            if !Path::new("output_files").exists() {
                                fs::create_dir("output_files").unwrap();
                            }

                            let mut found_count = 0;
                            let mut forks_file = match File::create(&output_path) {
                                Ok(file) => file,
                                Err(e) => {
                                    println!("Error creating {}: {}. Press Enter to continue.", output_path.display(), e);
                                    let _ = std::io::stdin().read_line(&mut String::new());
                                    clear_terminal();
                                    print_header();
                                    return;
                                }
                            };

                            println!("found {} so far...", found_count);
                            println!("trying: {}", generate_sequential_hex(hex_length, 0));
                            for i in 0..num_hexs {
                                let commit_id = generate_sequential_hex(hex_length, i as u64);

                                match check_commit(&client, &commit_id, &repo) {
                                    Ok(found) => {
                                        if found {
                                            let url = format!("https://github.com/{}/commit/{}", repo, commit_id);

                                            match check_url_for_phrase(&client, &url, &target_file) {
                                                Ok(contains_phrase) => {
                                                    if contains_phrase {
                                                        found_count += 1;
                                                        writeln!(forks_file, "{}", url).unwrap();
                                                    }
                                                }
                                                Err(e) => {
                                                    println!("Error checking URL for phrase: {}. Press Enter to continue.", e);
                                                    let _ = std::io::stdin().read_line(&mut String::new());
                                                    clear_terminal();
                                                    print_header();
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error checking commit: {}. Press Enter to continue.", e);
                                        let _ = std::io::stdin().read_line(&mut String::new());
                                        clear_terminal();
                                        print_header();
                                    }
                                }
                                clear_terminal();
                                print_header();
                                println!("found {} so far...", found_count);
                                println!("trying: {}", commit_id);
                            }

                            clear_terminal();
                            print_header();

                            println!("found {} GitHub forks in total, saved to {}", found_count, output_path.display());
                            println!("have a nice day :)");

                            println!("1. Continue");
                            println!("2. Exit");
                            let mut continue_choice = String::new();
                            std::io::stdin().read_line(&mut continue_choice).unwrap();
                            clear_terminal();

                            match continue_choice.trim() {
                                "1" => {
                                    clear_terminal();
                                    print_header();
                                },
                                "2" => {
                                    clear_terminal();
                                    break;
                                },
                                _ => {
                                    println!("Invalid choice. Press Enter to continue.");
                                    let _ = std::io::stdin().read_line(&mut String::new());
                                    clear_terminal();
                                    print_header();
                                }
                            }
                        }
                        "3" => {
                            println!("Simple Run:");
                            println!("This option scans a GitHub repository and looks for commits.");
                            println!("It will output a list of URLs to these commits.");
                            println!("\nAdvanced Run:");
                            println!("This option allows you to specify the file you're looking for in the commits.");
                            println!("You can either provide a specific file name or leave it blank to scan for changes in all files.");
                            println!("\nExample: If you are looking for commits that exposed API keys in a file named 'config.js',");
                            println!("you would choose the Advanced Run and enter 'config.js' when prompted for the file name.");
                            let _ = std::io::stdin().read_line(&mut String::new());
                            clear_terminal();
                            print_header();
                        }
                        "4" => {
                            break;
                        }
                        _ => {
                            println!("Invalid choice. Please enter a number between 1 and 4. Press Enter to continue.");
                            let _ = std::io::stdin().read_line(&mut String::new());
                            clear_terminal();
                            print_header();
                        }
                    }
                }
            }
            "2" => {
                println!("This tool helps you find potentially sensitive information leaks on GitHub.");
                println!("It works by scanning the commits of a repository and checking if any commits modified a specific file.");
                println!("This can be helpful for finding instances where sensitive data like API keys or credentials might have been accidentally committed.");
                println!("");
                println!("To use the tool, simply choose the 'Run' option and follow the on-screen prompts.");
                println!("You can either perform a simple scan that checks the validity of generated URLs,");
                println!("or choose the advanced option to specify a different file name.");
                println!("");
                println!("The tool will output a list of URLs to the commits that match your criteria.");
                println!("Please use this tool responsibly and ethically.");
                let _ = std::io::stdin().read_line(&mut String::new());
                clear_terminal();
                print_header();
            }
            "3" => {
                println!("This GitHub Fork Hex Collector is a tool created by RobinHirst11.");
                println!("It is designed to help security researchers and developers identify potential information leaks on GitHub.");
                println!("By scanning for commits that include changes to specific files, this tool aids in discovering accidental");
                println!("exposures of sensitive data like API keys, credentials, or other confidential information.");
                println!("");
                println!("This tool is provided as-is and should be used responsibly and ethically.");
                println!("The creator is not responsible for any misuse or damage caused by this tool.");
                let _ = std::io::stdin().read_line(&mut String::new());
                clear_terminal();
                print_header();
            }
            "4" => {
                loop {
                    println!("Current hex length: {}", hex_length);

                    let mut new_hex_length = String::new();
                    println!("Enter new hex length (or type 'cancel'):");
                    std::io::stdin().read_line(&mut new_hex_length).unwrap();
                    clear_terminal();
                    print_header();

                    new_hex_length = new_hex_length.trim().to_string();

                    if new_hex_length.to_lowercase() == "cancel" {
                        break;
                    }

                    match new_hex_length.parse::<usize>() {
                        Ok(length) => {
                            hex_length = length;
                            println!("Hex length updated to: {}. Press Enter to continue.", hex_length);
                            let _ = std::io::stdin().read_line(&mut String::new());
                            clear_terminal();
                            print_header();
                            break;
                        },
                        Err(_) => {
                            println!("Invalid input. Please enter a number. Press Enter to continue.");
                            let _ = std::io::stdin().read_line(&mut String::new());
                            clear_terminal();
                            print_header();
                        }
                    }
                }
            }
            "5" => break,
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 5. Press Enter to continue.");
                let _ = std::io::stdin().read_line(&mut String::new());
                clear_terminal();
                print_header();
            }
        }
    }
}
