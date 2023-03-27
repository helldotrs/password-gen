use rand::Rng;
use std ::io;

fn main() {
    println!("Hello, world!");
    //type input (full/upper/lower/num/special/extra-special/string)
    //length input 

    let     _uppers      = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let     _lowers      = "abcdefghijklmnopqrstuvwxyz";
    let     _numbers     = "0123456789";
    let     _specials    = "!@#£$%&[]+?*-_.:,;";
    let     _extra      = "¤";

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
//fixme: PSUDOCODE BELOW!!
fn decide_scan_type(x) -> String {
    let out_var = "";
    is x contaains s:
        return content_of_string;
    if x contains f:
        return upper+lower+numbers+specials+extra;
    
    if x contains u:
        out_var += upper;
    if x contains l:
        out_var += lower;
    //and so on

    return out_var;
}
