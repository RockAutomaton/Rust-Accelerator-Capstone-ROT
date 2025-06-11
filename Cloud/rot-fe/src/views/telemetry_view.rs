// src/components/telemetry_view.rs
use crate::components::ApexChart;
use crate::domain::telemetry::Telemetry;
use crate::services::device_service::DeviceService;
use chrono::{DateTime, Utc};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TelemetryViewProps {
    pub device_id: String,
}

#[function_component(TelemetryView)]
pub fn telemetry_view() -> Html {
    let device_id = use_state(|| "4321".to_string());
    let input_value = use_state(|| "4321".to_string());
    let telemetry_data = use_state(|| None::<Telemetry>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let refresh_count = use_state(|| 0);

    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let on_submit = {
        let device_id = device_id.clone();
        let input_value = input_value.clone();
        let error = error.clone();
        Callback::from(move |e: yew::events::SubmitEvent| {
            e.prevent_default();
            if input_value.trim().is_empty() {
                error.set(Some("Please enter a device ID.".to_string()));
            } else {
                device_id.set((*input_value).clone());
            }
        })
    };

    let refresh_count_setter = refresh_count.clone();
    let on_refresh = Callback::from(move |_| {
        refresh_count_setter.set(*refresh_count_setter + 1);
    });

    {
        let telemetry_data = telemetry_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let device_id = device_id.clone();
        let refresh_count = refresh_count.clone();
        use_effect_with(((*device_id).clone(), *refresh_count), move |(device_id, _)| {
            let device_id = device_id.clone();
            loading.set(true);
            error.set(None);

            if device_id.trim().is_empty() {
                error.set(Some("Please enter a device ID.".to_string()));
                loading.set(false);
                telemetry_data.set(None);
            } else {
                wasm_bindgen_futures::spawn_local(async move {
                    match DeviceService::get_latest_telemetry(&device_id).await {
                        Ok(data) => {
                            telemetry_data.set(Some(data));
                            loading.set(false);
                        }
                        Err(e) => {
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

fn format_timestamp(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| format!("{}", timestamp))
}

fn format_value(key: &str, value: &str) -> String {
    match key.to_lowercase().as_str() {
        "temperature" => format!("{}°C", value),
        "pressure" => format!("{} hPa", value),
        "voltage" => format!("{}V", value),
        _ => value.to_string(),
    }
}

fn get_sorted_telemetry_items(data: &Telemetry) -> Vec<(&str, &str)> {
    let mut items: Vec<_> = data
        .telemetry_data
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    items.sort_by_key(|(key, _)| *key);
    items
}
