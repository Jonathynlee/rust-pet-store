use uuid::Uuid;

#[derive(Debug)]
pub struct Customer {
    pub name: String,
    pub money: f64,
    pub id: Uuid,
    pub owned_animals: Vec<Uuid>
}

pub fn new_customer(name:String, money:f64)-> Customer{
    let customer = Customer {
        name,
        money,
        id: Uuid::new_v4(),
        owned_animals: vec!()
    };

    customer
}