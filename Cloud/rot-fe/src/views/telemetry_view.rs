/// # Telemetry View
///
/// This component provides a view for displaying telemetry data from devices.
/// It allows users to:
/// - Select a device by ID
/// - View the latest telemetry data for the device
/// - See charts of temperature and voltage history
/// - Refresh the data

use crate::components::ApexChart;
use crate::domain::telemetry::Telemetry;
use crate::services::device_service::DeviceService;
use chrono::{DateTime, Utc};
use yew::prelude::*;

/// Properties for the TelemetryView component.
#[derive(Properties, PartialEq)]
pub struct TelemetryViewProps {
    /// ID of the device to display telemetry for
    pub device_id: String,
}

/// Component for displaying device telemetry data.
///
/// This component fetches and displays the latest telemetry data
/// for a specified device, including numeric values and charts.
#[function_component(TelemetryView)]
pub fn telemetry_view() -> Html {
    // State for the currently selected device ID
    let device_id = use_state(|| "4321".to_string());
    
    // State for the device ID input field
    let input_value = use_state(|| "4321".to_string());
    
    // State for the fetched telemetry data
    let telemetry_data = use_state(|| None::<Telemetry>);
    
    // State for tracking loading status
    let loading = use_state(|| true);
    
    // State for error messages
    let error = use_state(|| None::<String>);
    
    // Counter for triggering data refresh
    let refresh_count = use_state(|| 0);

    // Callback for handling changes in the device ID input field
    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            // Get the input element and its current value
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            // Update the input_value state
            input_value.set(input.value());
        })
    };

    // Callback for handling form submission
    let on_submit = {
        let device_id = device_id.clone();
        let input_value = input_value.clone();
        let error = error.clone();
        Callback::from(move |e: yew::events::SubmitEvent| {
            // Prevent default form submission behavior (page reload)
            e.prevent_default();
            
            // Validate the input
            if input_value.trim().is_empty() {
                // Show error if input is empty
                error.set(Some("Please enter a device ID.".to_string()));
            } else {
                // Update the device_id state with the input value
                // This will trigger a data fetch via the use_effect hook
                device_id.set((*input_value).clone());
            }
        })
    };

    // Callback for handling refresh button clicks
    let refresh_count_setter = refresh_count.clone();
    let on_refresh = Callback::from(move |_| {
        // Increment the refresh counter to trigger a data refresh
        refresh_count_setter.set(*refresh_count_setter + 1);
    });

    // Effect hook for fetching telemetry data when device_id or refresh_count changes
    {
        // Clone state variables to use in the effect closure
        let telemetry_data = telemetry_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let device_id = device_id.clone();
        let refresh_count = refresh_count.clone();
        
        // Set up effect that runs when device_id or refresh_count changes
        use_effect_with(((*device_id).clone(), *refresh_count), move |(device_id, _)| {
            let device_id = device_id.clone();
            
            // Set loading state and clear any previous errors
            loading.set(true);
            error.set(None);

            // Validate device ID before making API request
            if device_id.trim().is_empty() {
                error.set(Some("Please enter a device ID.".to_string()));
                loading.set(false);
                telemetry_data.set(None);
            } else {
                // Spawn an async task to fetch the data
                wasm_bindgen_futures::spawn_local(async move {
                    // Call the API service to get latest telemetry
                    match DeviceService::get_latest_telemetry(&device_id).await {
                        // Success case
                        Ok(data) => {
                            // Update state with the fetched data
                            telemetry_data.set(Some(data));
                            loading.set(false);
                        }
                        // Error case
                        Err(e) => {
                            // Handle different error scenarios with user-friendly messages
                            if e.contains("No telemetry data found") {
                                error.set(Some(
                                    "No telemetry data found for this device ID.".to_string(),
                                ));
                            } else if e.contains("404") {
                                error.set(Some(
                                    "Device not found. Please check the device ID.".to_string(),
                                ));
                            } else {
                                error.set(Some(e));
                            }
                            loading.set(false);
                        }
                    }
                });
            }
            
            // Cleanup function (no-op in this case)
            || ()
        });
    }

    html! {
        <div class="w-full bg-white rounded-xl shadow-md p-8 mt-8">
            <div class="mb-6">
                <h2 class="text-3xl font-bold text-gray-800 mb-2">{"Device Telemetry"}</h2>
                <form onsubmit={on_submit} class="flex flex-col sm:flex-row gap-2 items-end">
                    <div class="flex-1">
                        <label for="device-id" class="block text-sm font-medium text-gray-700 mb-1">{"Device ID"}</label>
                        <input
                            type="text"
                            id="device-id"
                            value={(*input_value).clone()}
                            oninput={on_input_change}
                            class="w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm px-3 py-2"
                            placeholder="Enter device ID"
                            autofocus=true
                        />
                    </div>
                    <button
                        type="submit"
                        class={format!(
                            "mt-2 sm:mt-0 px-6 py-2 rounded bg-blue-600 text-white font-semibold shadow hover:bg-blue-700 transition {}",
                            if *loading { "opacity-50 cursor-not-allowed" } else { "" }
                        )}
                        disabled={*loading}
                    >
                        { if *loading { html! { <span class="animate-spin mr-2">{"⏳"}</span> } } else { html!{} } }
                        {"Submit"}
                    </button>
                    <button
                        type="button"
                        onclick={on_refresh}
                        class="mt-2 sm:mt-0 px-4 py-2 rounded bg-gray-500 text-white font-semibold shadow hover:bg-gray-700 transition ml-2"
                        disabled={*loading}
                    >
                        { if *loading { html! { <span class="animate-spin mr-2">{"⏳"}</span> } } else { html!{} } }
                        {"Refresh"}
                    </button>
                </form>
            </div>

            if let Some(err) = error.as_ref() {
                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded flex items-center gap-2 mb-4 animate-fade-in">
                    <span>{"❌"}</span>
                    <span>{format!("Error: {}", err)}</span>
                </div>
            }

            if *loading {
                <div class="flex justify-center items-center h-32">
                    <div class="text-gray-500 animate-pulse">{"Loading telemetry data..."}</div>
                </div>
            } else if let Some(data) = telemetry_data.as_ref() {
                <div>
                    <div class="mb-6">
                        <p class="text-gray-600">{format!("Device ID: {}", data.device_id)}</p>
                        {
                            if let Some(timestamp) = data.timestamp {
                                html! {
                                    <p class="text-sm text-gray-500">
                                        {format!("Last updated: {}", format_timestamp(timestamp))}
                                    </p>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        {
                            get_sorted_telemetry_items(data).into_iter().map(|(key, value)| {
                                html! {
                                    <div class="bg-white p-4 rounded-lg shadow border">
                                        <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wide">
                                            {key}
                                        </h3>
                                        <p class="text-2xl font-semibold text-gray-900 mt-2">
                                            {format_value(key, value)}
                                        </p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            } else {
                <div class="text-center text-gray-500 py-8">
                    {"No telemetry data available"}
                </div>
            }
                    <div class="mt-8 grid grid-cols-1 lg:grid-cols-2 gap-6">
            <ApexChart
                key={format!("temperature-{}-{}", *device_id, *refresh_count)}
                metric_key="temperature"
                title="Temperature Over Time"
                device_id={(*device_id).clone()}
                refresh_count={*refresh_count}
            />
            <ApexChart
                key={format!("voltage-{}-{}", *device_id, *refresh_count)}
                metric_key="voltage"
                title="Voltage Over Time"
                device_id={(*device_id).clone()}
                refresh_count={*refresh_count}
            />
        </div>
        </div>
    }
}

/// Formats a Unix timestamp into a human-readable date string.
///
/// # Parameters
/// * `timestamp` - Unix timestamp (seconds since epoch)
///
/// # Returns
/// * Formatted date string in "YYYY-MM-DD HH:MM:SS UTC" format
/// * If conversion fails, returns the raw timestamp as string
fn format_timestamp(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| format!("{}", timestamp))
}

/// Formats a telemetry value with appropriate units based on the metric type.
///
/// # Parameters
/// * `key` - Name of the telemetry metric (e.g., "temperature")
/// * `value` - Raw value as string
///
/// # Returns
/// * Formatted value with appropriate units
fn format_value(key: &str, value: &str) -> String {
    match key.to_lowercase().as_str() {
        "temperature" => format!("{}°C", value),  // Add Celsius units
        "pressure" => format!("{} hPa", value),   // Add hectopascal units
        "voltage" => format!("{}V", value),       // Add volt units
        _ => value.to_string(),                   // Use raw value for unknown metrics
    }
}

/// Extracts and sorts telemetry items from a Telemetry object.
///
/// # Parameters
/// * `data` - Telemetry object containing sensor readings
///
/// # Returns
/// * Vector of (metric_name, value) pairs, sorted alphabetically by metric name
fn get_sorted_telemetry_items(data: &Telemetry) -> Vec<(&str, &str)> {
    // Extract key-value pairs from the telemetry data
    let mut items: Vec<_> = data
        .telemetry_data
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    
    // Sort alphabetically by key
    items.sort_by_key(|(key, _)| *key);
    
    items
}
