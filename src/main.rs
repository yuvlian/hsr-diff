mod hpatchz;
mod diff_type;

use std::fs;
use std::process::Command;
use std::path::Path;
use std::io::{self, Write};
use diff_type::DiffMap;

fn wait_for_exit_benched(time: std::time::Instant) {
    println!("Elapsed time: {}", time.elapsed().as_secs_f32());
    print!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

fn wait_for_exit() {
    print!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

fn delete_file_if_exists(path: &Path) -> io::Result<()> {
    if path.exists() {
        println!("[INFO] Deleting {:?}", path);
        fs::remove_file(path)?;
    }
    Ok(())
}

fn execute_patch(source: &Path, patch: &Path, target: &Path, hpatchz_path: &Path) -> io::Result<()> {
    let status = Command::new(hpatchz_path)
        .arg(source)
        .arg(patch)
        .arg(target)
        .status()?;
    
    if !status.success() {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to run hpatchz.exe"))
    } else {
        Ok(())
    }
}

fn main() {
    let folder_path = rfd::FileDialog::new().set_directory(".").pick_folder();

    let game_folder = match folder_path {
        Some(path) => path,
        None => {
            println!("[ERROR] No folder selected.");
            wait_for_exit();
            return;
        }
    };

    let timer = std::time::Instant::now();

    let temp_hpatchz_path = std::env::temp_dir().join("hpatchz.exe");
    let delete_files_path = game_folder.join("deletefiles.txt");
    let hdiff_map_path = game_folder.join("hdiffmap.json");

    if !delete_files_path.exists() {
        println!("[ERROR] deletefiles.txt doesn't exist in the game directory!");
        wait_for_exit_benched(timer);
        return;
    }

    if !hdiff_map_path.exists() {
        println!("[ERROR] hdiffmap.json doesn't exist in the game directory!");
        wait_for_exit_benched(timer);
        return;
    }

    match fs::read_to_string(&delete_files_path) {
        Ok(content) => {
            for file in content.lines() {
                if file.trim().is_empty() {
                    continue;
                }

                let file_path = game_folder.join(file);
                if let Err(e) = delete_file_if_exists(&file_path) {
                    println!("[ERROR] Failed to delete file {:?}: {}", file_path, e);
                }
            }
        }
        Err(e) => {
            println!("[ERROR] Failed to read deletefiles.txt: {}", e);
            wait_for_exit_benched(timer);
            return;
        }
    }

    let hdiff_map_content = match fs::read_to_string(&hdiff_map_path) {
        Ok(content) => content,
        Err(e) => {
            println!("[ERROR] Failed to read hdiffmap.json: {}", e);
            wait_for_exit_benched(timer);
            return;
        }
    };

    let diff_map: DiffMap = match serde_json::from_str(&hdiff_map_content) {
        Ok(json) => json,
        Err(e) => {
            println!("[ERROR] Invalid JSON format in hdiffmap.json: {}", e);
            wait_for_exit_benched(timer);
            return;
        }
    };

    if let Err(e) = fs::write(&temp_hpatchz_path, hpatchz::BYTES) {
        println!("[ERROR] Failed to write hpatchz.exe: {}", e);
        wait_for_exit_benched(timer);
        return;
    }

    for diff in diff_map.diff_map {
        let source_file_path = game_folder.join(&diff.source_file_name);
        let patch_file_path = game_folder.join(&diff.patch_file_name);
        let target_file_path = game_folder.join(&diff.target_file_name);

        println!(
            "[INFO] Patching {:?} -> {:?} -> {:?}",
            source_file_path, patch_file_path, target_file_path
        );

        if let Err(e) = execute_patch(&source_file_path, &patch_file_path, &target_file_path, &temp_hpatchz_path) {
            println!("[ERROR] Failed to run hpatchz.exe: {}", e);
            break;
        }
    }

    if let Err(e) = delete_file_if_exists(&temp_hpatchz_path) {
        println!("[ERROR] Failed to remove hpatchz.exe: {}", e);
    }

    wait_for_exit_benched(timer);
}
