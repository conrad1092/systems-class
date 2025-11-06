use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum DogError {
    Api(String),
    Network(String),
    File(String),
}

fn fetch_random_dog_image() -> Result<DogImage, DogError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    let response = ureq::get(url)
        .call()
        .map_err(|e| DogError::Network(format!("Request failed: {}", e)))?;

    if response.status() != 200 {
        return Err(DogError::Api(format!(
            "HTTP error from API: {}",
            response.status()
        )));
    }

    response
        .into_json::<DogImage>()
        .map_err(|e| DogError::Api(format!("Failed to parse JSON: {}", e)))
}

fn download_image(url: &str, filename: &str) -> Result<(), DogError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| DogError::Network(format!("Image request failed: {}", e)))?;

    if response.status() != 200 {
        return Err(DogError::Api(format!(
            "HTTP error while downloading image: {}",
            response.status()
        )));
    }

    let mut reader = response.into_reader();
    let mut bytes = Vec::new();
    reader
        .read_to_end(&mut bytes)
        .map_err(|e| DogError::File(format!("Failed to read image bytes: {}", e)))?;

    let mut file = File::create(filename)
        .map_err(|e| DogError::File(format!("Failed to create file {}: {}", filename, e)))?;

    file.write_all(&bytes)
        .map_err(|e| DogError::File(format!("Failed to write file {}: {}", filename, e)))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader");
    println!("====================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);

        match fetch_random_dog_image() {
            Ok(dog_image) => {
                println!("✅ API Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                let filename = format!("dog_{}.jpg", i);
                println!("📥 Downloading to: {}", filename);

                match download_image(&dog_image.message, &filename) {
                    Ok(()) => println!("✅ Saved image as {}\n", filename),
                    Err(e) => println!("❌ Error saving image: {:?}\n", e),
                }
            }
            Err(e) => {
                println!("❌ Error talking to API: {:?}\n", e);
            }
        }
    }

    Ok(())
}
