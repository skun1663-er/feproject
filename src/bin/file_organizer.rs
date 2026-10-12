
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run --bin file_organizer -- <folder_path>");
        return;
    }

    let target = PathBuf::from(&args[1]);

    if !target.is_dir() {
        println!("Error: The specified folder does not exist.");
        return;
    }

    match organize_folder(&target) {
        Ok(count) => println!("Done! Organized {} files.", count),
        Err(error) => println!("Error: {}", error),
    }
}

fn organize_folder(folder: &Path) -> io::Result<usize> {
    let mut moved = 0;

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext.to_ascii_lowercase(),
            None => continue,
        };

        let category = match extension.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "webp" => "Images",
            "pdf" | "doc" | "docx" | "txt" => "Documents",
            "mp3" | "wav" | "flac" => "Audio",
            "mp4" | "mkv" | "mov" => "Videos",
            "zip" | "rar" | "7z" => "Archives",
            _ => "Others",
        };

        let destination_dir = folder.join(category);
        fs::create_dir_all(&destination_dir)?;

        let file_name = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        let mut destination = destination_dir.join(file_name);

        if destination.exists() {
            println!("Skipping duplicate filename: {}", file_name.to_string_lossy());
            continue;
        }

        // Rename moves the file on the same filesystem.
        match fs::rename(&path, &destination) {
            Ok(()) => {
                println!(
                    "Moved: {} -> {}",
                    path.display(),
                    destination.display()
                );
                moved += 1;
            }
            Err(error) => {
                println!("Could not move {}: {}", path.display(), error);
            }
        }
    }

    Ok(moved)
}
