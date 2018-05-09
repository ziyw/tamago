// test place 

use core::ptr;

pub struct Proc {
    p_id: usize,
    status: u8, // 0 ready, 1 running, 2 blocked
    message_buffer: Message,
}

impl Proc {
    pub fn init(p_id: usize) -> Proc {
        Proc {
            p_id: p_id,
            status: 0,
            message_buffer: Message {
                endpoint: p_id as u32, 
                message_type: 0,
                payload: MessageType::mess_u16 { data: [0; 28], } 
            }
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Message {
    endpoint: u32,
    message_type: u32,
    payload: MessageType, 
}

#[derive(Copy, Clone, Debug)]
pub enum MessageType {
    mess_u16 { data: [u16; 28] },
}

pub fn send(sender: &mut Proc, receiver: &mut Proc, message: Message) {

    if receiver.status == 2 {
        receiver.message_buffer = message; 
    }

    else {
        sender.status = 2;

    }
    
}

pub fn recv(sender: &mut Proc, receiver: &mut Proc, message: Message) {} 
pub fn sendrecv(sender: &mut Proc, receiver: &mut Proc, message: Message) {} 

pub fn init() {
    let mut A: Proc = Proc::init(0);
    let mut B: Proc = Proc::init(1);
    
    let message = Message {
        endpoint: 0,
        message_type: 0,
        payload: MessageType::mess_u16 { data: [1; 28], },
    };
    send(&mut A, &mut B, message); 
}
