use spin::Mutex;

pub static proc_num: Mutex<usize> = Mutex::new(0); 

pub const MAX_NUM: usize = 10;
pub const BUFFER_SIZE: usize = 5;

pub static PROC_TABLE: Mutex<Table> = Mutex::new( Table {
    num: 0,
    list: [Proc{ p_id: 0, state: State::ready, run_time: 0, buffer: [0; BUFFER_SIZE]} ; 10],
});


pub struct Table {
    num: usize,
    list:[Proc; MAX_NUM],
}

impl Table {
    pub fn get_num(&self) -> usize { 
        self.num
    }
    
    pub fn inc_proc_time(&mut self, index: usize) {
        self.list[index].run_time += 1;
    }

    pub fn print_proc_info(&self, index: usize) {
        println!("p_id: {}  run time: {} ", self.list[index].p_id, self.list[index].run_time);
    }

    pub fn add(&mut self, proc: Proc) {
        self.num += 1;
        self.list[self.num] = proc;
    }

    pub fn get_proc(&self, index: usize) -> Proc {
        self.list[index]
    }
}


#[derive(Copy, Clone)]
pub struct Proc {
    p_id: usize,
    run_time: usize,
    state: State,
    buffer: Message,
}

#[derive(Copy, Clone)]
enum State{
    running,
    blocked,
    ready,
}


type Message = [u16; BUFFER_SIZE];

impl Proc {
    pub fn new() -> Self {
        inc_proc_num();
        Proc {
            p_id: *proc_num.lock(),
            run_time: 0,
            state: State::ready,
            buffer: [0; BUFFER_SIZE],
        }
    }

    pub fn is_ready(&self) -> bool {
        match (self.state) {
            State::ready => true,
            _ => false,
        }
    }

    pub fn set_ready(&mut self){
        self.state = State::ready;
    }

    pub fn is_blocked(&self) -> bool {
        match (self.state) {
            State::blocked => true,
            _ => false,
        }
    }

    pub fn set_running(&mut self) {
        self.state = State::running;
    }

    pub fn block(&mut self) {
        self.state = State::blocked;
    }

}

pub fn inc_proc_num() {
    let mut num = proc_num.lock();
    *num += 1;
}



pub fn init() {

    PROC_TABLE.lock().add(Proc {
        p_id: 1,
        run_time: 1,
        state: State::ready,
        buffer: [0; BUFFER_SIZE],
    });

    PROC_TABLE.lock().add(Proc {
        p_id: 2,
        run_time: 1,
        state: State::ready,
        buffer: [0; BUFFER_SIZE],
    });

}

pub fn schedule() {
    let n = PROC_TABLE.lock().get_num();
    for i in 0..n{
        let mut proc = PROC_TABLE.lock().get_proc(i);
        if proc.is_ready() {
            proc.set_running();
        }
        
        if proc.is_blocked() { continue; }
        PROC_TABLE.lock().inc_proc_time(i);
       // PROC_TABLE.lock().print_proc_info(i);
    }

    println!("test message passing, Process A and Process B");

    let mut A = PROC_TABLE.lock().get_proc(0);
    let mut B = PROC_TABLE.lock().get_proc(1);

    println!("A send 3 to B, but B is not blocked");
    send(&mut A, &mut B, 3);
    PROC_TABLE.lock().print_proc_info(0);
    PROC_TABLE.lock().print_proc_info(1);
    A.set_ready();
    B.set_ready();

    println!("A send 3 to B, but B is blocked"); 
    B.block();
    send(&mut A, &mut B, 3);
    A.buffer[0] = 88;
    B.buffer[0] = 99;
    A.set_ready();
    B.set_ready();
    PROC_TABLE.lock().print_proc_info(0);
    PROC_TABLE.lock().print_proc_info(1);
}

pub fn send(sender: &mut Proc, receiver: &mut Proc, message: u16) { 
    if receiver.is_blocked() {
        receiver.buffer[0] = message; 
        println!("Receive Message");
    }


    else {
        sender.block();
    }
} 
pub fn recv(sender:&mut Proc, receiver: &mut Proc, message: u16) { 
    receiver.buffer = [message; BUFFER_SIZE];
    receiver.set_running();
} 
pub fn sendrecv(sender: &mut Proc, receiver: &mut Proc, message: u16) { 
    sender.block();
    while receiver.is_blocked() { }
    receiver.block();
    receiver.buffer = [message; BUFFER_SIZE];
    receiver.set_ready();
    sender.set_ready();
} 
    
