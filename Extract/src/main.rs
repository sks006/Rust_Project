

use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::io;
use zip::ZipArchive; // Fixed import for ZipArchive.

fn main() {
    // Properly handle exit code by calling the logical_main function.
    std::process::exit(logical_main());
}

fn logical_main() -> i32 {
    // Collect arguments from the command line.
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        // Inform the user of proper usage if not enough arguments are supplied.
        println!("Usage: {} <filename>", args[0]);
        return 1;
    };

    let fname = std::path::Path::new(&args[1]); // Removed redundant dereferencing.
    let file = match fs::File::open(&fname) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err); // Handle file open errors.
            return 1;
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => {
            eprintln!("Error reading ZIP archive: {}", err); // Handle ZIP file read errors.
            return 1;
        }
    };

    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error accessing file in archive: {}", err); // Handle errors for individual entries.
                continue;
            }
        };

        let output = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => {
                eprintln!("Invalid path for file {}", i);
                continue;
            }
        };

        if !file.comment().is_empty() {
            println!("File {} comment: {}", i, file.comment()); // Display file comments if available.
        }

        if file.name().ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output.display());
            if let Err(err) = create_dir_all(&output) {
                eprintln!("Error creating directory: {}", err); // Handle directory creation errors.
                continue;
            }
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output.display(),
                file.size()
            );

            if let Some(p) = output.parent() {
                if !p.exists() {
                    if let Err(err) = create_dir_all(p) {
                        eprintln!("Error creating parent directory: {}", err); // Handle parent directory creation errors.
                        continue;
                    }
                }
            }

            let mut outfile = match fs::File::create(&output) {
                Ok(outfile) => outfile,
                Err(err) => {
                    eprintln!("Error creating output file: {}", err); // Handle file creation errors.
                    continue;
                }
            };

            if let Err(err) = io::copy(&mut file, &mut outfile) {
                eprintln!("Error copying content: {}", err); // Handle file write errors.
                continue;
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    if let Err(err) = fs::set_permissions(&outfile, fs::Permissions::from_mode(mode))
                    {
                        eprintln!("Error setting permissions: {}", err); // Handle permission setting errors.
                    }
                }
            }
        }
    }

    0 // Return success code if all operations complete.
}
