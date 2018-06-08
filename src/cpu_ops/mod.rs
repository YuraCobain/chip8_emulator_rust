pub type ArgOctets = (u8, u8, u8, u8);
pub type Id = u16;

pub trait PipeLine {
    fn process_events(&mut self) -> bool;
    fn fetch(&mut self) -> Option<u16>;
    fn decode(&self, instruction: u16) -> Option<(Id, ArgOctets)>;
    fn execute(&mut self, id: Id, arg: ArgOctets) -> Option<()>;
    fn update_timers(&mut self);
}

