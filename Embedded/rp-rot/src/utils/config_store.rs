use crate::config::device::{DeviceConfigItem, MAX_DEVICE_ID_LEN};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use heapless::String;
use static_cell::StaticCell;

// The global config store (can be expanded for more items)
pub static DEVICE_CONFIG: StaticCell<Mutex<ThreadModeRawMutex, Option<DeviceConfigItem>>> =
    StaticCell::new();
pub static mut DEVICE_CONFIG_REF: Option<
    &'static Mutex<ThreadModeRawMutex, Option<DeviceConfigItem>>,
> = None;
static mut CTX: ThreadModeRawMutex = ThreadModeRawMutex::new();

pub fn init_config_store() {
    let reference = DEVICE_CONFIG.init(Mutex::new(None));
    unsafe {
        DEVICE_CONFIG_REF = Some(reference);
    }
}

pub async fn set_device_config(config: DeviceConfigItem) {
    let mutex = unsafe { DEVICE_CONFIG_REF.expect("Config store not initialized") };
    let mut guard = mutex.lock().await;
    *guard = Some(config);
}

pub async fn get_device_config() -> Option<DeviceConfigItem> {
    let mutex = unsafe { DEVICE_CONFIG_REF.expect("Config store not initialized") };
    let guard = mutex.lock().await;
    guard.clone()
}
