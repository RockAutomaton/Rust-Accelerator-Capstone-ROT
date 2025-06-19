use crate::domain::config::DeviceConfig;
use crate::services::device_service::DeviceService;
use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct ConfigViewProps {
    pub device_id: String,
}

#[function_component(ConfigView)]
pub fn config_view() -> Html {
    let device_id = use_state(|| "".to_string());
    let input_value = use_state(|| "".to_string());
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);
    let led_status = use_state(|| "off".to_string());

    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let on_led_change = {
        let led_status = led_status.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target() {
                if let Some(input) = target.dyn_into::<web_sys::HtmlInputElement>().ok() {
                    let value = input.value();
                    web_sys::console::log_1(&format!("Radio button changed to: {}", value).into());
                    led_status.set(value);
                }
            }
        })
    };

    let on_push_config = {
        let device_id = device_id.clone();
        let input_value = input_value.clone();
        let led_status = led_status.clone();
        let error = error.clone();
        let success_message = success_message.clone();
        let loading = loading.clone();
        Callback::from(move |_| {
            let device_id = (*input_value).clone();
            let led_status = (*led_status).clone();
            let error = error.clone();
            let success_message = success_message.clone();
            let loading = loading.clone();

            if device_id.trim().is_empty() {
                error.set(Some("Please enter a device ID.".to_string()));
                return;
            }

            // Debug: Log the LED status being sent
            web_sys::console::log_1(&format!("Sending LED status: {}", led_status).into());

            loading.set(true);
            error.set(None);
            success_message.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let config = DeviceConfig {
                    device_id: device_id.clone(),
                    config: serde_json::json!({
                        "LED": led_status
                    }),
                };

                // Debug: Log the full config being sent
                web_sys::console::log_1(&format!("Sending config: {:?}", config).into());

                match DeviceService::update_device_config(&device_id, &config).await {
                    Ok(_) => {
                        success_message.set(Some(format!("Configuration pushed successfully to device {}!", device_id)));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to push configuration: {}", e)));
                        loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="w-full bg-white rounded-xl shadow-md p-8 mt-8">
            <div class="mb-6">
                <h2 class="text-3xl font-bold text-gray-800 mb-2">{"Device Configuration"}</h2>
                <p class="text-gray-600 mb-4">{"Build and push configuration to your IoT devices"}</p>
            </div>

            if let Some(err) = error.as_ref() {
                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded flex items-center gap-2 mb-4 animate-fade-in">
                    <span>{"❌"}</span>
                    <span>{format!("Error: {}", err)}</span>
                </div>
            }

            if let Some(success) = success_message.as_ref() {
                <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded flex items-center gap-2 mb-4 animate-fade-in">
                    <span>{"✅"}</span>
                    <span>{success}</span>
                </div>
            }

            <div class="bg-gray-50 p-6 rounded-lg">
                <h3 class="text-lg font-semibold text-gray-800 mb-4">{"Configuration Settings"}</h3>
                
                <div class="space-y-4">
                    <div>
                        <label for="device-id" class="block text-sm font-medium text-gray-700 mb-2">
                            {"Device ID"}
                        </label>
                        <input
                            type="text"
                            id="device-id"
                            value={(*input_value).clone()}
                            oninput={on_input_change}
                            class="w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm px-3 py-2"
                            placeholder="Enter device ID (e.g., 4321)"
                            autofocus=true
                        />
                        <p class="text-sm text-gray-500 mt-1">
                            {"The ID of the device you want to configure"}
                        </p>
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            {"LED Status"}
                        </label>
                        <div class="space-y-2">
                            <label class="flex items-center">
                                <input
                                    type="radio"
                                    name="led-status"
                                    value="off"
                                    checked={*led_status == "off"}
                                    onchange={on_led_change.clone()}
                                    class="mr-2"
                                />
                                <span>{"Off"}</span>
                            </label>
                            <label class="flex items-center">
                                <input
                                    type="radio"
                                    name="led-status"
                                    value="on"
                                    checked={*led_status == "on"}
                                    onchange={on_led_change}
                                    class="mr-2"
                                />
                                <span>{"On"}</span>
                            </label>
                        </div>
                        <p class="text-sm text-gray-500 mt-1">
                            {"Control the LED status on the device"}
                        </p>
                        <p class="text-sm text-blue-600 mt-2">
                            {format!("Current selection: {}", *led_status)}
                        </p>
                    </div>
                </div>

                <div class="mt-6">
                    <button
                        onclick={on_push_config}
                        disabled={*loading}
                        class={format!(
                            "px-6 py-2 rounded bg-green-600 text-white font-semibold shadow hover:bg-green-700 transition {}",
                            if *loading { "opacity-50 cursor-not-allowed" } else { "" }
                        )}
                    >
                        { if *loading { html! { <span class="animate-spin mr-2">{"⏳"}</span> } } else { html!{} } }
                        {"Push Configuration"}
                    </button>
                </div>
            </div>

            <div class="mt-6 bg-blue-50 border border-blue-200 text-blue-700 px-4 py-3 rounded">
                <h4 class="font-semibold mb-2">{"How it works:"}</h4>
                <ul class="text-sm space-y-1">
                    <li>{"1. Enter the device ID you want to configure"}</li>
                    <li>{"2. Select the desired LED status (on/off)"}</li>
                    <li>{"3. Click 'Push Configuration' to send the settings to the device"}</li>
                    <li>{"4. The device will download and apply the new configuration"}</li>
                </ul>
            </div>
        </div>
    }
} 