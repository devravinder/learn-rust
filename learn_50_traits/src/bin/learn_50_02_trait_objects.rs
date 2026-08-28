// learn_50_02_trait_objects — static dispatch (generics) vs dynamic (dyn).
// Run: cargo run --bin learn_50_02_trait_objects
// impl Trait / <T: Trait> = compile-time (fast). dyn Trait = runtime (flexible).
trait Draw {
    fn draw(&self) -> String;
}

struct Button;
struct Checkbox;
impl Draw for Button {
    fn draw(&self) -> String {
        "[Button]".into()
    }
}
impl Draw for Checkbox {
    fn draw(&self) -> String {
        "[x]".into()
    }
}

// Static dispatch: one concrete type per call, resolved at compile time.
fn render_static(item: &impl Draw) {
    println!("static: {}", item.draw());
}

// Dynamic dispatch: a heterogeneous list of different types behind &dyn.
fn render_all(items: &[Box<dyn Draw>]) {
    for item in items {
        println!("dyn: {}", item.draw());
    }
}

fn main() {
    render_static(&Button);
    render_static(&Checkbox);

    let widgets: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(Checkbox)];
    render_all(&widgets);
}
