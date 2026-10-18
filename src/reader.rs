use std::{fs::File, io::{self, BufReader, ErrorKind, Read}};





pub fn next_message(reader: &mut BufReader<File>, buf: &mut Vec<u8>) -> io::Result<bool> {

    let mut len_buf = [0u8;2];

    match reader.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(false),
        Err(e) => return Err(e),
    }

    let len = u16::from_be_bytes(len_buf) as usize;

    buf.resize(len, 0);
    reader.read_exact(buf)?;

    Ok(true)
}

