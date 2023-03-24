use rand::Rng;

fn main() {
    println!("Hello, world!");
    //type input (full/upper/lower/num/special/extra-special/string)
    //length input 

    let     characters  = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let     chars_len   = characters.len();
    let     passw_len   = 16;

    let mut password    = String::new();

    let mut i           = 0;
    loop{
        let random = rand::thread_rng().gen_range(0, chars_len);
        password.push(characters.chars().nth(rand_index).unwrap()); //look deeper into nth()

        i += 1;
        if i >= passw_len {
            break;
        }
    }

    println!("password: {}", password);
    println!("created by hellmak at GitHub, cyberSecHell at Twitter");
}
