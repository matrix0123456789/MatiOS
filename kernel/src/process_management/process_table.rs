use crate::process_management::process::Process;
use crate::process_management::thread::Thread;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::arch::asm;
use core::cell::RefCell;
use crate::drivers::apic::local_apic::LocalApic;

pub static mut GLOBAL_PROCESS_TABLE: *mut ProcessTable = 0 as *mut ProcessTable;
pub struct ProcessTable {
    pub processes: Vec<Rc<RefCell<Process>>>,
    pub current_threads: [Option<Rc<RefCell<Thread>>>; 1],
    pub thread_queue: VecDeque<Rc<RefCell<Thread>>>,
}

impl ProcessTable {
    pub(crate) fn resume_thread() {
        LocalApic::reset_timer();
        let next_thread = Self::get_singleton().thread_queue.pop_front();
        let next_thread2 = next_thread.clone();
        if next_thread.is_some() {
            Self::get_singleton().current_threads[0] = Some(Rc::clone(&next_thread.unwrap()));
            let stack_pointer = Rc::clone(&next_thread2.unwrap()).borrow().stack_pointer;
            unsafe {
                asm!(
                "mov rsp, rax",
                "pop r15",
                "pop r14",
                "pop r13",
                "pop r12",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rbp",
                "pop rdi",
                "pop rsi",
                "pop rdx",
                "pop rcx",
                "pop rbx",
                "pop rax",
                "iretq",
                in("rax") stack_pointer as u64,
                )
            }
        } else {
            unsafe {
                asm!("hlt");
            }
        }
    }
}

impl ProcessTable {
    pub(crate) fn pause_current_thread(stack_pointer_raw: u64) {
        let mut current_thread = Option::clone(&Self::get_singleton().current_threads[0]);
        let mut current_thread2 = current_thread.clone();
        Self::get_singleton().current_threads[0] = None;
        Rc::clone(&current_thread.unwrap())
            .borrow_mut()
            .stack_pointer = stack_pointer_raw as *mut u8;

        Self::get_singleton()
            .thread_queue
            .push_back(current_thread2.unwrap());
    }
}

impl ProcessTable {
    pub(crate) fn add_kernel_process() {
        let kernel_process = Rc::from(RefCell::from(Process {
            threads: Vec::new(),
            name: Box::from("kernel"),
        }));
        Self::get_singleton()
            .processes
            .push(Rc::clone(&kernel_process));

        let main_thread = Rc::from(RefCell::from(Thread {
            stack_pointer: 0 as *mut u8,
            name: Box::from("main kernel thread"),
        }));
        kernel_process
            .borrow_mut()
            .threads
            .push(Rc::clone(&main_thread));
        Self::get_singleton().current_threads[0] = Some(main_thread);
    }
}

impl ProcessTable {
    pub fn get_singleton() -> &'static mut ProcessTable {
        unsafe {
            if GLOBAL_PROCESS_TABLE.is_null() {
                GLOBAL_PROCESS_TABLE = Box::into_raw(Box::new(ProcessTable {
                    processes: Vec::new(),
                    current_threads: [None],
                    thread_queue: VecDeque::new(),
                }));
            }
            return &mut *GLOBAL_PROCESS_TABLE;
        }
    }
}
