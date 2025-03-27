use clap::Parser;
use lvm_multi_api::cli::Cli;
use std::path::Path;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let provider = args.get_provider();
    let image = provider.text_to_image(args.request).await;
    match image {
        Ok(images) => {
            for img in images {
                // Save the image to a file with the current timestamp
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_else(|_| {
                        eprintln!("Failed to get current timestamp");
                        std::process::exit(1);
                    })
                    .as_secs();
                let filename = format!("image_{}.png", timestamp);
                let filename = Path::new(&filename);
                img.to_file(filename).unwrap_or_else(|_| {
                    eprintln!("Failed to save image to file");
                    std::process::exit(1);
                });
                println!("Image saved to: {}", filename.to_string_lossy());
            }
        }
        Err(e) => {
            eprintln!("Error generating image: {}", e);
        }
    }
}
