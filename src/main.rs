#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
use std::process;
use std::path::Path;
use inline_colorization::*;
fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let config = nu_hist::Config::new(&args).unwrap_or_else(|err| {
    eprintln!("{color_red}Problem parsing arguments: {}", 
    err
  );
    process::exit(1);
  });

  let path = Path::new(&config.path);
  if !path.exists() {
    eprintln!("{color_red}File does not exist: {}", config.path);
    process::exit(1);
  } else if config.analysis == "all" {
    let conn: Connection = Connection::open(&config.path)?;
    nu_hist::all(conn)?;
  } else if config.analysis.parse::<i32>().is_ok() {
    let conn: Connection = Connection::open(&config.path)?;
    nu_hist::year(conn, config.analysis)?;
  } else {
    eprintln!("Invalid year: {}", config.analysis);
    process::exit(1);
  }
  return Ok(());
}
