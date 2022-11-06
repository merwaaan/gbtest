use super::screen::Screen;

pub trait Driver {
    fn screen(&self) -> &Screen;
    fn screen_mut(&mut self) -> &mut Screen;
}
