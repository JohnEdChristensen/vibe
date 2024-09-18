pub fn innit() -> Vibe {
    return Vibe { tree: Tree {} };
}

#[derive(Debug)]
pub struct Vibe {
    tree: Tree,
}
impl Vibe {
    pub fn render(&mut self) {
        self.tree.render();
    }
}

//// Tree
#[derive(Debug)]
pub struct Tree {}

impl Tree {
    pub fn render(&mut self) {}
}
