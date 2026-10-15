
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Detect whether preview mode is enabled.
    let dry_run = args.iter().any(|arg| arg == "--dry-run");

    // Find the folder path, ignoring command-line flags.
    let folder_arg = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with("--"));

    let folder_arg = match folder_arg {
        Some(path) => path,
        None => {
            println!(
                "Usage: cargo run --bin file_organizer -- \
                 <folder_path> [--dry-run]"
            );
            return;
        }
    };

    let target = PathBuf::from(folder_arg);

    // Validate the target folder.
    if !target.is_dir() {
        println!("Error: The specified folder does not exist.");
        return;
    }

    // Organize files or preview the planned movements.
    match organize_folder(&target, dry_run) {
        Ok(count) => {
            if dry_run {
                println!(
                    "Dry run complete! {} files would be moved.",
                    count
                );
            } else {
                println!("Done! Organized {} files.", count);
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn organize_folder(folder: &Path, dry_run: bool) -> io::Result<usize> {
    let mut moved = 0;

    // Read the entries directly inside the target folder.
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        // Ignore directories and other non-file entries.
        if !path.is_file() {
            continue;
        }

        // Extract and normalize the file extension.
        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext.to_ascii_lowercase(),
            None => continue,
        };

        // Choose a category based on the file extension.
        let category = match extension.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "webp" => "Images",
            "pdf" | "doc" | "docx" | "txt" => "Documents",
            "mp3" | "wav" | "flac" => "Audio",
            "mp4" | "mkv" | "mov" => "Videos",
            "zip" | "rar" | "7z" => "Archives",
            _ => "Others",
        };

        let destination_dir = folder.join(category);

        // Only create destination folders in normal mode.
        if !dry_run {
            fs::create_dir_all(&destination_dir)?;
        }

        let file_name = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        let destination = destination_dir.join(file_name);

        // Never overwrite an existing file.
        if destination.exists() {
            println!(
                "Skipping duplicate filename: {}",
                file_name.to_string_lossy()
            );
            continue;
        }

        // Preview the move or perform it.
        if dry_run {
            println!(
                "[DRY RUN] Would move: {} -> {}",
                path.display(),
                destination.display()
            );

            moved += 1;
        } else {
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
                    println!(
                        "Could not move {}: {}",
                        path.display(),
                        error
                    );
                }
            }
        }
    }

    Ok(moved)
}
