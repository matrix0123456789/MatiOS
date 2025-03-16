use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use crate::process_management::thread::Thread;

pub struct Process
{
    pub threads: Vec<Rc<RefCell<Thread>>>,
    pub name: Box<str>
}