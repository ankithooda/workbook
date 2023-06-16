mod front_of_the_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist.");
        }

        fn seat_at_table() {
            println!("Seated at table.")
        }
    }

    mod serving {
        fn take_order() {
            println!("Took order.");
        }

        fn serve_order() {
            println!("Served order.");
        }

        fn take_payment() {
            println!("Took payment.");
        }
        
    }
}

pub fn handle_new_customer() {
    crate::front_of_the_house::hosting::add_to_waitlist();
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
