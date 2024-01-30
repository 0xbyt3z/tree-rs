pub fn does_flag_expect_a_value(flag: String) -> bool {
    let valued_flags :Vec<String> = String::from("-p -l").split_whitespace().map(String::from).collect();
    if valued_flags.contains(&flag) {
        return true
    }
    return false
}


pub fn help(){
    println!("This will be printed to provide help")
}
