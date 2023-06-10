pub fn inspect(arg: &String){
    if arg.ends_with("s"){
        println!("{} is plural.", arg)
    } else {
        println!("{} is singular.", arg)
    }
}

pub fn change(arg: &mut String){
    if !arg.ends_with("s"){
        arg.push_str("s");
    }
}

pub fn eat(arg: String) -> bool {
    if arg.starts_with("b") && arg.contains("a") { 
        true 
    } else {
        false
    }
}


pub fn bedazzle(material: &mut String) {
    // (*material).insert_str(0, "sparkly "); // dereferences, adds 'sparkly' to the begging on the string
    *material = String::from("sparkly");        // dereferences, replaces the string w/ the word 'sparkly'
}