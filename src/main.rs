// Step 1: Function that takes two string slices (&str) and returns a new String
fn concatenate_strings(s1: &str, s2: &str) -> String {
    // Step 2: Create a new String and use push_str to append contents
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);

    // Step 3: Return the result
    result
}

fn main() {
    // Step 4: Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rustaceans!");

    // Step 5: Call the function using references (slices) 
    // This allows string1 and string2 to remain valid after the call
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Step 6: Print the result
    println!("Result: {}", concatenated_string);
}