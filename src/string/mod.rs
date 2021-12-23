pub fn run(){
    second_word();    
    let s = String::from("OK ");
    println!("count: {}", count_word(s));  
}

fn count_word(s: String) -> i32 {

    for (i, item) in s.chars().enumerate() {
        if item == ' ' {
            return i as i32;
        }
    }
    s.len() as i32
}


fn second_word () {
    println!("Hello 2nd");
}