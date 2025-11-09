use std::io;

fn main() {

    println!("

                      Menu                       Price 

         P = Poundo Yam/Edinkaiko Soup         - #3,200             
         F = Fried Rice and Chicken            - #3,000
         A = Amala and Ewedu Soup              - #2,500
         E = Eba and Egusi Soup                - #2,000
         W = White Rice and Stew               - #2,500

        ");

              let mut total:f32 = 0.0;  
    
    loop {

    let mut code = String::new();
    let mut quantity = String::new();
    

    println!("Enter a food code (P/F/A/E/W) or N to finish order");
    io::stdin().read_line(&mut code).expect("Not a valid code");
    let code = code.trim().to_uppercase();

    if code == "N" {

        break;
    }



    let _price:f32;

    let (food, price) = match code.as_str() {

        "P" => ("Poundo Yam/Edinkaiko Soup", 3200.0),
        "F" => ("Fried Rice and Chicken", 3000.0),
        "A" => ("Amala and Ewedu Soup", 2500.0),
        "E" => ("Eba and Egusi Soup", 2000.0),
        "W" => ("White Rice and Stew", 2500.0),

        _=> {

            println!("Invalid food code!, Try Again");

            continue;
        }
    
   };


    println!("Enter your quantity of {}", food);
    io::stdin().read_line(&mut quantity).expect("Not a valid number");
    let quantity:f32 = quantity.trim().parse().expect("You can only enter numbers");

    
        

    let subtotal:f32 = price * quantity;

    total += subtotal;

    if total > 10000.0 {

        let discount:f32 = total * 0.05;
        total = total - discount;

        println!("You get a discount of 5 percent of #{}", discount);
    }



    println!("Your total price of {} is #{}", food, total);


}
}
