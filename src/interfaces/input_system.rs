#[allow(non_snake_case)]
#[repr(i32)]
pub enum ButtonCodeT {
    KeyNone = 0,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    KeyPad0,
    KeyPad1,
    KeyPad2,
    KeyPad3,
    KeyPad4,
    KeyPad5,
    KeyPad6,
    KeyPad7,
    KeyPad8,
    KeyPad9,
    KeyPadDivide,
    KeyPadMultiply,
    KeyPadMinus,
    KeyPadPlus,
    KeyPadEnter,
    KeyPadDecimal,
    KeyLbracket,
    KeyRbracket,
    KeySemicolon,
    KeyApostrophe,
    KeyBackquote,
    KeyComma,
    KeyPeriod,
    KeySlash,
    KeyBackslash,
    KeyMinus,
    KeyEqual,
    KeyEnter,
    KeySpace,
    KeyBackspace,
    KeyTab,
    KeyCapslock,
    KeyNumlock,
    KeyEscape,
    KeyScrolllock,
    KeyInsert,
    KeyDelete,
    KeyHome,
    KeyEnd,
    KeyPageup,
    KeyPagedown,
    KeyBreak,
    KeyLshift,
    KeyRshift,
    KeyLalt,
    KeyRalt,
    KeyLcontrol,
    KeyRcontrol,
    KeyLwin,
    KeyRwin,
    KeyApp,
    KeyUp,
    KeyLeft,
    KeyDown,
    KeyRight,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyCapslocktoggle,
    KeyNumlocktoggle,
    MouseLeft = 107,
    MouseRight,
    MouseMiddle,
    Mouse4,
    Mouse5,
    MouseWheelUp,
    // A fake button which is 'pressed' and 'released' when the wheel is moved up
    MouseWheelDown, // A fake button which is 'pressed' and 'released' when the wheel is moved down
}

impl Default for ButtonCodeT {
    fn default() -> Self {
        ButtonCodeT::KeyNone
    }
}

interface!(
    IInputSystem,
    pub enable_input[11](state: bool) -> (),
    pub is_button_down[15](button: ButtonCodeT) -> bool,
    pub reset_input_state[39]() -> (),
    pub virtual_key_to_button_code[44](virtual_key: i32) -> ButtonCodeT,
    pub button_code_to_virtual_key[45](button: ButtonCodeT) -> i32,
    get_cursor_pos_virtual[56](x: &mut i32, y: &mut i32) -> ()
);

impl IInputSystem {
    pub fn get_cursor_position(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        self.get_cursor_pos_virtual(&mut x, &mut y);
        (x, y)
    }
}