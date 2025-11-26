use std::io::Write;

fn main() {

    let table = "Lager      |       Stout       |       Non Alcoholic\n     

                33 Export   |       Legend      |      Maltina

                Desperados  |     Turbo King    |    Amstel Malta  

                Goldberg    |     Williams      |   Malta Gold

                Gulder      |                   |   Fayrouz

                Heineken    |                   |

                Star        |                   |
    ";

    let mut file = std::fs::File::create("High Quality of Drinks of Nigerian Breweries Plc.txt").expect("create failed");
    file.write_all(table.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}