use std::{fs::File, io::{BufReader, Read}};

const PATH: &str = "input_data/12302019.NASDAQ_ITCH50";


struct SystemEvent{
    stock_locate: u16,
    tracking_num: u16,
    timestamp:    u64,
    event_code:    u8,
}





fn handle_system_event(len: usize, buffer: Vec<u8>) {

}







fn read_message(len: usize, mut reader: BufReader<File>) {

    // reader here has already read length.
    // meaning we start nicely at the message type

    let mut message_buffer = vec![0u8;len];
    reader.read_exact(&mut message_buffer).unwrap();


    let message_type = message_buffer[0] as char;

    println!("{}", message_type);


    match message_type {
        'S' => handle_system_event(len, message_buffer),


        _ => println!("Nope"),
    }
}





fn parse_to_readable() -> Option<i32>{
    // reader and fixed size buffer for the length of of the length of the message
    let mut reader = BufReader::new(File::open(PATH).unwrap());
    let mut len_buf = [0u8;2];


    // read into buffer and represent that nicely
    reader.read_exact(&mut len_buf).unwrap();
    let len = u16::from_be_bytes(len_buf) as usize;


    read_message(len, reader);

    Some(32)

}










fn main() {
    
    parse_to_readable();
}


