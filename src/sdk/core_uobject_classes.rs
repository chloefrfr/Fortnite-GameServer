use std::ffi::c_void;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::sync::{Arc, Mutex, Weak};

use super::Basic::FName;
use super::Basic::{EClassCastFlags, TUObjectArray};

pub struct UObject {
    _vtable: Arc<Mutex<*mut c_void>>,
    _flags: EClassCastFlags,
    _index: i32,
    _name: FName,
    _outer: Option<Arc<UObject>>,
}

static mut _G_OBJECTS: *mut Mutex<TUObjectArray> = ptr::null_mut();

pub struct UClass {
    pad_67: [u8; 0x30],
    cast_flags: EClassCastFlags,
    pad_68: [u8; 0x38],
    default_object: Weak<UObject>,
    pad_69: [u8; 0x100],
}

#[allow(dead_code)]
impl UClass {
    pub fn static_class() -> *mut UClass {
        unsafe {
            static mut CLSS: *mut UClass = ptr::null_mut();
            if CLSS.is_null() {
                CLSS = UObject::find_class("Class");
            }
            CLSS
        }
    }
}

impl UObject {
    pub fn new() -> Self {
        UObject {
            _vtable: Arc::new(Mutex::new(std::ptr::null_mut())),
            _flags: EClassCastFlags::NONE,
            _index: 0,
            _name: FName::new(),
            _outer: None,
        }
    }

    pub fn has_type_flag(&self, type_flag: EClassCastFlags) -> bool {
        (self._flags.bits() & type_flag.bits()) == type_flag.bits()
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
        self._name.to_string()
    }

    pub fn get_full_name(&self) -> String {
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
        let obj = static_find_object(
            in_class.unwrap_or(ptr::null_mut()),
            ptr::null_mut(),
            wide_full_name.as_ptr(),
            false,
        );

        if obj.is_null() {
            None
        } else {
            Some(obj as *mut UEType)
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
            if let Some(mutex) = _G_OBJECTS.as_ref() {
                mutex.lock().unwrap()
            } else {
                panic!("_G_OBJECTS is null!");
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
