use std::os::raw::c_ulong;

pub mod utilities {
    use super::*;
    use winapi::ctypes::c_void;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::memoryapi::VirtualProtect;
    use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

    pub fn nop_function(function: *mut c_void) {
        if function.is_null() {
            return;
        }

        unsafe {
            let mut old_protect: c_ulong = 0;

            VirtualProtect(function, 1, PAGE_EXECUTE_READWRITE, &mut old_protect);

            *(function as *mut u8) = 0xC3;

            VirtualProtect(function, 1, old_protect, &mut old_protect);
        }
    }

    pub fn change_byte(byte_address: *mut u8, new_value: u8) {
        if byte_address.is_null() {
            return;
        }

        unsafe {
            let mut old_protect: c_ulong = 0;

            VirtualProtect(
                byte_address as *mut c_void,
                1,
                PAGE_EXECUTE_READWRITE,
                &mut old_protect,
            );

            *byte_address = new_value;

            VirtualProtect(
                byte_address as *mut c_void,
                1,
                old_protect,
                &mut old_protect,
            );
        }
    }

    pub fn get_image_base() -> u64 {
        let handle = unsafe { GetModuleHandleW(std::ptr::null()) };
        handle as u64
    }
}
