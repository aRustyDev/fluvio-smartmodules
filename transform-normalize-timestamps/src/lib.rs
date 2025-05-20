mod time_patterns;
// use chrono::{DateTime, Utc};
use fluvio_smartmodule::dataplane::smartmodule::SmartModuleExtraParams;
use fluvio_smartmodule::{RecordData, Result, SmartModuleRecord, smartmodule};

#[smartmodule(map)]
pub fn map(record: &SmartModuleRecord) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let string = std::str::from_utf8(record.value.as_ref())?;
    let int = string.parse::<i32>()?;
    let value = (int * 2).to_string();

    Ok((key, value.into()))
}

#[smartmodule(init)]
fn init(_params: SmartModuleExtraParams) -> Result<()> {
    // You can refer to the example SmartModules in Fluvio's GitHub Repository
    // https://github.com/infinyon/fluvio/tree/master/smartmodule
    todo!("Provide initialization logic for your SmartModule")
}

// fn main() {
//     let current_utc: DateTime<Utc> = Utc::now();
//     let rfc_format: String = current_utc.to_rfc3339();
//     println!("{}", rfc_format); // Outputs: 2023-10-03T13:39:01.385491+00:00
// }
