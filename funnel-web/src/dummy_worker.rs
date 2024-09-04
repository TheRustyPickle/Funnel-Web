use gloo_worker::Registrable;

fn main() {
    funnel_web::web_worker::WebWorker::registrar().register();
}
