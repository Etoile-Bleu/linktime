use scattered_collect::{gather, scatter, slice::ScatteredSlice};

#[gather]
pub static COLLECTION: ScatteredSlice<fn()> = ScatteredSlice::new();

#[scatter(COLLECTION)]
fn my_function() {
    println!("my_function");
}

pub fn main() {}
