use scattered_collect::{gather, scatter, sorted_slice::ScatteredSortedSlice};

// #[gather]
// pub static COLLECTION: ScatteredSortedSlice<fn()> = ScatteredSortedSlice::new();

#[scatter(COLLECTION)]
fn my_function() {
    println!("my_function");
}

pub fn main() {}
