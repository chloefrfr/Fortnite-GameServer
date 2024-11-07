extern crate minhook_sys;
extern crate winapi;

mod helpers;
mod sdk;

use helpers::utilities::utilities::{change_byte, get_image_base, nop_function};
use minhook_sys::*;
use sdk::Basic;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use winapi::ctypes::c_void as winapi_c_void;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::wincon::SetConsoleTitleA;

fn allocate_console() {
    unsafe {
        if AllocConsole() != 0 {
            write!(stdout(), "Console window allocated\n").unwrap();
        }
    }
}

fn main_thread() {
    write!(stdout(), "Main Thread Running\n").unwrap();

    unsafe {
        nop_function((get_image_base() + 0xB8A380) as *mut winapi_c_void);
        nop_function((get_image_base() + 0xB89E50) as *mut winapi_c_void);
        change_byte((get_image_base() + 0x10503CD) as *mut u8, 0x85);
        change_byte((get_image_base() + 0x29838A5) as *mut u8, 0x74);

        *(get_image_base() as *mut bool).offset(0x53EB16C as isize) = true;
        *(get_image_base() as *mut bool).offset(0x53EB16D as isize) = true;

        if MH_Initialize() != MH_OK {
            write!(stdout(), "MinHook Initialization Failed\n").unwrap();
        } else {
            write!(stdout(), "MinHook Initialized\n").unwrap();
        }

        Basic::init_gobjects();
        write!(stdout(), "GObjects Initialized\n").unwrap();

        write!(stdout(), "BaseAddress: 0x{:x}\n", get_image_base()).unwrap();

        SetConsoleTitleA("Rust GameServer".as_ptr() as *const i8);

        if MH_Initialize() != MH_OK {
            write!(stdout(), "MinHook Initialization Failed\n").unwrap();
        } else {
            write!(stdout(), "MinHook Initialized\n").unwrap();
        }
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

#[no_mangle]
pub extern "C" fn DllMain(
    _hinst_dll: *mut std::ffi::c_void,
    fdw_reason: u32,
    _lp_reserved: *mut std::ffi::c_void,
) -> i32 {
    match fdw_reason {
        1 => {
            allocate_console();

            thread::spawn(move || {
                main_thread();
            });
        }
        0 => {}
        _ => {}
    }

    1
}
