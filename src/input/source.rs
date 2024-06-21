//= IMPORTS ==================================================================

use crate::input::{KeyCode, MouseButton};

//= ENUM INPUT SOURCE ========================================================

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputSource {
    Key { source: KeyCode }, // TODO: 8bit per le key su Windows, 32bit su Wayland, il problema è che i keycode sono 16bit, quindi non capisco: ma forse centa la virtual key
    Mouse { source: MouseButton },
    //Joy { source: JoypadButton },
}
