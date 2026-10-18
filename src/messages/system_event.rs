use self::SystemEventCodes::{EndMarketHour, EndMessages, EndSystemHour, StartMarketHour, StartMessages, StartSystemHour, UnknownEventCode};


#[derive(Debug)]
pub enum SystemEventCodes {
    StartMessages,
    StartSystemHour,
    StartMarketHour,
    EndMarketHour,
    EndSystemHour,
    EndMessages,
    UnknownEventCode,
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
            _   => UnknownEventCode,
        }
    }


    pub fn as_str(&self) -> &'static str {
        match self {
            StartMessages   => "Start of messages",
            StartSystemHour => "Start of System hours",
            StartMarketHour => "Start of market hours",
            EndMarketHour   => "End of market hours",
            EndSystemHour   => "End of system hours",
            EndMessages     => "End of messages",
            UnknownEventCode => "Unknown event code"
        }
    }
}




#[derive(Debug)]
pub struct SystemEvent {
    stock_locate: u16,
    tracking_num: u16,
    pub timestamp:    u64,
    pub event_code:   SystemEventCodes,
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




pub fn handle_system_event(buffer: &[u8]) -> SystemEvent {
    let stock_locate = u16::from_be_bytes(buffer[1..3].try_into().unwrap());
    let tracking_num = u16::from_be_bytes(buffer[3..5].try_into().unwrap());

    let mut ts_bytes = [0u8;8];
    ts_bytes[2..8].copy_from_slice(&buffer[5..11]);
    let timestamp = u64::from_be_bytes(ts_bytes);

    let event_code = SystemEventCodes::parse_char_to_event_code(buffer[11] as char);

    SystemEvent::new(stock_locate, tracking_num, timestamp, event_code)

}