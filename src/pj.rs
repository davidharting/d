use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize};
use serde_json;

pub fn extract_scripts_from_package_json() -> Result<HashMap<String, String>, PackageJsonError> {
  let contents = read()?;
  match parse(&contents) {
    Ok(package_json) => {
      Ok(package_json.scripts) 
    },
    Err(e) => {
      Err(PackageJsonError::Parse(e))
    }
  }
}


fn parse(contents: &str) -> Result<PackageJson, serde_json::Error> {
  serde_json::from_str(contents)
}

fn read() -> Result<String, PackageJsonError> {
  let path = Path::new("./package.json");
  if !path.exists() {
    return Err(PackageJsonError::FileNotFound);
  }
  
  match fs::read_to_string("./package.json") {
    Ok(contents) => Ok(contents),
    Err(e) => Err(PackageJsonError::UnableToReadFile(e))
  }
}

pub enum PackageJsonError {
  // No package.json in current directory
  FileNotFound,
  UnableToReadFile(std::io::Error),
  Parse(serde_json::Error),
}

#[derive(Deserialize)]
struct PackageJson {
  scripts: HashMap<String, String>
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_detects_missing_file() -> Result<(),  &'static str> {
    match read() {
      Ok(_) => {
        Err("Should return an error when there is no package.json in the current directory.")
      },
      Err(pj_err) => {
        match pj_err {
          PackageJsonError::FileNotFound => { Ok(()) },
          _ => {
            Err("Should return a FileNotFound error")
          }
        }
      }
    }
  }

  #[test]
  fn parse_parses_scripts_from_valid_package_json() {
    let contents = r#"{
      "name": "My cool project",
      "version": "1.0.1",
      "scripts": {
        "run": "node index.js",
        "setup": "./scripts/setup.sh"
      },
      "deps": {}
    }"#;
    match parse(contents) {
      Ok(package_json) => {
        assert_eq!(package_json.scripts.get("run").unwrap(), "node index.js");
        assert_eq!(package_json.scripts.get("setup").unwrap(), "./scripts/setup.sh");
        assert_eq!(package_json.scripts.len(), 2);
      },
      Err(e) => {
        panic!("Could not parse package.json.\n{}", e)
      }
    }
  }

  #[test]
  fn parse_fails_to_parse_invalid_json() -> Result<(), &'static str> {
    let contents = r#"{
      "name: "My cool project",
      "version": "1.0.1",
      "scripts": {
        "run": "node index.js",
        "setup": "./scripts/setup.sh"
      },
      "deps": {},
    }"#;

    match parse(contents) {
      Ok(_) => {
        Err("Contents should not have parsed.")
      },
      Err(_) => {
        Ok(())
      }
    }
  }

  #[test]
  fn parse_fails_to_parse_if_scripts_are_missing() -> Result<(), &'static str> {
    let contents = r#"{
      "name: "My cool project",
      "version": "1.0.1",
      "deps": {},
    }"#;
    
    match parse(contents) {
      Ok(_) => {
        Err("Contents should not have parsed.")
      },
      Err(_) => {
        Ok(())
      }
    }
  }
}