use bindings;

pub struct Mode(bindings::mode_t);

impl Mode {
    pub fn from_int(m: u32) -> Mode {
        Mode(m)
    }
}
