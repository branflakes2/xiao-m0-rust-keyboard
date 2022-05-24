#![allow(dead_code)]
/**
 * Modifier masks - used for the first byte in the HID report.
 * NOTE: The second byte in the report is reserved, 0x00
 */
pub const KEY_MOD_LCTRL: u8 = 0x01;
pub const KEY_MOD_LSHIFT: u8 = 0x02;
pub const KEY_MOD_LALT: u8 = 0x04;
pub const KEY_MOD_LMETA: u8 = 0x08;
pub const KEY_MOD_RCTRL: u8 = 0x10;
pub const KEY_MOD_RSHIFT: u8 = 0x20;
pub const KEY_MOD_RALT: u8 = 0x40;
pub const KEY_MOD_RMETA: u8 = 0x80;

/**
 * Scan codes - last N slots in the HID report (usually 6).
 * 0x00 if no key pressed.
 *
 * If more than N keys are pressed, the HID reports
 * KEY_ERR_OVF in all slots to indicate this condition.
 */

pub const KEY_NONE: u8 = 0x00; // No key pressed
pub const KEY_ERR_OVF: u8 = 0x01; //  Keyboard Error Roll Over - used for all slots if too many keys are pressed ("Phantom key")
                                  // 0x02 //  Keyboard POST Fail
                                  // 0x03 //  Keyboard Error Undefined
pub const KEY_A: u8 = 0x04; // Keyboard a and A
pub const KEY_B: u8 = 0x05; // Keyboard b and B
pub const KEY_C: u8 = 0x06; // Keyboard c and C
pub const KEY_D: u8 = 0x07; // Keyboard d and D
pub const KEY_E: u8 = 0x08; // Keyboard e and E
pub const KEY_F: u8 = 0x09; // Keyboard f and F
pub const KEY_G: u8 = 0x0a; // Keyboard g and G
pub const KEY_H: u8 = 0x0b; // Keyboard h and H
pub const KEY_I: u8 = 0x0c; // Keyboard i and I
pub const KEY_J: u8 = 0x0d; // Keyboard j and J
pub const KEY_K: u8 = 0x0e; // Keyboard k and K
pub const KEY_L: u8 = 0x0f; // Keyboard l and L
pub const KEY_M: u8 = 0x10; // Keyboard m and M
pub const KEY_N: u8 = 0x11; // Keyboard n and N
pub const KEY_O: u8 = 0x12; // Keyboard o and O
pub const KEY_P: u8 = 0x13; // Keyboard p and P
pub const KEY_Q: u8 = 0x14; // Keyboard q and Q
pub const KEY_R: u8 = 0x15; // Keyboard r and R
pub const KEY_S: u8 = 0x16; // Keyboard s and S
pub const KEY_T: u8 = 0x17; // Keyboard t and T
pub const KEY_U: u8 = 0x18; // Keyboard u and U
pub const KEY_V: u8 = 0x19; // Keyboard v and V
pub const KEY_W: u8 = 0x1a; // Keyboard w and W
pub const KEY_X: u8 = 0x1b; // Keyboard x and X
pub const KEY_Y: u8 = 0x1c; // Keyboard y and Y
pub const KEY_Z: u8 = 0x1d; // Keyboard z and Z

const KEY_1: u8 = 0x1e; // Keyboard 1 and !
const KEY_2: u8 = 0x1f; // Keyboard 2 and @
const KEY_3: u8 = 0x20; // Keyboard 3 and #
const KEY_4: u8 = 0x21; // Keyboard 4 and $
const KEY_5: u8 = 0x22; // Keyboard 5 and %
const KEY_6: u8 = 0x23; // Keyboard 6 and ^
const KEY_7: u8 = 0x24; // Keyboard 7 and &
const KEY_8: u8 = 0x25; // Keyboard 8 and *
const KEY_9: u8 = 0x26; // Keyboard 9 and (
const KEY_0: u8 = 0x27; // Keyboard 0 and )

