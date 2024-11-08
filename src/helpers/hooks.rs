pub mod hooks {
    use crate::sdk::engine_classes::{UEngine, UNetDriver, UReplicationDriver};

    pub fn get_max_tick_rate_hook(engine: &UEngine, a2: f32, a3: char) -> f32 {
        30.0
    }

    type ServerReplicateActorsFn = fn(&UReplicationDriver);
    type OGTickFlushFn = fn(&mut UNetDriver, i32);

    pub static mut tick_flush: Option<OGTickFlushFn> = None;
    static mut SERVER_REPLICATE_ACTORS: Option<for<'a> fn(&'a UReplicationDriver)> = None;

    pub fn set_server_replicate_actors(
        server_replicate_actors_fn: for<'a> fn(&'a UReplicationDriver),
    ) {
        unsafe {
            SERVER_REPLICATE_ACTORS = Some(server_replicate_actors_fn);
        }
    }

    // No way this works right?
    pub fn tick_flush_hook(net_driver: &mut UNetDriver, a2: i32) {
        if let Some(replication_driver) = &net_driver.replication_driver {
            if !net_driver.client_connections.is_empty() {
                unsafe {
                    if let Some(replicate_fn) = SERVER_REPLICATE_ACTORS {
                        replicate_fn(replication_driver);
                    }
                }
            }
        }

        unsafe {
            if let Some(tick_flush_fn) = tick_flush {
                tick_flush_fn(net_driver, a2);
            }
        }
    }
}
