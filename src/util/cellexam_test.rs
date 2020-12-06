use std::cell::Cell;
use std::string::String;
use super::cellexam::House;

#[test]
fn unit_test_house() {
    let my_house = House {
        bedrooms: 2,
        name: &"2 houses",
    };
    let my_dream_house = House {
        bedrooms: 5,
        name: &"5 houses",
    };

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

#[test]
fn compare_owner_and_reference() {
    let s1 = String::from("House 1");
    let my_house = &mut House {
        bedrooms: 2,
        name: &s1.as_str(),
    };
    let s2 = String::from("House 2");
    my_house.set_name_string(&s2);
    println!("The s1 is {:?}", s1);
    let s3 = String::from("House 3");
    my_house.set_name_str(&s3.as_str());
    println!("The s3 is {:?}", s3);
}