const KEY_ENTER: u8 = 0x28; // Keyboard Return (ENTER)
const KEY_ESC: u8 = 0x29; // Keyboard ESCAPE
const KEY_BACKSPACE: u8 = 0x2a; // Keyboard DELETE (Backspace)
const KEY_TAB: u8 = 0x2b; // Keyboard Tab
const KEY_SPACE: u8 = 0x2c; // Keyboard Spacebar
const KEY_MINUS: u8 = 0x2d; // Keyboard - and _
const KEY_EQUAL: u8 = 0x2e; // Keyboard = and +
const KEY_LEFTBRACE: u8 = 0x2f; // Keyboard [ and {
const KEY_RIGHTBRACE: u8 = 0x30; // Keyboard ] and }
const KEY_BACKSLASH: u8 = 0x31; // Keyboard \ and |
const KEY_HASHTILDE: u8 = 0x32; // Keyboard Non-US # and ~
const KEY_SEMICOLON: u8 = 0x33; // Keyboard ; and :
const KEY_APOSTROPHE: u8 = 0x34; // Keyboard ' and "
const KEY_GRAVE: u8 = 0x35; // Keyboard ` and ~
const KEY_COMMA: u8 = 0x36; // Keyboard , and <
const KEY_DOT: u8 = 0x37; // Keyboard . and >
const KEY_SLASH: u8 = 0x38; // Keyboard / and ?
const KEY_CAPSLOCK: u8 = 0x39; // Keyboard Caps Lock

const KEY_F1: u8 = 0x3a; // Keyboard F1
const KEY_F2: u8 = 0x3b; // Keyboard F2
const KEY_F3: u8 = 0x3c; // Keyboard F3
const KEY_F4: u8 = 0x3d; // Keyboard F4
const KEY_F5: u8 = 0x3e; // Keyboard F5
const KEY_F6: u8 = 0x3f; // Keyboard F6
const KEY_F7: u8 = 0x40; // Keyboard F7
const KEY_F8: u8 = 0x41; // Keyboard F8
const KEY_F9: u8 = 0x42; // Keyboard F9
const KEY_F10: u8 = 0x43; // Keyboard F10
const KEY_F11: u8 = 0x44; // Keyboard F11
const KEY_F12: u8 = 0x45; // Keyboard F12

const KEY_SYSRQ: u8 = 0x46; // Keyboard Print Screen
const KEY_SCROLLLOCK: u8 = 0x47; // Keyboard Scroll Lock
const KEY_PAUSE: u8 = 0x48; // Keyboard Pause
const KEY_INSERT: u8 = 0x49; // Keyboard Insert
const KEY_HOME: u8 = 0x4a; // Keyboard Home
const KEY_PAGEUP: u8 = 0x4b; // Keyboard Page Up
const KEY_DELETE: u8 = 0x4c; // Keyboard Delete Forward
const KEY_END: u8 = 0x4d; // Keyboard End
const KEY_PAGEDOWN: u8 = 0x4e; // Keyboard Page Down
const KEY_RIGHT: u8 = 0x4f; // Keyboard Right Arrow
const KEY_LEFT: u8 = 0x50; // Keyboard Left Arrow
const KEY_DOWN: u8 = 0x51; // Keyboard Down Arrow
const KEY_UP: u8 = 0x52; // Keyboard Up Arrow

const KEY_NUMLOCK: u8 = 0x53; // Keyboard Num Lock and Clear
const KEY_KPSLASH: u8 = 0x54; // Keypad /
const KEY_KPASTERISK: u8 = 0x55; // Keypad *
const KEY_KPMINUS: u8 = 0x56; // Keypad -
const KEY_KPPLUS: u8 = 0x57; // Keypad +
const KEY_KPENTER: u8 = 0x58; // Keypad ENTER
const KEY_KP1: u8 = 0x59; // Keypad 1 and End
const KEY_KP2: u8 = 0x5a; // Keypad 2 and Down Arrow
const KEY_KP3: u8 = 0x5b; // Keypad 3 and PageDn
const KEY_KP4: u8 = 0x5c; // Keypad 4 and Left Arrow
const KEY_KP5: u8 = 0x5d; // Keypad 5
const KEY_KP6: u8 = 0x5e; // Keypad 6 and Right Arrow
const KEY_KP7: u8 = 0x5f; // Keypad 7 and Home
const KEY_KP8: u8 = 0x60; // Keypad 8 and Up Arrow
const KEY_KP9: u8 = 0x61; // Keypad 9 and Page Up
const KEY_KP0: u8 = 0x62; // Keypad 0 and Insert
const KEY_KPDOT: u8 = 0x63; // Keypad . and Delete

