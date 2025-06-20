/// # Configuration Storage
///
/// This module provides a thread-safe storage mechanism for device configuration.
/// It uses a mutex-protected global store that can be accessed by different tasks.

use crate::config::device::{DeviceConfigItem, MAX_DEVICE_ID_LEN};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use heapless::String;
use static_cell::StaticCell;

/// The global configuration store.
///
/// This static variable holds the mutex-protected device configuration.
/// It uses a StaticCell for initialization and can store an optional DeviceConfigItem.
pub static DEVICE_CONFIG: StaticCell<Mutex<ThreadModeRawMutex, Option<DeviceConfigItem>>> =
    StaticCell::new();

/// Global reference to the initialized configuration store.
///
/// This reference is set during initialization and used by the accessor functions.
/// It is unsafe because it involves static mutable state that must be properly initialized.
pub static mut DEVICE_CONFIG_REF: Option<
    &'static Mutex<ThreadModeRawMutex, Option<DeviceConfigItem>>,
> = None;

/// Thread mode raw mutex context for the configuration store.
///
/// This mutex ensures that only one task at a time can access the configuration.
static mut CTX: ThreadModeRawMutex = ThreadModeRawMutex::new();

/// Initializes the global configuration store.
///
/// This function must be called before any other functions in this module.
/// It creates the mutex-protected storage and sets up the global reference.
///
/// # Safety
/// This function is safe to call once at program startup.
/// Calling it multiple times or concurrently with other accesses could lead to undefined behavior.
pub fn init_config_store() {
    // Initialize the mutex with an empty (None) configuration
    let reference = DEVICE_CONFIG.init(Mutex::new(None));
    
    // Set the global reference to the initialized store
    // This is unsafe because we're modifying a static mutable variable
    unsafe {
        DEVICE_CONFIG_REF = Some(reference);
    }
}

/// Updates the device configuration in the global store.
///
/// This function acquires a lock on the configuration mutex and updates
/// the stored configuration with the provided value.
///
/// # Parameters
/// * `config` - The new device configuration to store
///
/// # Panics
/// Panics if the configuration store hasn't been initialized
pub async fn set_device_config(config: DeviceConfigItem) {
    // Get the mutex reference from the global variable
    let mutex = unsafe { DEVICE_CONFIG_REF.expect("Config store not initialized") };
    
    // Acquire a lock on the mutex (this will wait if another task has the lock)
    let mut guard = mutex.lock().await;
    
    // Update the configuration with the new value
    *guard = Some(config);
    
    // Lock is automatically released when guard goes out of scope
}

/// Retrieves the current device configuration from the global store.
///
/// This function acquires a lock on the configuration mutex and returns
/// a clone of the stored configuration (if any).
///
/// # Returns
/// * `Some(DeviceConfigItem)` - If a configuration has been stored
/// * `None` - If no configuration has been stored yet
///
/// # Panics
/// Panics if the configuration store hasn't been initialized
pub async fn get_device_config() -> Option<DeviceConfigItem> {
    // Get the mutex reference from the global variable
    let mutex = unsafe { DEVICE_CONFIG_REF.expect("Config store not initialized") };
    
    // Acquire a lock on the mutex (this will wait if another task has the lock)
    let guard = mutex.lock().await;
    
    // Return a clone of the stored configuration
    // We clone here to avoid holding the lock longer than necessary
    guard.clone()
    
    // Lock is automatically released when guard goes out of scope
}
