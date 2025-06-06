use yew::prelude::*;
use web_sys::{window, Element};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde::Serialize;

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
}

#[derive(Serialize)]
struct ChartOptions<'a> {
    chart: ChartType<'a>,
    series: [Series<'a>; 1],
    xaxis: XAxis<'a>,
}

#[derive(Serialize)]
struct ChartType<'a> {
    #[serde(rename = "type")]
    chart_type: &'a str,
    width: &'a str,
    height: &'a str,
}

#[derive(Serialize)]
struct Series<'a> {
    name: &'a str,
    data: [u32; 9],
}

#[derive(Serialize)]
struct XAxis<'a> {
    categories: [&'a str; 9],
}

#[function_component(ApexChart)]
pub fn apex_chart() -> Html {
    let chart_ref = use_node_ref();
    
    use_effect_with((), {
        let chart_ref = chart_ref.clone();
        move |_| {
            if let Some(element) = chart_ref.cast::<Element>() {
                let options = ChartOptions {
                    chart: ChartType {
                        chart_type: "line",
                        width: "100%",
                        height: "350",
                    },
                    series: [Series {
                        name: "Sales",
                        data: [30, 40, 35, 50, 49, 60, 70, 91, 125],
                    }],
                    xaxis: XAxis {
                        categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep"],
                    },
                };
                
                let chart = ApexCharts::new(&element, &to_value(&options).unwrap());
                chart.render();
            }
            || ()
        }
    });

    html! {
        <div class="bg-white p-5 rounded-lg shadow-lg">
            <h3 class="text-lg font-semibold mb-4">{"RoT Data"}</h3>
            <div ref={chart_ref}></div>
        </div>
    }
}