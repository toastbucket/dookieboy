// ░░░░░░░░░░░█▀▀░░█░░░░░░
// ░░░░░░▄▀▀▀▀░░░░░█▄▄░░░░
// ░░░░░░█░█░░░░░░░░░░▐░░░
// ░░░░░░▐▐░░░░░░░░░▄░▐░░░
// ░░░░░░█░░░░░░░░▄▀▀░▐░░░
// ░░░░▄▀░░░░░░░░▐░▄▄▀░░░░
// ░░▄▀░░░▐░░░░░█▄▀░▐░░░░░
// ░░█░░░▐░░░░░░░░▄░█░░░░░
// ░░░█▄░░▀▄░░░░▄▀▐░█░░░░░
// ░░░█▐▀▀▀░▀▀▀▀░░▐░█░░░░░
// ░░▐█▐▄░░▀░░░░░░▐░█▄▄░░
// ░░░▀▀░▄TSM▄░░░▐▄▄▄▀░░░

use super::*;

const P1: u16 = 0xff00;

// Verify register properly reflects state
#[test]
fn test_joypad_reg() {
    let mut jp = Joypad::new();

    let reg = jp.mem_read_byte(P1);
    assert_eq!(reg, 0x3f,
               "default: reported {:#04x} instead of {:#04x}", reg, 0x3f);

    jp.up = true;
    jp.down = true;
    jp.left = true;
    jp.right = true;
    jp.direction_select = true;
    let reg = jp.mem_read_byte(P1);
    assert_eq!(reg, 0x20,
               "direction: reported {:#04x} instead of {:#04x}", reg, 0x20);

    jp.direction_select = false;
    jp.action_select = true;
    let reg = jp.mem_read_byte(P1);
    assert_eq!(reg, 0x1f,
               "action: reported {:#04x} instead of {:#04x}", reg, 0x1f);

    jp.a = true;
    jp.b = true;
    jp.select = true;
    jp.start = true;
    let reg = jp.mem_read_byte(P1);
    assert_eq!(reg, 0x10,
               "action: reported {:#04x} instead of {:#04x}", reg, 0x10);
}
