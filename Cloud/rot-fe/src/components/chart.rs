use yew::prelude::*;
use web_sys::{window, Element};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde::Serialize;
use crate::services::device_service::DeviceService;
use crate::domain::telemetry::Telemetry;
use chrono::{DateTime, Utc};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = ApexCharts)]
    type ApexCharts;
    
    #[wasm_bindgen(constructor, js_namespace = window, js_class = ApexCharts)]
    fn new(element: &Element, options: &JsValue) -> ApexCharts;
    
    #[wasm_bindgen(method)]
    fn render(this: &ApexCharts);
    
    #[wasm_bindgen(method, js_name = destroy)]
    fn destroy(this: &ApexCharts);
    
    #[wasm_bindgen(method, js_name = updateSeries)]
    fn update_series(this: &ApexCharts, series: &JsValue);
}

#[derive(Serialize)]
struct ChartOptions {
    chart: ChartType,
    series: Vec<Series>,
    xaxis: XAxis,
    yaxis: YAxis,
    title: Title,
    stroke: Stroke,
    markers: Markers,
}

#[derive(Serialize)]
struct ChartType {
    #[serde(rename = "type")]
    chart_type: String,
    width: String,
    height: String,
    animations: Animations,
}

#[derive(Serialize)]
struct Animations {
    enabled: bool,
}

#[derive(Serialize)]
struct Series {
    name: String,
    data: Vec<DataPoint>,
}

#[derive(Serialize)]
struct DataPoint {
    x: String, // timestamp as string
    y: f64,    // value as number
}

#[derive(Serialize)]
struct XAxis {
    #[serde(rename = "type")]
    axis_type: String,
    title: AxisTitle,
}

#[derive(Serialize)]
struct YAxis {
    title: AxisTitle,
}

#[derive(Serialize)]
struct AxisTitle {
    text: String,
}

#[derive(Serialize)]
struct Title {
    text: String,
    align: String,
}

#[derive(Serialize)]
struct Stroke {
    curve: String,
    width: u32,
}

#[derive(Serialize)]
struct Markers {
    size: u32,
}

#[derive(Properties, PartialEq)]
pub struct ApexChartProps {
    pub metric_key: String, // Which telemetry key to chart (e.g., "temperature")
    pub title: String,      // Chart title
    pub device_id: String,  // Device ID to fetch data for
}

#[function_component(ApexChart)]
pub fn apex_chart(props: &ApexChartProps) -> Html {
    let chart_ref = use_node_ref();
    let chart_instance = use_state(|| None::<ApexCharts>);
    let telemetry_data = use_state(|| Vec::<Telemetry>::new());
    let loading = use_state(|| true);

    // Fetch telemetry data
    {
        let telemetry_data = telemetry_data.clone();
        let loading = loading.clone();
        let device_id = props.device_id.clone();
        
        use_effect_with(props.device_id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match DeviceService::get_telemetry(&device_id).await {
                    Ok(data) => {
                        telemetry_data.set(data);
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Failed to fetch telemetry: {}", e).into());
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Create/update chart when data changes
    {
        let chart_ref = chart_ref.clone();
        let chart_instance = chart_instance.clone();
        let telemetry_data = telemetry_data.clone();
        let metric_key = props.metric_key.clone();
        let title = props.title.clone();
        let loading = *loading;
        
        use_effect_with((telemetry_data.clone(), loading), move |_| {
            if !loading {
                if let Some(element) = chart_ref.cast::<Element>() {
                    // Prepare chart data
                    let chart_data = prepare_chart_data(&telemetry_data, &metric_key);
                    
                    if let Some(existing_chart) = chart_instance.as_ref() {
                        // Update existing chart
                        if let Ok(series_js) = to_value(&chart_data) {
                            existing_chart.update_series(&series_js);
                        }
                    } else if !chart_data.is_empty() {
                        // Create new chart
                        let options = ChartOptions {
                            chart: ChartType {
                                chart_type: "line".to_string(),
                                width: "100%".to_string(),
                                height: "350".to_string(),
                                animations: Animations { enabled: true },
                            },
                            series: vec![Series {
                                name: metric_key.clone(),
                                data: chart_data,
                            }],
                            xaxis: XAxis {
                                axis_type: "datetime".to_string(),
                                title: AxisTitle {
                                    text: "Time".to_string(),
                                },
                            },
                            yaxis: YAxis {
                                title: AxisTitle {
                                    text: get_unit_for_metric(&metric_key),
                                },
                            },
                            title: Title {
                                text: title.clone(),
                                align: "left".to_string(),
                            },
                            stroke: Stroke {
                                curve: "smooth".to_string(),
                                width: 2,
                            },
                            markers: Markers { size: 4 },
                        };
                        
                        if let Ok(options_js) = to_value(&options) {
                            let chart = ApexCharts::new(&element, &options_js);
                            chart.render();
                            chart_instance.set(Some(chart));
                        }
                    }
                }
            }
            || ()
        });
    }

    html! {
        <div class="bg-white p-5 rounded-lg shadow-lg">
            <h3 class="text-lg font-semibold mb-4">{&props.title}</h3>
            {
                if *loading {
                    html! {
                        <div class="flex justify-center items-center h-80">
                            <div class="text-gray-500">{"Loading chart data..."}</div>
                        </div>
                    }
                } else {
                    html! { <div ref={chart_ref}></div> }
                }
            }
        </div>
    }
}

fn prepare_chart_data(telemetry_data: &[Telemetry], metric_key: &str) -> Vec<DataPoint> {
    telemetry_data
        .iter()
        .filter_map(|telemetry| {
            // Get the value for the specific metric
            let value = telemetry.telemetry_data.get(metric_key)?;
            
            // Parse the value as a number
            let numeric_value: f64 = value.parse().ok()?;
            
            // Format timestamp
            let timestamp = telemetry.timestamp?;
            let datetime = DateTime::from_timestamp(timestamp, 0)?;
            let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            
            Some(DataPoint {
                x: formatted_time,
                y: numeric_value,
            })
        })
        .collect()
}

fn get_unit_for_metric(metric_key: &str) -> String {
    match metric_key.to_lowercase().as_str() {
        "temperature" => "Temperature (°C)".to_string(),
        "humidity" => "Humidity (%)".to_string(),
        "pressure" => "Pressure (hPa)".to_string(),
        "voltage" => "Voltage (V)".to_string(),
        _ => metric_key.to_string(),
    }
}