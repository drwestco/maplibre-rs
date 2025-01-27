use crate::io::scheduler::ScheduleMethod;
use crate::platform::schedule_method::TokioScheduleMethod;
use crate::window::FromWindow;
use crate::MapBuilder;
pub use std::time::Instant;

// macOS and iOS (Metal)
pub const COLOR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

#[no_mangle]
pub fn maplibre_apple_main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    MapBuilder::from_window("A fantastic window!")
        .with_schedule_method(ScheduleMethod::Tokio(TokioScheduleMethod::new()))
        .build()
        .run_sync();
}
