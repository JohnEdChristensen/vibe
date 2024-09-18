use vibe::{innit, Vibe};

fn main() {
    let vibe = innit();
    let mut state = State { vibe };
    loop {
        state = state.update();
    }
}

struct State {
    vibe: Vibe,
}

impl State {
    fn update(self) -> Self {
        State {
            vibe: self.vibe.render(),
        }
    }
}
