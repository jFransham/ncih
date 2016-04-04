extern crate input;

use input::*;

pub trait EventTools<I> {
    fn input(&self) -> Option<&I>;
}

pub trait InputTools {
    fn press(&self) -> Option<Button>;
    fn release(&self) -> Option<Button>;
    fn movement(&self) -> Option<Motion>;
    fn text(&self) -> Option<&str>;
    fn resize(&self) -> Option<(u32, u32)>;
    fn focus(&self) -> Option<bool>;
    fn cursor(&self) -> Option<bool>;
}

pub trait ButtonTools {
    fn keyboard(&self) -> Option<Key>;
    fn mouse(&self) -> Option<MouseButton>;
    fn controller(&self) -> Option<ControllerButton>;
}

pub trait MotionTools {
    fn mouse_cursor(&self) -> Option<(f64, f64)>;
    fn mouse_relative(&self) -> Option<(f64, f64)>;
    fn mouse_scroll(&self) -> Option<(f64, f64)>;
    fn controller_axis(&self) -> Option<ControllerAxisArgs>;
}

impl<I> EventTools<I> for Event<I> {
    fn input(&self) -> Option<&I> {
        match self {
            &Event::Input(ref i) => Some(i),
            _ => None,
        }
    }
}

impl<I> EventTools<I> for Option<Event<I>> {
    fn input(&self) -> Option<&I> {
        match self {
            &Some(Event::Input(ref i)) => Some(i),
            _ => None,
        }
    }
}

impl InputTools for Input {
    fn press(&self) -> Option<Button> {
        match self {
            &Input::Press(b) => Some(b),
            _ => None,
        }
    }

    fn release(&self) -> Option<Button> {
        match self {
            &Input::Release(b) => Some(b),
            _ => None,
        }
    }

    fn movement(&self) -> Option<Motion> {
        match self {
            &Input::Move(m) => Some(m),
            _ => None,
        }
    }

    fn text(&self) -> Option<&str> {
        match self {
            &Input::Text(ref t) => Some(t),
            _ => None,
        }
    }

    fn resize(&self) -> Option<(u32, u32)> {
        match self {
            &Input::Resize(x, y) => Some((x, y)),
            _ => None,
        }
    }

    fn focus(&self) -> Option<bool> {
        match self {
            &Input::Focus(b) => Some(b),
            _ => None,
        }
    }

    fn cursor(&self) -> Option<bool> {
        match self {
            &Input::Cursor(b) => Some(b),
            _ => None,
        }
    }
}

impl InputTools for Option<Input> {
    fn press(&self) -> Option<Button> {
        match self {
            &Some(Input::Press(b)) => Some(b),
            _ => None,
        }
    }

    fn release(&self) -> Option<Button> {
        match self {
            &Some(Input::Release(b)) => Some(b),
            _ => None,
        }
    }

    fn movement(&self) -> Option<Motion> {
        match self {
            &Some(Input::Move(m)) => Some(m),
            _ => None,
        }
    }

    fn text(&self) -> Option<&str> {
        match self {
            &Some(Input::Text(ref t)) => Some(t),
            _ => None,
        }
    }

    fn resize(&self) -> Option<(u32, u32)> {
        match self {
            &Some(Input::Resize(x, y)) => Some((x, y)),
            _ => None,
        }
    }

    fn focus(&self) -> Option<bool> {
        match self {
            &Some(Input::Focus(b)) => Some(b),
            _ => None,
        }
    }

    fn cursor(&self) -> Option<bool> {
        match self {
            &Some(Input::Cursor(b)) => Some(b),
            _ => None,
        }
    }
}

impl ButtonTools for Button {
    fn keyboard(&self) -> Option<Key> {
        match self {
            &Button::Keyboard(k) => Some(k),
            _ => None,
        }
    }

    fn mouse(&self) -> Option<MouseButton> {
        match self {
            &Button::Mouse(m) => Some(m),
            _ => None,
        }
    }

    fn controller(&self) -> Option<ControllerButton> {
        match self {
            &Button::Controller(c) => Some(c),
            _ => None,
        }
    }
}

impl ButtonTools for Option<Button> {
    fn keyboard(&self) -> Option<Key> {
        match self {
            &Some(Button::Keyboard(k)) => Some(k),
            _ => None,
        }
    }

    fn mouse(&self) -> Option<MouseButton> {
        match self {
            &Some(Button::Mouse(m)) => Some(m),
            _ => None,
        }
    }

    fn controller(&self) -> Option<ControllerButton> {
        match self {
            &Some(Button::Controller(c)) => Some(c),
            _ => None,
        }
    }
}

impl MotionTools for Motion {
    fn mouse_cursor(&self) -> Option<(f64, f64)> {
        match self {
            &Motion::MouseCursor(x, y) => Some((x, y)),
            _ => None,
        }
    }

    fn mouse_relative(&self) -> Option<(f64, f64)> {
        match self {
            &Motion::MouseRelative(x, y) => Some((x, y)),
            _ => None,
        }
    }

    fn mouse_scroll(&self) -> Option<(f64, f64)> {
        match self {
            &Motion::MouseScroll(x, y) => Some((x, y)),
            _ => None,
        }
    }

    fn controller_axis(&self) -> Option<ControllerAxisArgs> {
        match self {
            &Motion::ControllerAxis(a) => Some(a),
            _ => None,
        }
    }
}

impl MotionTools for Option<Motion> {
    fn mouse_cursor(&self) -> Option<(f64, f64)> {
        match self {
            &Some(Motion::MouseCursor(x, y)) => Some((x, y)),
            _ => None,
        }
    }

    fn mouse_relative(&self) -> Option<(f64, f64)> {
        match self {
            &Some(Motion::MouseRelative(x, y)) => Some((x, y)),
            _ => None,
        }
    }

    fn mouse_scroll(&self) -> Option<(f64, f64)> {
        match self {
            &Some(Motion::MouseScroll(x, y)) => Some((x, y)),
            _ => None,
        }
    }

    fn controller_axis(&self) -> Option<ControllerAxisArgs> {
        match self {
            &Some(Motion::ControllerAxis(a)) => Some(a),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
