
#[derive(Copy, Clone)]
pub struct House {
    pub bedrooms: u8
}

impl Default for House {
    fn default() -> Self {
        House { bedrooms: 1 }
    }
}

impl House {
    pub fn print_house(&self) {
        println!("The house has {} bedrooms", self.bedrooms);
    }
}

#[cfg(test)]
mod tests {

    use std::cell::Cell;
    use super::*;

    #[test]
    fn unit_test_house() {
        let my_house = House { bedrooms: 2 };
        let my_dream_house = House { bedrooms: 5 };

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
}
