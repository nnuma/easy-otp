use chrono::{ Local, DateTime };
use hmac::{ Hmac, Mac };
use sha1::Sha1;

use crate::init::Config;

pub fn generate(config: Config) -> (String, i64) {
    // Create alias for HMAC-SHA256
    type HmacSha1 = Hmac<Sha1>;

    let (datetime_str, remaining_seconds) = get_current_datetime_str(&config);
    let mut mac = HmacSha1::new_from_slice(config.secret.as_bytes()).expect(
      "HMAC can take key of any size"
    );
    let s = format!("{}{}", config.user_id, datetime_str);
    mac.update(s.as_bytes());

    let result = mac.finalize();
    let s = format!("{:02x}", result.into_bytes());

    return (s[0..10].to_string(), remaining_seconds);
}

fn get_current_datetime_str(config: &Config) -> (String, i64) {
  let now: DateTime<Local> = Local::now();
  let minutes = now.timestamp_millis() / 1000 / 60 % 60;
  let interval_minutes = config.interval_minutes;
  let minutes_str = format!("{:0>2}", interval_minutes * (minutes / interval_minutes));
  let datetime_str = format!("{}{}", now.format("%Y%m%d%H").to_string(), minutes_str);
  println!("{}", datetime_str);
  let targ_datetime_str = now.format("%Y-%m-%dT%H:{}:00%:z").to_string().replace("{}", minutes_str.as_str()).to_string();
  let targ_datetime = DateTime::parse_from_rfc3339(&targ_datetime_str).unwrap();
  let diff = now.timestamp_millis() - targ_datetime.timestamp_millis();
  let remaining_seconds = (60 * config.interval_minutes) - (diff / 1000);

  return (datetime_str, remaining_seconds);
}
