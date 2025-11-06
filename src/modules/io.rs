use std::collections::HashMap;
use std::io::Error;
use std::sync::mpsc::{Receiver, Sender};
use std::{mem, ptr};
use std::sync::{mpsc, LazyLock, Mutex};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::winuser::{
    CallNextHookEx, DispatchMessageW, MapVirtualKeyA, PeekMessageW, SendInput, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, HC_ACTION, INPUT, INPUT_KEYBOARD, KBDLLHOOKSTRUCT, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MAPVK_VK_TO_VSC, MSG, PM_REMOVE, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP
};

use super::networking::Message;

// static HOOK: Mutex<HHOOK> = Mutex::new(None);
static mut CHANNEL: LazyLock<(Sender<Key>, Receiver<Key>)> = LazyLock::new(|| {mpsc::channel() });
static mut ENABLED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::from(false));

pub struct Keyboard {
    pub key_states: HashMap<u8, Key>,
    pub state_changes: Vec<Key>,
    hook: Option<HHOOK>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_states: HashMap::new(),
            state_changes: Vec::new(),
            hook: None,
        }
    }

    pub fn parse_callbacks(&mut self) {
        #[allow(static_mut_refs)]
        let iter = unsafe { CHANNEL.1.try_iter() };
        
        for key in iter {
            let map_state = match self.key_states.get(&key.key_code) {
                Some(k) => k,
                None => {
                    self.key_states.insert(key.key_code, key);
                    self.state_changes.push(key);
                    continue;
                }
            };

            if map_state.key_state != key.key_state {
                self.state_changes.push(key);
            }

            self.key_states.insert(key.key_code, key);
        }
    } 

    pub fn install_hook(&mut self) -> Result<(), Error>{
        // Create and install the keyboard hook
        unsafe {
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(hook_callback),
                0 as HINSTANCE, // NULL for system-wide hooks
                0, // 0 for global hook
            );
    
            if hook.is_null() {
                return Err(Error::last_os_error());
            }
    
            self.hook = Some(hook);
        }

        Ok(())
    }
    
    pub fn uninstall_hook(&self) {
        unsafe {
            UnhookWindowsHookEx(self.hook.unwrap());
        }
    }

    pub fn press_key(key_code: u8) {
        let mut keydown_input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        
        *unsafe { keydown_input.u.ki_mut() } = KEYBDINPUT {
            wVk: 0,
            wScan: unsafe { MapVirtualKeyA(key_code as u32, MAPVK_VK_TO_VSC).try_into().unwrap() },
            dwFlags: KEYEVENTF_SCANCODE,
            time: 0,
            dwExtraInfo: 0,
        };

        let mut inputs = [keydown_input];

        unsafe { SendInput(
            inputs.len() as UINT,
            inputs.as_mut_ptr(),
            size_of::<INPUT>() as i32,
        ) };
    }

    pub fn release_key(key_code: u8) {
        let mut keyup_input = INPUT {
            type_: INPUT_KEYBOARD,
            u: unsafe { std::mem::zeroed() },
        };
        
        *unsafe { keyup_input.u.ki_mut() } = KEYBDINPUT {
            wVk: 0,
            wScan: unsafe { MapVirtualKeyA(key_code as u32, MAPVK_VK_TO_VSC).try_into().unwrap() },
            dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        };

        let mut inputs = [keyup_input];

        unsafe { SendInput(
            inputs.len() as UINT,
            inputs.as_mut_ptr(),
            size_of::<INPUT>() as i32,
        ) };
    }

    pub fn set_hooking(enabled: bool) {
            #[allow(static_mut_refs)]
            unsafe{ 
                *ENABLED.lock().unwrap() = enabled;
            };
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.uninstall_hook();
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum KeyState {
    Up,
    Down
}

#[derive(Clone, Copy)]
pub struct Key {
    pub key_code: u8,
    pub key_state: KeyState,
}

impl Key {
    pub fn new(key_code: u8, key_state: KeyState) -> Key {
        Key { 
            key_code,
            key_state,
        }
    }

    /// https://boostrobotics.eu/windows-key-codes/
    pub fn is_keycode_whitelist(code: u32) -> bool {
        match code {
            87 => true,
            65 => true,
            83 => true,
            68 => true,
            81 => true,
            69 => true,
            _ => false,
        }
    }
}

impl From<Message> for Key {
    fn from(message: Message) -> Self {
        match message {
            Message::WDown => Key::new(87, KeyState::Down),
            Message::ADown => Key::new(65, KeyState::Down),
            Message::SDown => Key::new(83, KeyState::Down),
            Message::DDown => Key::new(68, KeyState::Down),
            Message::QDown => Key::new(81, KeyState::Down),
            Message::EDown => Key::new(69, KeyState::Down),
            Message::WUp => Key::new(87, KeyState::Up),
            Message::AUp => Key::new(65, KeyState::Up),
            Message::SUp => Key::new(83, KeyState::Up),
            Message::DUp => Key::new(68, KeyState::Up),
            Message::QUp => Key::new(81, KeyState::Up),
            Message::EUp => Key::new(69, KeyState::Up),
        }
    }
}

// Callback function for the keyboard hook
unsafe extern "system" fn hook_callback(code: c_int, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    #[allow(static_mut_refs)]
    let enabled = unsafe{ ENABLED.lock().unwrap().clone() };

    if code >= HC_ACTION {
        let kb_struct = unsafe { *(lparam as *const KBDLLHOOKSTRUCT) };

        let key_code = kb_struct.vkCode;
        let event_type = wparam as u32;
        
        match event_type {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                // is this a key we need to block
                if Key::is_keycode_whitelist(key_code) && enabled {
                    let key = Key::new(key_code.try_into().unwrap(), KeyState::Down);
                    #[allow(static_mut_refs)]
                    unsafe { CHANNEL.0.send(key).unwrap() };

                    return 1;
                }

                // is this key leftctrl (toggle interception)
                if key_code == 0xA2 {
                    let key = Key::new(key_code.try_into().unwrap(), KeyState::Down);
                    #[allow(static_mut_refs)]
                    unsafe { CHANNEL.0.send(key).unwrap() };
                }
            },
            WM_KEYUP | WM_SYSKEYUP => {
                if Key::is_keycode_whitelist(key_code) && enabled {
                    let key = Key::new(key_code.try_into().unwrap(), KeyState::Up);
                    #[allow(static_mut_refs)]
                    unsafe { CHANNEL.0.send(key).unwrap() };

                    return 1;
                }

                // is this key leftctrl (toggle interception)
                if key_code == 0xA2 {
                    let key = Key::new(key_code.try_into().unwrap(), KeyState::Up);
                    #[allow(static_mut_refs)]
                    unsafe { CHANNEL.0.send(key).unwrap() };
                }
            }
            _ => {}
        }
    }

    unsafe { return CallNextHookEx(ptr::null_mut(), code, wparam, lparam) }
}


pub fn message_loop_keepalive() {
    let mut msg: MSG = unsafe { mem::zeroed() };
        
    // Standard Windows message loop
    unsafe {
        if PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}