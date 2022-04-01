trait State {
    fn enter(&mut self, shared: &mut Shared);

    fn update(&mut self, shared: &mut Shared) -> Option<Box<dyn State>>;

    fn exit(&mut self, shared: &mut Shared);
}

struct Shared {

}

struct StateMachine {
    state: Box<dyn State>,
    shared: Shared,
}

impl StateMachine {
    fn update(&mut self) {
        if let Some(new_state) = self.state.update(&mut self.shared) {
            self.state.exit(&mut self.shared);
            self.state = new_state;
            self.state.enter(&mut self.shared);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
