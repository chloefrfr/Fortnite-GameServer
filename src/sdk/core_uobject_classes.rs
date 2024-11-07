use std::ffi::c_void;
use std::ffi::OsStr;

use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::sync::{Arc, Mutex};

use super::Basic::FName;
use super::Basic::{EClassCastFlags, TUObjectArray};

pub struct UObject {
    vtable: Arc<Mutex<*mut c_void>>,
    flags: EClassCastFlags,
    index: i32,
    name: FName,
    outer: Option<Arc<UObject>>,
}

static mut G_OBJECTS: *mut Mutex<TUObjectArray> = ptr::null_mut();

impl UObject {
    pub fn new() -> Self {
        UObject {
            vtable: Arc::new(Mutex::new(std::ptr::null_mut())),
            flags: EClassCastFlags::NONE,
            index: 0,
            name: FName::new(),
            outer: None,
        }
    }

    pub fn has_type_flag(&self, type_flag: EClassCastFlags) -> bool {
        (self.flags.bits() & type_flag.bits()) == type_flag.bits()
    }

    pub fn static_class() -> *mut UClass {
        unsafe {
            static mut CLSS: *mut UClass = ptr::null_mut();
            if CLSS.is_null() {
                CLSS = UObject::find_class("Object");
            }
            CLSS
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_full_name(&self) -> String {
        // TODO: Fully implement UClass to do self.get_class().get_name()
        format!("{}.{}", self.get_name(), self.get_name())
    }

    pub fn static_find_object<UEType>(
        full_name: &str,
        in_class: Option<*mut UClass>,
    ) -> Option<*mut UEType> {
        let static_find_object: extern "C" fn(
            *mut UClass,
            *mut UObject,
            *const u16,
            bool,
        ) -> *mut UObject = unsafe { std::mem::transmute(0x18B1A40 as *const u8) };

        let wide_full_name: Vec<u16> = OsStr::new(full_name).encode_wide().collect();
        let obj = unsafe {
            static_find_object(
                in_class.unwrap_or(ptr::null_mut()),
                ptr::null_mut(),
                wide_full_name.as_ptr(),
                false,
            )
        };
        if obj.is_null() {
            None
        } else {
            Some(unsafe { obj as *mut UEType })
        }
    }

    pub fn find_object<UEType>(
        full_name: &str,
        required_type: EClassCastFlags,
    ) -> Option<*mut UEType> {
        if let Some(obj) = UObject::static_find_object::<UEType>(full_name, None) {
            return Some(obj);
        }

        let g_objects = unsafe {
            if let Some(mutex) = G_OBJECTS.as_ref() {
                mutex.lock().unwrap()
            } else {
                panic!("G_OBJECTS is null!");
            }
        };

        for i in 0..g_objects.num() {
            let object = g_objects.get_by_index(i);
            if let Some(obj) = object {
                let locked_obj = obj.lock().unwrap();

                let obj_ref: &UObject = &*locked_obj;

                let required_type_clone = required_type.clone();

                if obj_ref.has_type_flag(required_type_clone)
                    && obj_ref.get_full_name() == full_name
                {
                    return Some(obj_ref as *const UObject as *mut UEType);
                }
            }
        }

        None
    }

    pub fn find_class(class_full_name: &str) -> *mut UClass {
        UObject::find_object::<UClass>(class_full_name, EClassCastFlags::UCLASS)
            .unwrap_or(ptr::null_mut())
    }
}

pub struct UClass {}
