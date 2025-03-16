use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use crate::process_management::process::Process;
use crate::process_management::thread::Thread;

pub static mut GLOBAL_PROCESS_TABLE: *mut ProcessTable = 0 as *mut ProcessTable;
pub struct ProcessTable{
    pub processes:Vec<Rc<RefCell<Process>>>,
    pub current_threads: [Option<Rc<RefCell<Thread>>>; 1],
    pub thread_queue: VecDeque<Rc<RefCell<Thread>>>,
}

impl ProcessTable {
    pub(crate) fn add_kernel_process() {
        let kernel_process = Rc::from(RefCell::from( Process{
            threads:Vec::new(),
            name:Box::from("kernel"),
        }));
        Self::get_singleton().processes.push(kernel_process);
    }
}

impl ProcessTable{
    pub fn get_singleton()->&'static mut ProcessTable{
        unsafe {
            if GLOBAL_PROCESS_TABLE.is_null(){
                GLOBAL_PROCESS_TABLE = Box::into_raw(Box::new(ProcessTable{
                    processes:Vec::new(),
                    current_threads:[None],
                    thread_queue:VecDeque::new(),
                }));
            }
            return &mut *GLOBAL_PROCESS_TABLE;
        }
    }
}

