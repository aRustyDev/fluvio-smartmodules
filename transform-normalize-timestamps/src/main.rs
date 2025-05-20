use chrono::{DateTime, Utc};
​
fn main() {
    let current_utc: DateTime<Utc> = Utc::now();
    let rfc_format: String = current_utc.to_rfc3339();
    println!("{}", rfc_format); // Outputs: 2023-10-03T13:39:01.385491+00:00
}
