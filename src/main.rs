use vibe::{innit, Vibe};

fn main() {
    let vibe = innit();
    let mut state = State {
        vibe,
        cube: Cube {
            position: Position::new(0., 0., 0.),
            length: 1.,
        },
    };
    loop {
        state.update();
        println!("{state:?}");
        state.vibe.render();
    }
}

#[derive(Debug)]
struct State {
    vibe: Vibe,
    cube: Cube,
}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(&mut self, p2: Position) {
        self.x += p2.x;
        self.y += p2.y;
        self.z += p2.z;
    }
}

#[derive(Debug)]
struct Cube {
    position: Position,
    length: f32,
}

impl State {
    fn update(&mut self) {
        self.cube.position.add(Position {
            x: 0.,
            y: 1.,
            z: 0.,
        });
        self.cube.length += 0.001;
    }
}
