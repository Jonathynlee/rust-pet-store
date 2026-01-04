use uuid::Uuid;

#[derive(Debug)]
pub struct Pet {
    pub name: String,
    pub animal_type: String,
    pub price: f64,
    pub owner_id: Option<Uuid>,
    pub id: Uuid
}

impl Pet {
    pub fn get_price(&self) -> f64{
        self.price
    }
}

pub fn new_pet(name:String, price:f64, breed: String)-> Pet{
    let pet = Pet {
        name,
        price,
        id: Uuid::new_v4(),
        owner_id: None,
        animal_type: breed
    };

    pet
}