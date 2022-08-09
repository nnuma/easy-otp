mod otp;
mod init;
use clipboard::{ClipboardProvider, ClipboardContext};

fn main() {
    let config = init::init();
    let (password,remaining_seconds) = otp::generate(config);
    println!("Password   : [ {} ] -> Copied to your clipboard!!", password);
    println!("Expires in : {} seconds.", remaining_seconds);
    let mut cb = ClipboardContext::new().unwrap();
    cb.set_contents(password).unwrap();
}