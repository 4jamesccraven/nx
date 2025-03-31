use super::{change_to_config, run_command};

use std::fs;

pub fn update(shells: bool) -> Result<(), String> {
    change_to_config()?;

    if !shells {
        run_command("nix flake update")?;
        run_command("nx build --fast")?;
    }

    let mut shells_to_cache: Vec<String> = Vec::new();

    let directory = fs::read_dir("./shells")
        .map_err(|e| format!("error reading from shells directory {}", e))?;

    for file in directory {
        let file = match file {
            Ok(file) => file,
            Err(_) => continue,
        };
        let file_path = file.path();

        let ext = match file_path.as_path().extension() {
            Some(ext) => ext.to_str().expect("literally how could this fail"),
            None => continue,
        };

        if matches!(ext, "nix") {
            let content = match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(_) => continue,
            };

            if content.contains("# CACHE") && file_path.file_stem().is_some() {
                shells_to_cache.push(file_path.file_stem().unwrap().to_str().unwrap().into());
            }
        }
    }

    for shell in shells_to_cache {
        let shell_path = format!("/home/jamescraven/nixos#{}", shell);
        let command = format!("nix develop {shell_path} --command echo");
        run_command(command.as_ref())?;
    }

    Ok(())
}
