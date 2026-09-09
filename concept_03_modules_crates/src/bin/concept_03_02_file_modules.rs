// concept_03_02_file_modules — modules split across files.
// Run: cargo run --bin concept_03_02_file_modules
// `mod mathutil;` loads src/bin/concept_03_02_file_modules/mathutil.rs.
#[path = "concept_03_02_file_modules/mathutil.rs"]
mod mathutil; // declares the module; #[path] points to the file explicitly

use mathutil::geometry;

fn main() {
    println!("add = {}", mathutil::add(2, 3));
    println!("circle area = {:.2}", geometry::circle_area(2.0));
}
