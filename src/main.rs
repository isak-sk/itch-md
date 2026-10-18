mod reader;
mod messages;

use std::{fs::File, io::{self, BufReader}};

use crate::reader::next_message;
use crate::messages::system_event::SystemEvent;

const PATH: &str = "input_data/12302019.NASDAQ_ITCH50";



fn parse_to_readable() -> io::Result<Vec<SystemEvent>> {
    let mut reader = BufReader::new(File::open(PATH)?);
    let mut buf = Vec::new();
    let mut events = Vec::new();

    while next_message(&mut reader, &mut buf)? {
        if let Some(ev) = messages::parse(&buf) {
            events.push(ev);
        }
    }

    Ok(events)
}







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