const KEY_102ND: u8 = 0x64; // Keyboard Non-US \ and |
const KEY_COMPOSE: u8 = 0x65; // Keyboard Application
const KEY_POWER: u8 = 0x66; // Keyboard Power
const KEY_KPEQUAL: u8 = 0x67; // Keypad =

const KEY_F13: u8 = 0x68; // Keyboard F13
const KEY_F14: u8 = 0x69; // Keyboard F14
const KEY_F15: u8 = 0x6a; // Keyboard F15
const KEY_F16: u8 = 0x6b; // Keyboard F16
const KEY_F17: u8 = 0x6c; // Keyboard F17
const KEY_F18: u8 = 0x6d; // Keyboard F18
const KEY_F19: u8 = 0x6e; // Keyboard F19
const KEY_F20: u8 = 0x6f; // Keyboard F20
const KEY_F21: u8 = 0x70; // Keyboard F21
const KEY_F22: u8 = 0x71; // Keyboard F22
const KEY_F23: u8 = 0x72; // Keyboard F23
const KEY_F24: u8 = 0x73; // Keyboard F24

const KEY_OPEN: u8 = 0x74; // Keyboard Execute
const KEY_HELP: u8 = 0x75; // Keyboard Help
const KEY_PROPS: u8 = 0x76; // Keyboard Menu
const KEY_FRONT: u8 = 0x77; // Keyboard Select
const KEY_STOP: u8 = 0x78; // Keyboard Stop
const KEY_AGAIN: u8 = 0x79; // Keyboard Again
const KEY_UNDO: u8 = 0x7a; // Keyboard Undo
const KEY_CUT: u8 = 0x7b; // Keyboard Cut
const KEY_COPY: u8 = 0x7c; // Keyboard Copy
const KEY_PASTE: u8 = 0x7d; // Keyboard Paste
const KEY_FIND: u8 = 0x7e; // Keyboard Find
const KEY_MUTE: u8 = 0x7f; // Keyboard Mute
const KEY_VOLUMEUP: u8 = 0x80; // Keyboard Volume Up
const KEY_VOLUMEDOWN: u8 = 0x81; // Keyboard Volume Down
                                 // 0x82:u8  Keyboard Locking Caps Lock
                                 // 0x83  Keyboard Locking Num Lock
                                 // 0x84  Keyboard Locking Scroll Lock
const KEY_KPCOMMA: u8 = 0x85; // Keypad Comma
                              // 0x86  Keypad Equal Sign
const KEY_RO: u8 = 0x87; // Keyboard International1
const KEY_KATAKANAHIRAGANA: u8 = 0x88; // Keyboard International2
const KEY_YEN: u8 = 0x89; // Keyboard International3
const KEY_HENKAN: u8 = 0x8a; // Keyboard International4
const KEY_MUHENKAN: u8 = 0x8b; // Keyboard International5
const KEY_KPJPCOMMA: u8 = 0x8c; // Keyboard International6
                                // 0x8d  Keyboard International7
                                // 0x8e  Keyboard International8
                                // 0x8f  Keyboard International9
const KEY_HANGEUL: u8 = 0x90; // Keyboard LANG1
const KEY_HANJA: u8 = 0x91; // Keyboard LANG2
const KEY_KATAKANA: u8 = 0x92; // Keyboard LANG3
const KEY_HIRAGANA: u8 = 0x93; // Keyboard LANG4
const KEY_ZENKAKUHANKAKU: u8 = 0x94; // Keyboard LANG5
                                     // 0x95  Keyboard LANG6
                                     // 0x96  Keyboard LANG7
                                     // 0x97  Keyboard LANG8
                                     // 0x98  Keyboard LANG9
                                     // 0x99  Keyboard Alternate Erase
                                     // 0x9a  Keyboard SysReq/Attention
                                     // 0x9b  Keyboard Cancel
                                     // 0x9c  Keyboard Clear
                                     // 0x9d  Keyboard Prior
                                     // 0x9e  Keyboard Return
                                     // 0x9f  Keyboard Separator
                                     // 0xa0  Keyboard Out
                                     // 0xa1  Keyboard Oper
                                     // 0xa2  Keyboard Clear/Again
                                     // 0xa3  Keyboard CrSel/Props
                                     // 0xa4  Keyboard ExSel

