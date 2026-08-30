#![allow(unused)]
// learn_05_02_trait_objects — static dispatch (generics) vs dynamic (dyn).
// Run: cargo run --bin learn_05_02_trait_objects
// impl Trait / <T: Trait> = compile-time (fast). dyn Trait = runtime (flexible).

// Dispatch = overloading/polymorphism

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
// Box is pointer that stores a value on the heap  ( instead of directly on the stack )
// Box is pointer on heap
// run time polymorphism
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

    println!("--------&dyn Draw-----------");

    let input = "button";

    // run time polymorphism 
    // we are returing &(reference) -> bcz rust should know size at compile time
    let draw : &dyn Draw = match input {
         "button" => &Button,
        _ => &Checkbox
    };

    println!("selected: {}",draw.draw());

    println!("--------Box<dyn Draw>-----------");
    let input = "checkbox";

    let draw: Box<dyn Draw> = match input {
        "button" => Box::new(Button),
        _ => Box::new(Checkbox),
    };

    println!("selected: {}", draw.draw());

    println!("----------combined traits-------");
    let dog = Dog;
    dog.eat();
    dog.play();

    println!("-------Pet Animal---------");
    let police_dog: PoliceDog = PoliceDog;
    police_dog.eat();
    police_dog.work();

    println!("---------------Ambiguity = needs fully qualified syntax--------------");
    // multiple applicable items in scope

    let square = Square{
        color: "Blue".to_string(),
        side: 10,
    };

    // square.get();// error: multiple applicable items in scope

    let color = Color::get(&square);
    let (height, width) = Rectangle::get(&square);
    println!("color:{color}, heidht:{height}, width:{width}")


}

// combined traits

trait Animal {
    fn eat(&self) {
        println!("Eating");
    }
}

// Pet requires Animal
trait Pet: Animal {
    fn play(&self) {
        println!("Playing");
    }
}

struct Dog;

impl Animal for Dog {}

impl Pet for Dog {
    fn play(&self) {
        println!("Pet Dog..");
        Animal::eat(self);
        println!("then playing")
    }
}

// another way

trait Trained {
    fn work(&self){
      println!("working as trained");
    }
}

// TrainedAnimal requires both Pet and Trained
trait TrainedAnimal: Pet + Trained {   }

struct PoliceDog;

impl Animal for PoliceDog {}

impl Pet for PoliceDog {}

impl Trained for PoliceDog {}

impl TrainedAnimal for PoliceDog {}

//====Ambiguity = needs fully qualified syntax

trait Color {
    fn get(&self) -> String;
}

trait Rectangle {
    fn get(&self) -> (i32, i32); // heidht, width
}

struct Square {
    color: String,
    side: i32,
}

impl Color for Square {
    fn get(&self) -> String {
        // todo!()
        self.color.clone()
    }
}

impl Rectangle for Square {
    fn get(&self) -> (i32, i32) {
        (self.side, self.side)
    }
}