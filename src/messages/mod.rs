pub mod system_event;
pub mod stock_messages;

use system_event::SystemEvent;
use stock_messages::StockMessage;


pub enum Message {
    SystemEvent(SystemEvent),
    StockMessage(StockMessage),
}


pub fn parse(buf: &[u8]) -> Option<Message> {

    match buf[0] {

        b'S' => {
            let event = system_event::handle_system_event(buf);
            Some(Message::SystemEvent(event))
        }
        b'R' => {
            let message = stock_messages::handle_stock_messages(buf);
            Some(Message::StockMessage(message))
        }
        _    => None,
    }



}