// 0xb0  Keypad 00
// 0xb1  Keypad 000
// 0xb2  Thousands Separator
// 0xb3  Decimal Separator
// 0xb4  Currency Unit
// 0xb5  Currency Sub-unit
const KEY_KPLEFTPAREN: u8 = 0xb6; // Keypad (
const KEY_KPRIGHTPAREN: u8 = 0xb7; // Keypad )
                                   // 0xb8  Keypad {
                                   // 0xb9  Keypad }
                                   // 0xba  Keypad Tab
                                   // 0xbb  Keypad Backspace
                                   // 0xbc  Keypad A
                                   // 0xbd  Keypad B
                                   // 0xbe  Keypad C
                                   // 0xbf  Keypad D
                                   // 0xc0  Keypad E
                                   // 0xc1  Keypad F
                                   // 0xc2  Keypad XOR
                                   // 0xc3  Keypad ^
                                   // 0xc4  Keypad %
                                   // 0xc5  Keypad <
                                   // 0xc6  Keypad >
                                   // 0xc7  Keypad &
                                   // 0xc8  Keypad &&
                                   // 0xc9  Keypad |
                                   // 0xca  Keypad ||
                                   // 0xcb  Keypad :
                                   // 0xcc  Keypad #
                                   // 0xcd  Keypad Space
                                   // 0xce  Keypad @
                                   // 0xcf  Keypad !
                                   // 0xd0  Keypad Memory Store
                                   // 0xd1  Keypad Memory Recall
                                   // 0xd2  Keypad Memory Clear
                                   // 0xd3  Keypad Memory Add
                                   // 0xd4  Keypad Memory Subtract
                                   // 0xd5  Keypad Memory Multiply
                                   // 0xd6  Keypad Memory Divide
                                   // 0xd7  Keypad +/-
                                   // 0xd8  Keypad Clear
                                   // 0xd9  Keypad Clear Entry
                                   // 0xda  Keypad Binary
                                   // 0xdb  Keypad Octal
                                   // 0xdc  Keypad Decimal
                                   // 0xdd  Keypad Hexadecimal

const KEY_LEFTCTRL: u8 = 0xe0; // Keyboard Left Control
const KEY_LEFTSHIFT: u8 = 0xe1; // Keyboard Left Shift
const KEY_LEFTALT: u8 = 0xe2; // Keyboard Left Alt
const KEY_LEFTMETA: u8 = 0xe3; // Keyboard Left GUI
const KEY_RIGHTCTRL: u8 = 0xe4; // Keyboard Right Control
const KEY_RIGHTSHIFT: u8 = 0xe5; // Keyboard Right Shift
const KEY_RIGHTALT: u8 = 0xe6; // Keyboard Right Alt
const KEY_RIGHTMETA: u8 = 0xe7; // Keyboard Right GUI

const KEY_MEDIA_PLAYPAUSE: u8 = 0xe8;
const KEY_MEDIA_STOPCD: u8 = 0xe9;
const KEY_MEDIA_PREVIOUSSONG: u8 = 0xea;
const KEY_MEDIA_NEXTSONG: u8 = 0xeb;
const KEY_MEDIA_EJECTCD: u8 = 0xec;
const KEY_MEDIA_VOLUMEUP: u8 = 0xed;
const KEY_MEDIA_VOLUMEDOWN: u8 = 0xee;
const KEY_MEDIA_MUTE: u8 = 0xef;
const KEY_MEDIA_WWW: u8 = 0xf0;
const KEY_MEDIA_BACK: u8 = 0xf1;
const KEY_MEDIA_FORWARD: u8 = 0xf2;
const KEY_MEDIA_STOP: u8 = 0xf3;
const KEY_MEDIA_FIND: u8 = 0xf4;
const KEY_MEDIA_SCROLLUP: u8 = 0xf5;
const KEY_MEDIA_SCROLLDOWN: u8 = 0xf6;
const KEY_MEDIA_EDIT: u8 = 0xf7;
const KEY_MEDIA_SLEEP: u8 = 0xf8;
const KEY_MEDIA_COFFEE: u8 = 0xf9;
const KEY_MEDIA_REFRESH: u8 = 0xfa;
const KEY_MEDIA_CALC: u8 = 0xfb;
