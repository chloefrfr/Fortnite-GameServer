use crate::sdk::core_uobject_classes::UObject;
use bitflags::bitflags;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null;
use std::sync::{Arc, Mutex};

use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winnt::LPCWSTR;

use super::core_uobject_classes::UClass;

pub unsafe fn init_gobjects() -> Option<*mut TUObjectArray> {
    const MODULE_NAME: LPCWSTR = null();

    let module_handle = GetModuleHandleW(MODULE_NAME);

    if module_handle.is_null() {
        return None;
    }

    let g_objects_ptr = (module_handle as usize + 0x054F1988) as *mut TUObjectArray;

    Some(g_objects_ptr)
}

pub struct TUObjectArray {
    elements_per_chunk: usize,
    max_elements: i32,
    num_elements: i32,
    max_chunks: i32,
    num_chunks: i32,
    objects: Vec<Vec<FUObjectItem>>,
}

#[derive(Debug)]
pub struct FName {
    pub comparison_index: i32,
    pub number: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FString {
    data: String,
}

impl FString {
    pub fn new() -> Self {
        FString {
            data: String::new(),
        }
    }

    pub fn from_wide(wide: &[u16]) -> Self {
        let string_data = String::from_utf16_lossy(wide);
        FString { data: string_data }
    }

    pub fn to_string(&self) -> String {
        self.data.clone()
    }

    pub fn to_wide_string(&self) -> Vec<u16> {
        OsStr::new(&self.data).encode_wide().collect::<Vec<u16>>()
    }
}

impl From<&str> for FString {
    fn from(s: &str) -> Self {
        FString {
            data: s.to_string(),
        }
    }
}

impl From<String> for FString {
    fn from(s: String) -> Self {
        FString { data: s }
    }
}

impl From<FString> for String {
    fn from(fstr: FString) -> Self {
        fstr.data
    }
}

struct FUObjectItem {
    object: Arc<Mutex<UObject>>,
    pad: [u8; 16],
}
impl TUObjectArray {
    pub fn num(&self) -> i32 {
        self.num_elements
    }

    pub fn get_by_index(&self, index: i32) -> Option<Arc<Mutex<UObject>>> {
        if index < 0 || index >= self.num_elements {
            None
        } else {
            let chunk_index = (index as usize) / self.elements_per_chunk;
            let in_chunk_idx = (index as usize) % self.elements_per_chunk;
            self.objects
                .get(chunk_index)
                .and_then(|chunk| chunk.get(in_chunk_idx))
                .map(|item| Arc::clone(&item.object))
        }
    }

