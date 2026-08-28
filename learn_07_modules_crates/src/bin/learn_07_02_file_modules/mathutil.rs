// mathutil.rs — a file-based module for learn_07_02_file_modules.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Nested inline submodule inside a file module.
pub mod geometry {
    pub fn circle_area(r: f64) -> f64 {
        std::f64::consts::PI * r * r
    }
}
