use rand::Rng;
use std ::io;

fn main() {
    println!("Hello, world!");
    //type input (full/upper/lower/num/special/extra-special/string)
    //length input 

    let     uppers      = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let     lowers      = "abcdefghijklmnopqrstuvwxyz";
    let     numbers     = "0123456789";
    let     specials    = "!@#£$%&[]+?*-_.:,;";

    let mut characters  = "";
    let     chars_len   = characters.len();
    let     passw_len   = 16;



    let mut password    = String::new();

    let mut i           = 0;
    loop{
        let random = rand::thread_rng().gen_range(0, chars_len);
        password.push(characters.chars().nth(random).unwrap()); //look deeper into nth()

        i += 1;
        if i >= passw_len {
            break;
        }
    }

    println!("password: {}", password);
    println!("created by hellmak at GitHub, cyberSecHell at Twitter");
}

fn ask_for_input() -> String {
    let mut input   = String::new();

    println!("Password type [f]ull/[u]pper/[l]ower/[n]um/[s]pecial/[e]xtra-special/[s]ring:");
    io::stdin()
        .read_line(&mut input)
        .expect("failture to read input"); //if not reference, would value be dropped after this line?
    let input   = input
        .trim()
        .to_string();

    input
    

}
