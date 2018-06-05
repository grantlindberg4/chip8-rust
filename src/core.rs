#[derive(Clone, Copy)]
pub enum Pixel {
    Black,
    White,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum KeyState {
    Pressed,
    Released,
}
