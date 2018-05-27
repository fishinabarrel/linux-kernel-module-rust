use bindings;

pub struct Mode(bindings::umode_t);

impl Mode {
    pub fn from_int(m: u16) -> Mode {
        Mode(m)
    }

    pub fn as_int(&self) -> u16 {
        return self.0;
    }
}
