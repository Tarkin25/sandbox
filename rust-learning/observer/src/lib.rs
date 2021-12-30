mod observable;
mod event_emitter;
mod subscription;

pub use event_emitter::EventEmitter;
pub use subscription::Subscription;
pub use observable::Observable;

pub trait OnEvent<T> {
    fn on_event(&mut self, event: T);
}

impl<T, F: FnMut(T)> OnEvent<T> for F {
    fn on_event(&mut self, event: T) {
        self(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut emitter = EventEmitter::new();
        const CLICK: &str = "click";
        const HOVER: &str = "hover";
        let click_sub = emitter.subscribe(CLICK, |target: &str| println!("clicked on {}", target));
        let hover_sub = emitter.subscribe(HOVER, |target: &str| println!("hovered over {}", target));

        emitter.emit_event(CLICK, "button");
        emitter.emit_event(HOVER, "button");

        emitter.unsubscribe(CLICK, click_sub);
        emitter.unsubscribe(HOVER, hover_sub);

        emitter.emit_event(CLICK, "button");
        emitter.emit_event(HOVER, "button");
    }
}
