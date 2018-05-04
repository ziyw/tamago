use spin::Mutex; 


// kernel clock, global time 
pub static KCLOCK: Mutex<Clock> = Mutex::new ( Clock {
    time: 0, 
});

pub struct Clock {
    time: usize,
}

impl Clock {
    pub fn set_zero(&mut self) {
        self.time = 0;
    }
    pub fn inc(&mut self) {
        self.time += 1;
    }
    pub fn get_time(&self) -> usize {
        self.time 
    }
    pub fn set_time(&mut self, t: usize) {
        self.time = t;
    }
}



pub fn init() {
    KCLOCK.lock().inc();
}

