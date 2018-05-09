use spin::Mutex;

pub static proc_num: Mutex<usize> = Mutex::new(0); 

pub const MAX_NUM: usize = 10;
pub const BUFFER_SIZE: usize = 5;

pub static PROC_TABLE: Mutex<Table> = Mutex::new( Table {
    num: 0,
    list: [Proc{ p_id: 0, run_time: 0, buffer: [0; BUFFER_SIZE]} ; 10],
});


pub struct Table{
    num: usize,
    list: [Proc; MAX_NUM],
}

impl Table{
    pub fn get_num(&self) -> usize { 
        self.num
    }
    
    pub fn inc_proc_time(&mut self, index: usize) {
        self.list[index].run_time += 1;
    }

    pub fn print_proc_info(&self, index: usize) {
        println!("p_id: {}", self.list[index].p_id);
        println!("run time: {}", self.list[index].run_time);
        println!("message: {:#?}", self.list[index].buffer);
    }

    pub fn add(&mut self, proc: Proc) {
        self.num += 1;
        self.list[self.num] = proc;
    }

}


#[derive(Copy, Clone)]
pub struct Proc {
    p_id: usize,
    run_time: usize,
    
    buffer: Message,
}

type Message = [u16; BUFFER_SIZE];

impl Proc {
    pub fn new() -> Self {
        inc_proc_num();
        Proc {
            p_id: *proc_num.lock(),
            run_time: 0,
            buffer: [0; BUFFER_SIZE],
        }
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
        buffer: [0; BUFFER_SIZE],
    });

    PROC_TABLE.lock().add(Proc {
        p_id: 2,
        run_time: 1,
        buffer: [0; BUFFER_SIZE],
    });


}

pub fn schedule() {
    let n = PROC_TABLE.lock().get_num();
    for i in 0..n {
        PROC_TABLE.lock().inc_proc_time(i);
        PROC_TABLE.lock().print_proc_info(i);
    }
}

pub fn send(caller: &mut Proc, receiver: &Proc, message: u16) { } 
pub fn recv(caller: &mut Proc, sender: &mut Proc, message: u16) { } 
pub fn sendrecv(call: &mut Proc, receiver: &Proc, message: u16) { } 
    
