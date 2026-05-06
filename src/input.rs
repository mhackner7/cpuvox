use gxhash::{HashMap, HashMapExt};
use std::collections::VecDeque;
use std::hash::Hash;
use std::mem::transmute;
use std::ops::{Index, IndexMut};

use crate::camera::Camera;
use winit::event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta};
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct InputHandler {
    pub keyqueue: VecDeque<(Control, ElementState)>,
    pub keymap: KeyMap,
    pub active_keys: ActiveKeys,
    pub mouse_sensitivity: f32,
    pub scroll_wheel: ScrollWheel,
}

/// maps a key for each control
/// may add chording in the future for specific actions
pub struct KeyMap {
    pub inputs: [PressedInput; Control::MaxControlCount as usize],
    pub map: HashMap<PressedInput, Control>,
}

/// pressed input (keypress or mouse button)
/// Key is a event::DeviceEvent::Key(RawKeyEvent).KeyCode
/// Mouse is a event::MouseEvent
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum PressedInput {
    Key(KeyCode),
    MouseKey(MouseButton),
}

/// stores any currently active keys
#[derive(Default)]
pub struct ActiveKeys {
    pub buffer: [KeyLog; Control::MaxControlCount as usize],
}

/// all possible control options
#[repr(usize)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Default)]
pub enum Control {
    Forward = 0,
    Backward,
    Left,
    Right,
    Jump,
    Sprint,
    Sneak,
    Destroy,
    Interact,
    MaxControlCount,
    #[default]
    Invalid,
}

/// stores the current state of the scroll wheel
/// for use with menus and minecraft style hotbars
/// loops around when going below the min or above the max
/// min always zero for simplicity
/// will need to be refactored eventually
/// to allow for context or menu specific states
/// when horizontal left is 0 and right is max
#[derive(Clone, Copy)]
pub struct ScrollWheel {
    current_pos: u8,
    max_slots: u8,
    is_circular: bool,
}

/// stores the amount of ticks a key has been active, zero for inactive
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct KeyLog {
    pub duration: u32,
}

/*
-
-
-
-
-
-
*/

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keyqueue: VecDeque::new(),
            keymap: Default::default(),
            active_keys: Default::default(),
            mouse_sensitivity: 2.0,
            scroll_wheel: Default::default(),
        }
    }

    pub fn control(&mut self, event: DeviceEvent) {}

    pub fn mouse_wheel(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, v) => {
                self.scroll_wheel = self.scroll_wheel.add(v as i16);
            }
            _ => (),
        }
    }

    pub fn mouse_motion(&self, camera: &mut Camera, delta: (f64, f64)) {
        camera.pitch += delta.0 as f32 * self.mouse_sensitivity;
        camera.yaw += delta.1 as f32 * self.mouse_sensitivity;
        camera.update();
    }
}

/*
-
-
-
-
-
-
*/

impl ScrollWheel {
    pub fn new(pos: u8, max: u8, is_circular: bool) -> Self {
        Self {
            current_pos: pos,
            max_slots: max,
            is_circular,
        }
    }

    #[inline(always)]
    pub fn add(mut self, delta: i16) -> Self {
        Self::new(
            self.current_pos + (delta % self.max_slots as i16) as u8,
            self.max_slots,
            self.is_circular,
        )
    }
}

impl Default for ScrollWheel {
    fn default() -> Self {
        ScrollWheel {
            current_pos: 0,
            max_slots: 10,
            is_circular: true,
        }
    }
}

/*
-
-
-
-
-
-
*/

impl ActiveKeys {
    pub fn add(&mut self, key: Control) {
        self.buffer[key as usize] = KeyLog::from(0);
    }

    pub fn remove(&mut self, slice: &[Control]) {
        for ctrl in slice {
            self[*ctrl] = KeyLog::from(0);
        }
    }

    pub fn tick(&mut self, ticks: u32) {
        self.buffer.iter_mut().for_each(|key| {
            key.duration += ticks * (key.duration != 0) as u32;
        })
    }
}

impl Index<Control> for ActiveKeys {
    type Output = KeyLog;
    fn index(&self, index: Control) -> &Self::Output {
        &self.buffer[index as usize]
    }
}

impl IndexMut<Control> for ActiveKeys {
    fn index_mut(&mut self, index: Control) -> &mut Self::Output {
        &mut self.buffer[index as usize]
    }
}

impl From<u32> for KeyLog {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self { duration: value }
    }
}

/*
-
-
-
-
-
-
*/

impl KeyMap {
    pub fn find(&self, key: PressedInput) -> Control {
        return match self.map.get(&key) {
            Some(ctrl) => *ctrl,
            None => Control::Invalid,
        };
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        use Control::*;

        const DEFAULT: PressedInput = PressedInput::Key(KeyCode::Digit0);

        let mut keymap = Self {
            inputs: [DEFAULT; Control::MaxControlCount as usize],
            map: HashMap::new(),
        };

        keymap[Forward] = KeyCode::KeyW.into();
        keymap[Backward] = KeyCode::KeyS.into();
        keymap[Left] = KeyCode::KeyA.into();
        keymap[Right] = KeyCode::KeyD.into();
        keymap[Jump] = KeyCode::Space.into();
        keymap[Sprint] = KeyCode::ShiftLeft.into();
        keymap[Sneak] = KeyCode::KeyC.into();
        keymap[Destroy] = MouseButton::Left.into();
        keymap[Interact] = MouseButton::Right.into();

        for i in 0..keymap.inputs.len() {
            unsafe {
                keymap.map.insert(keymap.inputs[i], transmute(i));
            }
        }

        keymap
    }
}

impl Index<Control> for KeyMap {
    type Output = PressedInput;
    fn index(&self, index: Control) -> &Self::Output {
        &self.inputs[index as usize]
    }
}

impl IndexMut<Control> for KeyMap {
    fn index_mut(&mut self, index: Control) -> &mut Self::Output {
        &mut self.inputs[index as usize]
    }
}

impl From<KeyCode> for PressedInput {
    fn from(value: KeyCode) -> Self {
        PressedInput::Key(value)
    }
}
impl From<MouseButton> for PressedInput {
    fn from(value: MouseButton) -> Self {
        PressedInput::MouseKey(value)
    }
}
