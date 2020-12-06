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