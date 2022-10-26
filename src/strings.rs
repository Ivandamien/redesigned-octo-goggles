pub fn run(){

 let mut hello = String::from("Hello ");

//  Get length;
println!("Length: {}", hello.len());

//  Push char
hello.push('w');


//  Push String
hello.push_str("orld"); 

// capacity in bytes
println!("Capacity: {}", hello.capacity());

// Check if empty
println!("Is Empty: {}", hello.is_empty());

// Contains
println!("Contains 'world' {}", hello.contains("world"));

println!("{}",hello);
}