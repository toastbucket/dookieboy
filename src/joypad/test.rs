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

    jp.update_button(Button::UP, true);
    jp.update_button(Button::DOWN, true);
    jp.update_button(Button::LEFT, true);
    jp.update_button(Button::RIGHT, true);
    jp.update_dir_select(true);
    jp.update_act_select(false);
    let reg = jp.mem_read_byte(P1);
    assert_eq!(reg, 0x20,
               "direction: reported {:#04x} instead of {:#04x}", reg, 0x20);

    jp.update_dir_select(false);
    jp.update_act_select(true);
    let reg = jp.mem_read_byte(P1);
    assert_eq!(reg, 0x1f,
               "action: reported {:#04x} instead of {:#04x}", reg, 0x1f);

    jp.update_button(Button::A, true);
    jp.update_button(Button::B, true);
    jp.update_button(Button::SELECT, true);
    jp.update_button(Button::START, true);
    let reg = jp.mem_read_byte(P1);
    assert_eq!(reg, 0x10,
               "action: reported {:#04x} instead of {:#04x}", reg, 0x10);
}
