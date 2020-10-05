extern crate flexi_logger;

use log::info;

pub trait Layer {
    fn new() -> Self;

    fn name(&self) -> &'static str;

    fn on_attach(&self);

    fn on_detach(&self);

    fn on_update(&self);
    fn on_event(&self);
}

pub struct DebugLayer {
    name: &'static str,
}

impl Layer for DebugLayer {
    fn new() -> DebugLayer {
        DebugLayer {
            name: "debug_layer",
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn on_attach(&self) {
        info!("on_attach");
    }

    fn on_detach(&self) {
        info!("on_detach");
    }

    fn on_update(&self) {
        info!("on_update");
    }
    fn on_event(&self) {
        info!("on_event");
    }
}
