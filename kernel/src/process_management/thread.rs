use alloc::boxed::Box;

pub struct Thread{
    pub stack_pointer:*mut u8,
    pub name:Box<str>,
}
impl Thread {
    pub fn new(stack_pointer:*mut u8, name:Box<str>) -> Thread {
        return Self{
            stack_pointer,
            name
        }
    }
}