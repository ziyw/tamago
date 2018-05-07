
// message types defined in com.h
// current test use 0 -> just receive 
pub struct Message {
    endpoint: u32,
    message_type: u32,
    payload: MessageType, 
}

pub enum MessageType {
    mess_u16(Mess_u16),
}

pub struct Mess_u16{
    data: [u16; 28],
}

