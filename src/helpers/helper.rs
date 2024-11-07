pub mod helper {
    use crate::sdk::{
        engine_classes::{UEngine, UWorld},
        Basic::{FName, UIpNetDriver},
    };

    // TODO: Use UNetDriver instead (will have to implement it first....)
    type CreateNetDriverType = unsafe fn(*mut UEngine, *mut UWorld, FName) -> *mut UIpNetDriver;

    pub fn create_net_driver(
        engine: *mut UEngine,
        world: *mut UWorld,
        name: FName,
    ) -> *mut UIpNetDriver {
        let create_net_driver: CreateNetDriverType =
            unsafe { std::mem::transmute((0x1A1E1C0 as *const u8).add(0)) };

        unsafe { create_net_driver(engine, world, name) }
    }
}
