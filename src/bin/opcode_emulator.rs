fn main() {

    let mut stack : Vec<String> = Vec::new();

    stack.push("hello".to_string());
    stack.push("world".to_string());

    let expected : String = stack.pop().expect("stack underflow");

    let actual : String = stack.pop().expect("stack underflow");

    if expected == actual {
        println!("Valid");
    } else {
        println!("not Valid");
    }
}