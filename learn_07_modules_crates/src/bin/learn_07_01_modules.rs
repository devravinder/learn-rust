// learn_07_01_modules — mod, pub, use, paths.
// Run: cargo run --bin learn_07_01_modules
// Modules namespace your code (like TS namespaces / Java packages).
// Items are PRIVATE by default; `pub` exposes them.

mod restaurant {
    pub mod front {
        // pub fn is callable from outside the module.
        pub fn seat() -> String {
            // super:: refers to the parent module.
            format!("seated; {}", super::back::prep())
        }
    }

    pub mod back {
        pub fn prep() -> String {
            "food prepped".to_string()
        }
        fn _secret_recipe() {} // private: not visible outside `back`
    }
}

// `use` brings a path into scope (like an import).
use restaurant::front;

fn main() {
    // Full path:
    println!("{}", restaurant::front::seat());
    // Via `use`:
    println!("{}", front::seat());
    println!("{}", restaurant::back::prep());
}
