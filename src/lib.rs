use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  for line in search(&config.query, &contents) {
    println!("{line}");
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  let query_lower_case = query.to_lower_case();
  
  for line in contents.lines() {
    if line.to_lower_case().contains(&query) {
      results.push(line);
    }
  }

  results
}

pub struct Config {
  pub query: String,
  pub file_path: String
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len() <= 2 {
          return Err("Not enough arguments.");
      }

      Ok(
          Config {
              query: args[1].clone(),
              file_path: args[2].clone()
          }
      )

  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_insensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_sensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";  
    
    assert_eq!(vec!["Rust:","Trust me."], search_case_insensitive(query, contents));
  }
}