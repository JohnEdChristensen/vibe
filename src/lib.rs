pub fn innit() -> Vibe {
    return Vibe { tree: Tree {} };
}

pub struct Vibe {
    tree: Tree,
}
impl Vibe {
    pub fn render(self) -> Self {
        Vibe {
            tree: self.tree.render(),
        }
    }
}

//// Tree
pub struct Tree {}

impl Tree {
    pub fn render(self) -> Self {
        self
    }
}
