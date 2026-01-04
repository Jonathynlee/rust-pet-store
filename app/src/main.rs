use inquire::{Select, Text, error::InquireError, prompt_f64, prompt_u32};
// customers and pets
mod customer;
mod pets;
//


fn main() {
    
    let mut customers:Vec<customer::Customer> = vec!();
    let mut pets = create_pets();
    loop{


        let main_console_options = vec!("Add customer", "See Customers", "See Pets!", "Leave Store", "Customer Purchase");
        println!("Please select an action:");
        let first_selection: Result<&str, InquireError> = Select::new("Please select an action:", main_console_options).prompt();

        match first_selection {
            Ok(choice) => {
                match choice {
                    "Add customer" => {
                        handle_add_customer(&mut customers);
                    },
                    "See Customers" => {
                        handle_see_customer(&customers)
                    },
                    "See Pets!" => {
                        handle_see_pets(&pets);
                    },
                    "Customer Purchase" => {
                        handle_customer_purchase(&mut customers, &mut pets);
                    }
                    _ => {
                        println!("Thank you for shopping!");
                        break;
                    }
                }
            },
            Err(_) => println!("There was an error, please try again"),
        }


        // End Selection
        let end_options = vec!("Stay in the store", "Leave the store");
        let end_selection = Select::new("Please select an action:", end_options).prompt();
        match end_selection {
            Ok(choice) => {
                match choice {
                    "Stay in the store" => {
                        println!("Taking you back to the store!");
                    },
                    "Leave the store" => {
                        println!("Thank you for shopping!");
                        break
                    },
                    _ => {
                        break;
                    }
                }
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
    

}

fn handle_add_customer(current_customers: &mut Vec<customer::Customer>) {
    let customer_name = Text::new("What is the customers name?").prompt().unwrap();
    let customer_money = prompt_f64("How much money does this customer have?").unwrap();

    let new_customer = customer::new_customer(customer_name, customer_money);

    current_customers.push(new_customer);
}

fn handle_see_pets(pets_list:&Vec<pets::Pet>){
    for a_pet in pets_list {
        println!("{:#?}", a_pet);

    }
}

fn handle_see_customer(customer_list:&Vec<customer::Customer>){
    for a_customer in customer_list {
        println!("{:#?}", a_customer);
    }
}

fn handle_customer_purchase(customers_list: &mut Vec<customer::Customer> , pets_list: &mut Vec<pets::Pet> ){
    let mut continue_purchase = true;
    let mut customer_names_list: Vec<String> = vec!();

    for a_customer in customers_list.iter(){
        let customer_name = &a_customer.name;
        customer_names_list.push(customer_name.clone());
    }
    let selected_customer_name = Select::new("Who is buying?", customer_names_list.clone()).prompt().unwrap();
    let selected_customer_index = customer_names_list.iter().position(|name| name == &selected_customer_name).unwrap();
    
    for (index, a_pet) in pets_list.iter().enumerate(){
        println!("Option {index}");
        println!("{:#?}", a_pet);
    }

    println!("Use '-1' to exit.");

    let mut selected_pet_index: usize;
    loop {
        let temp_selected_pet_index:usize = prompt_u32("Please insert the number of the pet to be purchased:").unwrap() as usize;
        selected_pet_index = temp_selected_pet_index.clone();
        let pet_count = pets_list.len();

        println!("You selected pet index {}", temp_selected_pet_index);
        if temp_selected_pet_index < pet_count {
            let pet_price = pets_list[temp_selected_pet_index].get_price();
            if customers_list[selected_customer_index].money >= pet_price {
                break;
            } else {
                println!("{} does not have enough money to buy {}, please select another pet within their budget", customers_list[selected_customer_index].name, pets_list[temp_selected_pet_index].name);
            }
        } else {
            println!("Please insert a valid pet number (0-{}):", pet_count.saturating_sub(1));
        }

        let continue_options = vec!["Continue With Purchase", "Cancel Purchase"];
        let selected_continue_option = Select::new("How would you like to proceed?", continue_options).prompt().unwrap();
        match selected_continue_option {
            "Continue With Purchase" => {

            }
            "Cancel Purchase" => {
                continue_purchase = false;
                break
            }
            _ => {

            }
        }

        
    }

    if continue_purchase {
        // avoid borrow checker errors, collect pet id and customer id before mutating vectors.
        let selected_customer_id = customers_list[selected_customer_index].id.clone();
        let selected_pet_id = pets_list[selected_pet_index].id.clone();

        pets_list[selected_pet_index].owner_id = Some(selected_customer_id);
        customers_list[selected_customer_index].owned_animals.push(selected_pet_id);
        customers_list[selected_customer_index].money -= pets_list[selected_pet_index].price;
    }
    
}


fn create_pets() -> Vec<pets::Pet> {

    let pet1 = pets::new_pet("Sam".to_string(), 250.00, "Dog - Dalmation".to_string());
    let pet2 = pets::new_pet("Fuzz".to_string(), 125.50, "Cat - Orange".to_string());
    let pet3 = pets::new_pet("Cornel".to_string(), 67.75, "Bird - Parrot".to_string());
    let pet4 = pets::new_pet("Jack".to_string(), 420.00, "Monkey - Eyy'".to_string());

    vec!(pet1, pet2, pet3, pet4)
}