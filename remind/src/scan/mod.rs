// r2s -> remind to send
use abi::note::Note;
use anyhow::Result;
use std::path::Path;
use tracing::error;
use walkdir::WalkDir;

/*
scan the notes path find all notes with the file_suffix, and return a vector of Note
*/
pub async fn scan_notes<P: AsRef<Path>>(paths: Vec<P>, file_suffix: &str) -> Result<Vec<Note>> {
    let mut notes = Vec::new();
    for path in paths {
        for entry in WalkDir::new(path) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let p = entry.path().to_string_lossy();
                    if p.ends_with(file_suffix) {
                        let note = Note {
                            note_name: entry.file_name().to_string_lossy().to_string(),
                            note_suffix: file_suffix.to_string(),
                            note_path: p.to_string(),
                        };
                        notes.push(note);
                    }
                }
            } else {
                error!("scan_notes error: {:?}", entry.err().unwrap());
            }
        }
    }
    Ok(notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_dir() {
        let mut entries = tokio::fs::read_dir(".").await.unwrap();
        while let Some(entry) = entries.next_entry().await.unwrap() {
            println!("{:?}, {}", entry.file_name(), entry.ino());
        }
    }

    #[tokio::test]
    async fn test_scan_notes() {
        let paths = vec!["./"];
        let notes = scan_notes(paths, ".rs").await.unwrap();
        for note in notes {
            println!("{:?}", note);
        }
    }
}
