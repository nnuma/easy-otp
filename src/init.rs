use dirs::home_dir;
use std::{fs::{OpenOptions, create_dir_all}, io::Read, io::{stdin, Write, stdout}, path::Path};
use serde::{Serialize, Deserialize};
use toml::{to_string, from_str};

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub secret: String,
  pub user_id: String,
  pub interval_minutes: i64,
}

impl Config {

  fn new(path: &Path) -> Self {
    let dir_path = path.parent().unwrap();
    create_dir_all(dir_path).expect("Something went wrong when creating a directory.");
    OpenOptions::new().write(true).create(true).open(path).expect("Something went wrong when creating a file.");

    let mut file = OpenOptions::new().read(true).open(path).expect("Something went wrong when reading a file.");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error");
    return match from_str(buf.as_str()) {
      Ok(r) => r,
      Err(_) => {
        Self {
          secret: String::new(),
          user_id: String::new(),
          interval_minutes: 0,
        }
      }
    };
  }

  fn initted(&self) -> bool {
    return !self.secret.is_empty() && !self.user_id.is_empty() && self.interval_minutes.is_positive();
  }

  fn init(&mut self, path: &Path) {

    // Secret key.
    print!("Enter your secret key: ");
    stdout().flush().unwrap();
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Please specify your secret key.");

    // User ID.
    print!("Enter your user name: ");
    stdout().flush().unwrap();
    let mut u = String::new();
    stdin().read_line(&mut u).expect("Please specify your user name.");

    // Interval in minutes to re-generate an OTP.
    print!("Enter an interval in minutes: ");
    stdout().flush().unwrap();
    let mut i = String::new();
    stdin().read_line(&mut i).expect("Please specify an interval.");

    self.secret = s.trim_end().to_string();
    self.user_id = u.trim_end().to_string();
    self.interval_minutes = i.trim_end().parse().expect("Please specify an interval in minutes.");
    
    let mut file = OpenOptions::new().write(true).create(true).open(path).expect("Something went wrong when opening a file.");
    file.write_all(to_string(self).unwrap().as_bytes()).expect("Something went wrong when writing a file.");
  }
}

pub fn init() -> Config {
  let mut path_buf = home_dir().unwrap();
  path_buf.push(".easy-otp");
  path_buf.push("config.toml");
  let path = path_buf.as_path();

  let mut config = Config::new(path);
  if !config.initted() {
    config.init(path);
  }
  return config;
}