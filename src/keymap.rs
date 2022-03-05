use phf;
use phf::phf_map;

// Mapping taken from here: https://gist.github.com/ekaitz-zarraga/2b25b94b711684ba4e969e5a5723969b

#[allow(unused)]
#[derive(Clone,Copy)]
#[repr(u8)]
pub enum Modifier {
    None = 0x00,
    LCtl = 0x01,
    LShift = 0x02,
    LAlt = 0x04,
    LMeta = 0x08,
    RCtl = 0x10,
    RShift = 0x20,
    RAlt = 0x40,
    RMeta = 0x80
}

#[allow(unused)]
#[derive(Clone,Copy)]
#[repr(u8)]
pub enum Keys {
    None = 0x00,
    KeyA = 0x04,
    KeyB = 0x05,
    KeyC = 0x06,
    KeyD = 0x07,
    KeyE = 0x08,
    KeyF = 0x09,
    KeyG = 0x0a,
    KeyH = 0x0b,
    KeyI = 0x0c,
    KeyJ = 0x0d,
    KeyK = 0x0e,
    KeyL = 0x0f,
    KeyM = 0x10,
    KeyN = 0x11,
    KeyO = 0x12,
    KeyP = 0x13,
    KeyQ = 0x14,
    KeyR = 0x15,
    KeyS = 0x16,
    KeyT = 0x17,
    KeyU = 0x18,
    KeyV = 0x19,
    KeyW = 0x1a,
    KeyX = 0x1b,
    KeyY = 0x1c,
    KeyZ = 0x1d,

    Key1 = 0x1e,
    Key2 = 0x1f,
    Key3 = 0x20,
    Key4 = 0x21,
    Key5 = 0x22,
    Key6 = 0x23,
    Key7 = 0x24,
    Key8 = 0x25,
    Key9 = 0x26,
    Key0 = 0x27,

    KeyEnter = 0x28,
    KeyEsc = 0x29,
    KeyBackspace = 0x2a,
    KeyTab = 0x2b,
    KeySpace = 0x2c,
    KeyMinus = 0x2d,
    KeyEqual = 0x2e,
    KeyLeftBrace = 0x2f,
    KeyRightBrace = 0x30,
    KeyBackslash = 0x31,
    KeyHashTilde = 0x32,
    KeySemicolon = 0x33,
    KeyApostrophe = 0x34,
    KeyGrave = 0x35,
    KeyComma = 0x36,
    KeyDot = 0x37,
    KeySlash = 0x38,
    KeyCapslock = 0x39,
}

static KEYS: phf::Map<char, Keys> = phf_map! {
    'a' => Keys::KeyA,
    'b' => Keys::KeyB,
    'c' => Keys::KeyC,
    'd' => Keys::KeyD,
    'e' => Keys::KeyE,
    'f' => Keys::KeyF,
    'g' => Keys::KeyG,
    'h' => Keys::KeyH,
    'i' => Keys::KeyI,
    'j' => Keys::KeyJ,
    'k' => Keys::KeyK,
    'l' => Keys::KeyL,
    'm' => Keys::KeyM,
    'n' => Keys::KeyN,
    'o' => Keys::KeyO,
    'p' => Keys::KeyP,
    'q' => Keys::KeyQ,
    'r' => Keys::KeyR,
    's' => Keys::KeyS,
    't' => Keys::KeyT,
    'u' => Keys::KeyU,
    'v' => Keys::KeyV,
    'w' => Keys::KeyW,
    'x' => Keys::KeyX,
    'y' => Keys::KeyY,
    'z' => Keys::KeyZ,

    '1' => Keys::Key1,
    '2' => Keys::Key2,
    '3' => Keys::Key3,
    '4' => Keys::Key4,
    '5' => Keys::Key5,
    '6' => Keys::Key6,
    '7' => Keys::Key7,
    '8' => Keys::Key8,
    '9' => Keys::Key9,
    '0' => Keys::Key0,

    ' ' => Keys::KeySpace,
    '-' => Keys::KeyMinus,
    '=' => Keys::KeyEqual,
    '[' => Keys::KeyLeftBrace,
    ']' => Keys::KeyRightBrace,
    '\\' => Keys::KeyBackslash,
    '#' => Keys::KeyHashTilde,
    ';' => Keys::KeySemicolon,
    '\'' => Keys::KeyApostrophe,
    ',' => Keys::KeyComma,
    '.' => Keys::KeyDot,
    '/' => Keys::KeySlash,
};

pub fn parse_char(character: &char) -> Option<Keys> {
    KEYS.get(character).cloned()
}
