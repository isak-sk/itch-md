pub mod system_event;


use system_event::SystemEvent;


pub fn parse(buf: &[u8]) -> Option<SystemEvent> {
    match buf[0] {
        b'S' => Some(system_event::handle_system_event(buf)),
        _    => None,
    }
}
