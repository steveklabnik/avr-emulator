pub use self::add::perform as add;
pub use self::inc::perform as inc;
pub use self::jmp::perform as jmp;
pub use self::ldi::perform as ldi;

mod add;
mod inc;
mod jmp;
mod ldi;
