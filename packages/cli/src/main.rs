// TODO: make shared have the same structure as the repo.

const API_URL: &str = "https://api.github.com";
const REPO: &str = "m1ten/shadcn-svelte-plus";
const USER_AGENT: &str = "shadcn-svelte-plus";

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let cmd = match args.get(1) {
        Some(cmd) => cmd,
        None => {
            println!("Usage: shadcn-svelte-plus <command>");
            return;
        }
    };

    match cmd.as_str() {
        "init" => {
            println!("Not implemented yet");
        }
        "add" => {
            let component = match args.get(2) {
                Some(component) => component,
                None => {
                    println!("Usage: shadcn-svelte-plus add <component>");
                    return;
                }
            };

            println!("Adding component: {}", component);
            println!("Continue? This will overwrite existing files. [y/N]");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() != "y" {
                eprintln!("Aborted");
                return;
            }

            let url = format!(
                "{}/repos/{}/contents/shared/ui/{}",
                API_URL, REPO, component
            );

            let client = reqwest::Client::new();

            let res = match client
                .get(&url)
                .header("User-Agent", USER_AGENT)
                .send()
                .await
            {
                Ok(res) => res,
                Err(err) => {
                    println!("Failed to fetch component: {}", err);
                    return;
                }
            };

            if res.status() == 404 {
                println!("Component not found: {}", component);
                return;
            }

            let body = match res.text().await {
                Ok(body) => body,
                Err(err) => {
                    println!("Failed to read response body: {}", err);
                    return;
                }
            };

            let json: serde_json::Value = match serde_json::from_str(&body) {
                Ok(json) => json,
                Err(err) => {
                    println!("Failed to parse response body: {}", err);
                    return;
                }
            };

            for file in match json.as_array() {
                Some(array) => array,
                None => {
                    println!("Failed to parse response body: not an array");
                    return;
                }
            } {
                let name = match file["name"].as_str() {
                    Some(name) => name,
                    None => {
                        println!("Failed to parse response body: not a string");
                        return;
                    }
                };

                let download_url = match file["download_url"].as_str() {
                    Some(url) => url,
                    None => {
                        println!("Failed to parse response body: not a string");
                        return;
                    }
                };

                println!("Downloading: {}", name);
                println!("From: {}", download_url);

                let res = match client
                    .get(download_url)
                    .header("User-Agent", USER_AGENT)
                    .send()
                    .await
                {
                    Ok(res) => res,
                    Err(err) => {
                        println!("Failed to fetch component: {}", err);
                        return;
                    }
                };

                let body = match res.bytes().await {
                    Ok(body) => body,
                    Err(err) => {
                        println!("Failed to read response body: {}", err);
                        return;
                    }
                };

                let dir = format!("src/lib/components/ui/{}", component);
                let path = format!("{}/{}", dir, name);

                match tokio::fs::create_dir_all(dir).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Failed to create directory: {}", err);
                        return;
                    }
                }

                match tokio::fs::write(&path, body).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Failed to write file: {}", err);
                        return;
                    }
                }

                println!("Done");
            }
        }
        _ => {
            println!("Unknown command: {}", cmd);
        }
    }
}
