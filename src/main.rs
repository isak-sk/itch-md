mod messages;
mod reader;

use crate::messages::Message;
use crate::reader::next_message;
use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write},
};

const PATH: &str = "input_data/12302019.NASDAQ_ITCH50";

fn parse_to_readable() -> io::Result<Vec<Message>> {
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

fn main() -> io::Result<()> {
    let events = parse_to_readable()?;
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for ev in &events {
        match ev {
            Message::SystemEvent(se) => {
                writeln!(writer, "{}", se.format_summary())?;
            }
            Message::StockMessage(sm) => {
                writeln!(writer, "{}", sm.format_summary())?;
            }
        }
    }

    writer.flush()?;
    Ok(())
}
