use float_cmp::ApproxEq;

pub enum MouseScrollDir {
    Up,
    Down,
    None,
}

impl Default for MouseScrollDir {
    fn default() -> Self {
        MouseScrollDir::None
    }
}

impl From<&MouseScrollDir> for bool {
    fn from(dir: &MouseScrollDir) -> bool {
        match dir {
            MouseScrollDir::Up => true,
            _ => false,
        }
    }
}

pub enum MousePressedState {
    Down,
    Up,
    JustReleased,
}

impl Default for MousePressedState {
    fn default() -> Self {
        MousePressedState::Up
    }
}

#[derive(Default)]
pub struct Input {
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_pressed: MousePressedState,
    pub mouse_scroll_dir: MouseScrollDir,
    cum_position_diff: (f64, f64),
    last_update_position: Option<(f64, f64)>,
    down_position: Option<(f64, f64)>,
}

impl Input {
    pub fn reset_scroll(&mut self) {
        self.mouse_scroll_dir = MouseScrollDir::None;
    }

    pub fn mouse_down(&mut self) {
        self.mouse_pressed = MousePressedState::Down;
        self.down_position = Some((self.mouse_x, self.mouse_y));
        self.cum_position_diff = Default::default();
    }

    pub fn mouse_up(&mut self) {
        self.mouse_pressed = match self.mouse_pressed {
            MousePressedState::Down => MousePressedState::JustReleased,
            _ => {
                self.down_position = None;
                MousePressedState::Up
            }
        };
    }

    pub fn mouse_pos(&mut self, x: f64, y: f64) {
        // Save the current position as the last update position.
        self.last_update_position = Some((self.mouse_x, self.mouse_y));

        // Update the cumulative difference in position from the last time position was read.
        let (cum_x, cum_y) = self.cum_position_diff;
        self.cum_position_diff = (cum_x + self.mouse_x - x, cum_y + self.mouse_y - y);

        // Update the current position.
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn mouse_scroll(&mut self, up: bool) {
        if up {
            self.mouse_scroll_dir = MouseScrollDir::Up;
        } else {
            self.mouse_scroll_dir = MouseScrollDir::Down;
        }
    }

    pub fn mouse_pos_diff(&mut self) -> (f64, f64) {
        let (x, y) = self.cum_position_diff;
        self.cum_position_diff = Default::default();
        (x, y)
    }

    pub fn barely_moved_from_down(&self) -> bool {
        if let Some((old_x, old_y)) = self.down_position {
            old_x.approx_eq(self.mouse_x, (0.0, 2)) && old_y.approx_eq(self.mouse_y, (0.0, 2))
        } else {
            false
        }
    }
}
