pub mod hooks {
    use crate::sdk::engine_classes::UEngine;

    pub fn get_max_tick_rate_hook(engine: &UEngine, a2: f32, a3: char) -> f32 {
        30.0
    }
}
