pub trait MediaIf {
    fn draw_display(&mut self, buf: &[[u8; 10]]) -> Option<u8>;
    fn clear_display(&mut self) -> Option<u8>;
    fn present_display(&mut self) -> Option<u8>;

    fn process_events(&mut self) -> bool;
    fn is_key_pressed(&mut self, key: u8) -> bool;
    fn get_pressed_key(&self) -> Option<&u8>;
}

