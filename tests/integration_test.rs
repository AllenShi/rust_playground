// extern crate rust_playground;

use std::cell::Cell;
// use crate::rust_playground::util::cellexam::House;
use rust_playground::util::cellexam::House;

mod common;

#[test]
fn int_test_house_with_cell() {
    common::setup();
    let my_house = House { bedrooms: 2, name: &"my house" };
    let my_dream_house = House { bedrooms: 5, name: &"dream house" };


    let my_cell = Cell::new(my_house);
    my_house.print_house();
    println!("My house has {} bedrooms", my_cell.get().bedrooms);

    my_cell.set(my_dream_house);
    println!("(My new house has {} bedrooms.", my_cell.get().bedrooms);

    let my_new_old_house = my_cell.replace(my_house);
    println!(
        "My house has {} bedrooms, it was better with {}",
        my_cell.get().bedrooms,
        my_new_old_house.bedrooms
    );

    let my_new_cell = Cell::new(my_dream_house);
    my_cell.swap(&my_new_cell);
    println!(
        "Yay! my current house has {} bedrooms! (my new house {})",
        my_cell.get().bedrooms,
        my_new_cell.get().bedrooms
    );

    let my_final_house = my_cell.take();
    println!(
        "My final house has {} bedrooms, the shared one {}",
        my_final_house.bedrooms,
        my_cell.get().bedrooms
    );
}