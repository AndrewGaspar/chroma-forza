use std::os::raw::c_int;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct RZKEY(pub c_int);

pub const RZKEY_ESC: RZKEY = RZKEY(0x0001);
pub const RZKEY_F1: RZKEY = RZKEY(0x0003);
pub const RZKEY_F2: RZKEY = RZKEY(0x0004);
pub const RZKEY_F3: RZKEY = RZKEY(0x0005);
pub const RZKEY_F4: RZKEY = RZKEY(0x0006);
pub const RZKEY_F5: RZKEY = RZKEY(0x0007);
pub const RZKEY_F6: RZKEY = RZKEY(0x0008);
pub const RZKEY_F7: RZKEY = RZKEY(0x0009);
pub const RZKEY_F8: RZKEY = RZKEY(0x000A);
pub const RZKEY_F9: RZKEY = RZKEY(0x000B);
pub const RZKEY_F10: RZKEY = RZKEY(0x000C);
pub const RZKEY_F11: RZKEY = RZKEY(0x000D);
pub const RZKEY_F12: RZKEY = RZKEY(0x000E);
pub const RZKEY_1: RZKEY = RZKEY(0x0102);
pub const RZKEY_2: RZKEY = RZKEY(0x0103);
pub const RZKEY_3: RZKEY = RZKEY(0x0104);
pub const RZKEY_4: RZKEY = RZKEY(0x0105);
pub const RZKEY_5: RZKEY = RZKEY(0x0106);
pub const RZKEY_6: RZKEY = RZKEY(0x0107);
pub const RZKEY_7: RZKEY = RZKEY(0x0108);
pub const RZKEY_8: RZKEY = RZKEY(0x0109);
pub const RZKEY_9: RZKEY = RZKEY(0x010A);
pub const RZKEY_0: RZKEY = RZKEY(0x010B);
pub const RZKEY_A: RZKEY = RZKEY(0x0302);
pub const RZKEY_B: RZKEY = RZKEY(0x0407);
pub const RZKEY_C: RZKEY = RZKEY(0x0405);
pub const RZKEY_D: RZKEY = RZKEY(0x0304);
pub const RZKEY_E: RZKEY = RZKEY(0x0204);
pub const RZKEY_F: RZKEY = RZKEY(0x0305);
pub const RZKEY_G: RZKEY = RZKEY(0x0306);
pub const RZKEY_H: RZKEY = RZKEY(0x0307);
pub const RZKEY_I: RZKEY = RZKEY(0x0209);
pub const RZKEY_J: RZKEY = RZKEY(0x0308);
pub const RZKEY_K: RZKEY = RZKEY(0x0309);
pub const RZKEY_L: RZKEY = RZKEY(0x030A);
pub const RZKEY_M: RZKEY = RZKEY(0x0409);
pub const RZKEY_N: RZKEY = RZKEY(0x0408);
pub const RZKEY_O: RZKEY = RZKEY(0x020A);
pub const RZKEY_P: RZKEY = RZKEY(0x020B);
pub const RZKEY_Q: RZKEY = RZKEY(0x0202);
pub const RZKEY_R: RZKEY = RZKEY(0x0205);
pub const RZKEY_S: RZKEY = RZKEY(0x0303);
pub const RZKEY_T: RZKEY = RZKEY(0x0206);
pub const RZKEY_U: RZKEY = RZKEY(0x0208);
pub const RZKEY_V: RZKEY = RZKEY(0x0406);
pub const RZKEY_W: RZKEY = RZKEY(0x0203);
pub const RZKEY_X: RZKEY = RZKEY(0x0404);
pub const RZKEY_Y: RZKEY = RZKEY(0x0207);
pub const RZKEY_Z: RZKEY = RZKEY(0x0403);
pub const RZKEY_NUMLOCK: RZKEY = RZKEY(0x0112);
pub const RZKEY_NUMPAD0: RZKEY = RZKEY(0x0513);
pub const RZKEY_NUMPAD1: RZKEY = RZKEY(0x0412);
pub const RZKEY_NUMPAD2: RZKEY = RZKEY(0x0413);
pub const RZKEY_NUMPAD3: RZKEY = RZKEY(0x0414);
pub const RZKEY_NUMPAD4: RZKEY = RZKEY(0x0312);
pub const RZKEY_NUMPAD5: RZKEY = RZKEY(0x0313);
pub const RZKEY_NUMPAD6: RZKEY = RZKEY(0x0314);
pub const RZKEY_NUMPAD7: RZKEY = RZKEY(0x0212);
pub const RZKEY_NUMPAD8: RZKEY = RZKEY(0x0213);
pub const RZKEY_NUMPAD9: RZKEY = RZKEY(0x0214);
pub const RZKEY_NUMPAD_DIVIDE: RZKEY = RZKEY(0x0113);
pub const RZKEY_NUMPAD_MULTIPLY: RZKEY = RZKEY(0x0114);
pub const RZKEY_NUMPAD_SUBTRACT: RZKEY = RZKEY(0x0115);
pub const RZKEY_NUMPAD_ADD: RZKEY = RZKEY(0x0215);
pub const RZKEY_NUMPAD_ENTER: RZKEY = RZKEY(0x0415);
pub const RZKEY_NUMPAD_DECIMAL: RZKEY = RZKEY(0x0514);
pub const RZKEY_PRINTSCREEN: RZKEY = RZKEY(0x000F);
pub const RZKEY_SCROLL: RZKEY = RZKEY(0x0010);
pub const RZKEY_PAUSE: RZKEY = RZKEY(0x0011);
pub const RZKEY_INSERT: RZKEY = RZKEY(0x010F);
pub const RZKEY_HOME: RZKEY = RZKEY(0x0110);
pub const RZKEY_PAGEUP: RZKEY = RZKEY(0x0111);
pub const RZKEY_DELETE: RZKEY = RZKEY(0x020f);
pub const RZKEY_END: RZKEY = RZKEY(0x0210);
pub const RZKEY_PAGEDOWN: RZKEY = RZKEY(0x0211);
pub const RZKEY_UP: RZKEY = RZKEY(0x0410);
pub const RZKEY_LEFT: RZKEY = RZKEY(0x050F);
pub const RZKEY_DOWN: RZKEY = RZKEY(0x0510);
pub const RZKEY_RIGHT: RZKEY = RZKEY(0x0511);
pub const RZKEY_TAB: RZKEY = RZKEY(0x0201);
pub const RZKEY_CAPSLOCK: RZKEY = RZKEY(0x0301);
pub const RZKEY_BACKSPACE: RZKEY = RZKEY(0x010E);
pub const RZKEY_ENTER: RZKEY = RZKEY(0x030E);
pub const RZKEY_LCTRL: RZKEY = RZKEY(0x0501);
pub const RZKEY_LWIN: RZKEY = RZKEY(0x0502);
pub const RZKEY_LALT: RZKEY = RZKEY(0x0503);
pub const RZKEY_SPACE: RZKEY = RZKEY(0x0507);
pub const RZKEY_RALT: RZKEY = RZKEY(0x050B);
pub const RZKEY_FN: RZKEY = RZKEY(0x050C);
pub const RZKEY_RMENU: RZKEY = RZKEY(0x050D);
pub const RZKEY_RCTRL: RZKEY = RZKEY(0x050E);
pub const RZKEY_LSHIFT: RZKEY = RZKEY(0x0401);
pub const RZKEY_RSHIFT: RZKEY = RZKEY(0x040E);
pub const RZKEY_MACRO1: RZKEY = RZKEY(0x0100);
pub const RZKEY_MACRO2: RZKEY = RZKEY(0x0200);
pub const RZKEY_MACRO3: RZKEY = RZKEY(0x0300);
pub const RZKEY_MACRO4: RZKEY = RZKEY(0x0400);
pub const RZKEY_MACRO5: RZKEY = RZKEY(0x0500);
pub const RZKEY_OEM_1: RZKEY = RZKEY(0x0101);
pub const RZKEY_OEM_2: RZKEY = RZKEY(0x010C);
pub const RZKEY_OEM_3: RZKEY = RZKEY(0x010D);
pub const RZKEY_OEM_4: RZKEY = RZKEY(0x020C);
pub const RZKEY_OEM_5: RZKEY = RZKEY(0x020D);
pub const RZKEY_OEM_6: RZKEY = RZKEY(0x020E);
pub const RZKEY_OEM_7: RZKEY = RZKEY(0x030B);
pub const RZKEY_OEM_8: RZKEY = RZKEY(0x030C);
pub const RZKEY_OEM_9: RZKEY = RZKEY(0x040A);
pub const RZKEY_OEM_10: RZKEY = RZKEY(0x040B);
pub const RZKEY_OEM_11: RZKEY = RZKEY(0x040C);
pub const RZKEY_EUR_1: RZKEY = RZKEY(0x030D);
pub const RZKEY_EUR_2: RZKEY = RZKEY(0x0402);
pub const RZKEY_JPN_1: RZKEY = RZKEY(0x0015);
pub const RZKEY_JPN_2: RZKEY = RZKEY(0x040D);
pub const RZKEY_JPN_3: RZKEY = RZKEY(0x0504);
pub const RZKEY_JPN_4: RZKEY = RZKEY(0x0509);
pub const RZKEY_JPN_5: RZKEY = RZKEY(0x050A);
pub const RZKEY_KOR_1: RZKEY = RZKEY(0x0015);
pub const RZKEY_KOR_2: RZKEY = RZKEY(0x030D);
pub const RZKEY_KOR_3: RZKEY = RZKEY(0x0402);
pub const RZKEY_KOR_4: RZKEY = RZKEY(0x040D);
pub const RZKEY_KOR_5: RZKEY = RZKEY(0x0504);
pub const RZKEY_KOR_6: RZKEY = RZKEY(0x0509);
pub const RZKEY_KOR_7: RZKEY = RZKEY(0x050A);
pub const RZKEY_INVALID: RZKEY = RZKEY(0xFFFF);
