use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[derive(Deserialize)]
struct ApiResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<Content>,
}

#[derive(Deserialize)]
struct Content {
    parts: Option<Vec<Part>>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Fetch the API key from the environment
    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set in the .env file");

    // Define the API endpoint
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        api_key
    );

    let text = "Explain how AI works";

    // Create the JSON body for the request
    let request_body = json!({
        "contents": [
            {
                "role": "user",
                "parts": [
                    {
                        "text": text
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 1,
            "topK": 40,
            "topP": 0.95,
            "maxOutputTokens": 8192,
            "responseMimeType": "text/plain"
        }
    });

    // Create a reqwest client
    let client = Client::new();

    // Send the POST request
    let response = client
        .post(&url) // Set the URL
        .header("Content-Type", "application/json") // Set the Content-Type header
        .json(&request_body) // Attach the JSON body
        .send() // Send the request
        .await?; // Await the response and handle errors

    // Check if the response is successful
    if response.status().is_success() {
        // Parse and print the response content
        let response_text = response.text().await?;

        // Deserialize the response JSON into a struct
        let api_response: ApiResponse = serde_json::from_str(&response_text)?;

        // Extract and print the content text
        if let Some(candidates) = api_response.candidates {
            for candidate in candidates {
                if let Some(content) = candidate.content {
                    if let Some(parts) = content.parts {
                        for part in parts {
                            println!("Response: {}", part.text);
                        }
                    }
                }
            }
        } else {
            eprintln!("No candidates found in the response");
        }
    } else {
        // Handle the error
        eprintln!("Failed to send request. Status: {}", response.status());
        let error_text = response.text().await?;
        eprintln!("Error: {}", error_text);
    }

    Ok(())
}
