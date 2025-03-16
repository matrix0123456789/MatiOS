use alloc::boxed::Box;

pub struct Thread{
    pub stack_pointer:*mut u8,
    pub name:Box<str>,
}