use eframe::wasm_bindgen::JsCast as _;
use eframe::{WebLogger, WebOptions, WebRunner};
use funnel_web::core::MainWindow;
use log::LevelFilter;

#[cfg(target_arch = "wasm32")]
fn main() {
    WebLogger::init(LevelFilter::Debug).ok();

    let web_options = WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let is_mobile = web_sys::window()
            .expect("No window")
            .inner_width()
            .expect("Failed to get window width")
            .as_f64()
            .unwrap_or(0.0)
            <= 768.0;

        if is_mobile {
            document.get_element_by_id("loading_text").unwrap().remove();
            let mobile_warning = document.get_element_by_id("mobile-warning").unwrap();
            mobile_warning.set_inner_html("<p>This app is not compatible with small screens.</p>");
            mobile_warning.set_class_name("mobile-warning");
            return;
        }

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))),
            )
            .await;

        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
