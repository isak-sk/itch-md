use std::{fs::File, io::{self, BufReader, ErrorKind, Read}};

use crate::PATH;
use crate::messages::system_event::{handle_system_event, SystemEvent};


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


pub fn parse_to_readable() -> io::Result<Vec<SystemEvent>> {
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