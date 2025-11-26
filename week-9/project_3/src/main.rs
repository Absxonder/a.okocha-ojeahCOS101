fn main() {

    let commisioners = vec![

    "Aigbogun Alamba Daudun",

    "Murtala Afeez Bendu",

    "Okorocha Calistus Ogbona",

    "Adewale Jimoh Akanbi",

    "Osazuwa Faith Etieye",
    ];


    let ministries = vec![

    "Internal Affairs",

    "Justice",

    "Defense",

    "Power & Steel",

    "Petroleum",
    ];


    let geopolitical_zone = vec![

    "South West",

    "North East",

    "South South",

    "South West",

    "South East",

    ];


   println!("__________________________________________________________________________________________________");

    println!("| S/N |COMMISSIONER        | MINISTRY  | GEOPOLITICAL ZONE |");

    println!("__________________________________________________________________________________________________");

    for i in 1..commisioners.len() {  
        
        println!("| {} | {} | {} | {} |",i,commisioners[i], ministries[i], geopolitical_zone[i]);
    }

    println!("__________________________________________________________________________________________________");
}