    pub fn new() -> Self {
        TUObjectArray {
            elements_per_chunk: 0,
            max_elements: 0,
            num_elements: 0,
            max_chunks: 0,
            num_chunks: 0,
            objects: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct UIpNetDriver {
    log_port_unreach: bool,
    allow_player_port_unreach: bool,
    max_port_count_to_try: u32,
    server_desired_socket_receive_buffer_bytes: u32,
    server_desired_socket_send_buffer_bytes: u32,
    client_desired_socket_receive_buffer_bytes: u32,
    client_desired_socket_send_buffer_bytes: u32,
}

impl UIpNetDriver {
    pub fn new(
        log_port_unreach: bool,
        allow_player_port_unreach: bool,
        max_port_count_to_try: u32,
        server_desired_socket_receive_buffer_bytes: u32,
        server_desired_socket_send_buffer_bytes: u32,
        client_desired_socket_receive_buffer_bytes: u32,
        client_desired_socket_send_buffer_bytes: u32,
    ) -> Self {
        UIpNetDriver {
            log_port_unreach,
            allow_player_port_unreach,
            max_port_count_to_try,
            server_desired_socket_receive_buffer_bytes,
            server_desired_socket_send_buffer_bytes,
            client_desired_socket_receive_buffer_bytes,
            client_desired_socket_send_buffer_bytes,
        }
    }

    pub fn static_class() -> *mut UClass {
        unsafe {
            static mut CLSS: *mut UClass = std::ptr::null_mut();
            if CLSS.is_null() {
                CLSS = UObject::find_class("IpNetDriver");
            }
            CLSS
        }
    }
}

impl FName {
    pub fn to_string(&self) -> String {
        let mut temp = String::new();

        unsafe {
            let name_to_string: unsafe extern "C" fn(&FName, &mut String) =
                std::mem::transmute((0x16F2EE0 as *const u8).add(0));
            name_to_string(self, &mut temp);
        }

        temp
    }

    pub fn new() -> Self {
        FName {
            comparison_index: 0,
            number: 0,
        }
    }
}

impl PartialEq for FName {
    fn eq(&self, other: &Self) -> bool {
        self.comparison_index == other.comparison_index
    }
}

impl Eq for FName {}

impl PartialOrd for FName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.comparison_index == other.comparison_index {
            self.number.partial_cmp(&other.number)
        } else {
            self.comparison_index.partial_cmp(&other.comparison_index)
        }
    }
}

impl Ord for FName {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.comparison_index == other.comparison_index {
            self.number.cmp(&other.number)
        } else {
            self.comparison_index.cmp(&other.comparison_index)
        }
    }
}

bitflags! {
    pub struct EClassCastFlags: u64 {
        const NONE = 0x0000000000000000;
        const UFIELD = 0x0000000000000001;
        const UINT8_PROPERTY = 0x0000000000000002;
        const UENUM = 0x0000000000000004;
        const USTRUCT = 0x0000000000000008;
        const USCRIPT_STRUCT = 0x0000000000000010;
        const UCLASS = 0x0000000000000020;
        const UBYTE_PROPERTY = 0x0000000000000040;
        const UINT_PROPERTY = 0x0000000000000080;
        const UFLOAT_PROPERTY = 0x0000000000000100;
        const UUINT64_PROPERTY = 0x0000000000000200;
        const UCLASS_PROPERTY = 0x0000000000000400;
        const UUINT32_PROPERTY = 0x0000000000000800;
        const UINTERFACE_PROPERTY = 0x0000000000001000;
        const UNAME_PROPERTY = 0x0000000000002000;
        const USTR_PROPERTY = 0x0000000000004000;
        const UPROPERTY = 0x0000000000008000;
        const UOBJECT_PROPERTY = 0x0000000000010000;
        const UBOOL_PROPERTY = 0x0000000000020000;
        const UUINT16_PROPERTY = 0x0000000000040000;
        const UFUNCTION = 0x0000000000080000;
        const USTRUCT_PROPERTY = 0x0000000000100000;
        const UARRAY_PROPERTY = 0x0000000000200000;
        const UINT64_PROPERTY = 0x0000000000400000;
        const UDELEGATE_PROPERTY = 0x0000000000800000;
        const UNUMERIC_PROPERTY = 0x0000000001000000;
        const UMULTICAST_DELEGATE_PROPERTY = 0x0000000002000000;
        const UOBJECT_PROPERTY_BASE = 0x0000000004000000;
        const UWEAK_OBJECT_PROPERTY = 0x0000000008000000;
        const ULAZY_OBJECT_PROPERTY = 0x0000000010000000;
        const USOFT_OBJECT_PROPERTY = 0x0000000020000000;
        const UTEXT_PROPERTY = 0x0000000040000000;
        const UINT16_PROPERTY_2 = 0x0000000080000000;
        const UDOUBLE_PROPERTY = 0x0000000100000000;
        const USOFT_CLASS_PROPERTY = 0x0000000200000000;
        const UPACKAGE = 0x0000000400000000;
        const ULEVEL = 0x0000000800000000;
        const AACTOR = 0x0000001000000000;
        const APLAYER_CONTROLLER = 0x0000002000000000;
        const APAWN = 0x0000004000000000;
        const USCENE_COMPONENT = 0x0000008000000000;
        const UPRIMITIVE_COMPONENT = 0x0000010000000000;
        const USKINNED_MESH_COMPONENT = 0x0000020000000000;
        const USKELETAL_MESH_COMPONENT = 0x0000040000000000;
        const UBLUEPRINT = 0x0000080000000000;
        const UDELEGATE_FUNCTION = 0x0000100000000000;
        const USTATIC_MESH_COMPONENT = 0x0000200000000000;
        const UMAP_PROPERTY = 0x0000400000000000;
        const USET_PROPERTY = 0x0000800000000000;
        const UENUM_PROPERTY = 0x0001000000000000;
    }
}

impl EClassCastFlags {
    pub fn to_u64(&self) -> u64 {
        self.bits()
    }

    pub fn clone(&self) -> Self {
        EClassCastFlags::from_bits_truncate(self.bits())
    }
}
