mod reader;
mod messages;

use std::io;

use crate::reader::parse_to_readable;

const PATH: &str = "input_data/12302019.NASDAQ_ITCH50";


fn main() -> io::Result<()>{
    let events = parse_to_readable()?;

    for ev in &events {

        let total_sec = ev.timestamp / 1_000_000_000;
        let h = total_sec / 3600;
        let m = (total_sec % 3600) / 60;
        let s = total_sec % 60;

        println!("{:02}:{:02}:{:02}  {}", h, m, s, ev.event_code.as_str());
    }

    Ok(())
}