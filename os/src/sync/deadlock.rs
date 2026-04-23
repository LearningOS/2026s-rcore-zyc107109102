use alloc::vec::Vec;
use alloc::vec;
///
pub struct Deadlock {
    ///
    pub available: Vec<isize>,
    allocation: Vec<Vec<isize>>,
    need: Vec<Vec<isize>>,
    lock: bool,
    enable_deadlock_detect: bool,
}

impl Deadlock {
    ///new
    pub fn new() -> Self {
        Self {
            available: vec![],
            allocation: vec![vec![0; 16]; 16],
            need: vec![vec![0; 16]; 16],
            lock: false,
            enable_deadlock_detect: false,
        }
    }
    ///get
    pub fn get_enable_deadlock_detect(&self) -> bool{
        self.enable_deadlock_detect
    } 
    ///set
    pub fn set_enable_deadlock_detect(&mut self, enable: bool) {
        self.enable_deadlock_detect = enable;
    }
    ///get
    pub fn get_lock(&self) -> bool{
        self.lock
    } 
    ///set
    pub fn set_lock(&mut self, enable: bool) {
        self.lock = enable;
    }
    ///
    pub fn detect_sem(&mut self) -> isize{
        let mut work = self.available.clone();//步骤一
        let mut finish = vec![false; self.allocation.len()];
        loop {
            for i in 0..finish.len() {
                if finish[i] == false {
                    let mut flag = true;
                    for j in 0..self.available.len() {
                        if self.need[i][j] > work[j] {//步骤二
                            flag = false;
                            break;
                        }
                    }
                    if flag {
                        finish[i] = true;
                        for j in 0..self.available.len() {
                            work[j] += self.allocation[i][j];//步骤三
                        }
                    }
                } else if i == finish.len() - 1 {//步骤四
                    if finish.iter().all(|&x| x) {
                        return 0;
                    } else {
                        return -0xDEAD;
                    }
                }
            }
        }
    }
    ///type0分配，1释放
    pub fn modify_sem(&mut self ,modify: isize ,task_id: usize ,sem_id: usize) {
        match modify {
            0 => {
                if self.available[sem_id] == 0 {
                    self.need[task_id][sem_id] += 1;
                    return;
                }
                self.available[sem_id] -= 1;
                self.allocation[task_id][sem_id] += 1;
            } , 
            _ => {
                self.available[sem_id] += 1;
                self.allocation[task_id][sem_id] -= 1;
            }
        }
    }
}