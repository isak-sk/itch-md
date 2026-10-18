use std::{fs::File, io::{self, BufReader, ErrorKind, Read}};

use crate::SystemEventCodes::{EndMarketHour, EndMessages, EndSystemHour, StartMarketHour, StartMessages, StartSystemHour, UnkownEventCode};

const PATH: &str = "input_data/12302019.NASDAQ_ITCH50";


enum MessageTypes {
    SystemEvent,
    StockMessage,
}

#[derive(Debug)]
enum SystemEventCodes {
    StartMessages,
    StartSystemHour,
    StartMarketHour,
    EndMarketHour,
    EndSystemHour,
    EndMessages,
    UnkownEventCode,
}


impl SystemEventCodes {


    fn parse_char_to_event_code(event_code: char) -> Self {
        match event_code {
            'O' => StartMessages,
            'S' => StartSystemHour,
            'Q' => StartMarketHour,
            'M' => EndMarketHour,
            'E' => EndSystemHour,
            'C' => EndMessages,
            _   => UnkownEventCode,
        }
    }


    fn as_str(&self) -> &'static str {
        match self {
            StartMessages   => "Start of messages",
            StartSystemHour => "Start of System hours",
            StartMarketHour => "Start of market hours",
            EndMarketHour   => "End of market hours",
            EndSystemHour   => "End of system hours",
            EndMessages     => "End of messages",
            UnkownEventCode => "Unkown event code"
        }
    }
}




#[derive(Debug)]
struct SystemEvent {
    stock_locate: u16,
    tracking_num: u16,
    timestamp:    u64,
    event_code:   SystemEventCodes,
}


impl SystemEvent {

    fn new(stock_locate: u16, tracking_num: u16, timestamp: u64, event_code: SystemEventCodes) -> Self {
        Self {
            stock_locate,
            tracking_num,
            timestamp,
            event_code
        }
    }
}




fn handle_system_event(buffer: &[u8]) -> SystemEvent {
    let stock_locate = u16::from_be_bytes(buffer[1..3].try_into().unwrap());
    let tracking_num = u16::from_be_bytes(buffer[3..5].try_into().unwrap());

    let mut ts_bytes = [0u8;8];
    ts_bytes[2..8].copy_from_slice(&buffer[5..11]);
    let timestamp = u64::from_be_bytes(ts_bytes);

    let event_code = SystemEventCodes::parse_char_to_event_code(buffer[11] as char);

    SystemEvent::new(stock_locate, tracking_num, timestamp, event_code)

}







fn read_message(len: usize, reader: &mut BufReader<File>) -> Option<SystemEvent> {

    let mut message_buffer = vec![0u8;len];
    reader.read_exact(&mut message_buffer).unwrap();


    let message_type = message_buffer[0] as char;


    match message_type {
        'S' => {
            Some(handle_system_event(&message_buffer))
        }

        _ => None
    }

}





fn parse_to_readable() -> io::Result<Vec<SystemEvent>> {
    // reader and fixed size buffer for the length of of the length of the message

    let mut events = Vec::new();

    let mut reader = BufReader::new(File::open(PATH).unwrap());
    let mut len_buf = [0u8;2];

    loop {
        match reader.read_exact(&mut len_buf) {
            Ok(()) => {
                let len = u16::from_be_bytes(len_buf) as usize;
                if let Some(ev) = read_message(len, &mut reader) {
                    events.push(ev);
                };
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => {
                return Err(e);
            }
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


