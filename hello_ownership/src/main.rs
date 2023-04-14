/*
    Storage -> 
    In Rust, memory management is crucial as it is a systems programming language. 
    Understanding the difference between the stack and the heap is important. 
    The stack stores values in a last in, first out (LIFO) manner and is used for storing data with a known, fixed size. 
    On the other hand, the heap is used for data with an unknown size or a size that might change. 
    When data is allocated on the heap, the memory allocator finds an empty spot in the heap and marks it as in use, returning a pointer to the location. 
    Accessing data on the stack is faster than accessing data on the heap because a pointer must be followed to access heap data.
*/

/*
 *
 * Three important Rules for ownership:- 
 * Each value in Rust has an owner.
 * There can only be one owner at a time.
 * When the owner goes out of scope, the value will be dropped.
 *
 */


fn main() {

    let s1 = String::from("hello");
    let s2 = s1; 

    /*
        The below print will give an error because of the RUST ownership, to overcome the problem of Double Free Error
        As s1 is stored in heap and in stack only its pointer, length and capacity is stored
        s1 is moved to s2
        This is called shallow copy, Rust does not provide Deep copy in case of resources stored in heap.
    */
    // println!("{s1}");

    // Function Call can be tidious, in case value is required in future
    let (s3, len) = calulate_length(s2);

    // Can't be used as ownership is gone!!
    // println!("{s2}");
    println!("string is => {} of length => {}", s3, len);

    // Call by Reference of the function
    let len1 = calulate_length_reference(&s3);
    println!("string is => {} of length => {}", s3, len1);

    let mut s4 = String::from("Hello");
    change(&mut s4);    // More than one mutable reference can't be given to prevent Data Race!!
    let len2 = calulate_length_reference(&s4);
    println!("string is => {} with length => {}", s4, len2);

    // Dangling Reference --> compiler will not allow me to do this!! 
    // let reference_to_nothing = dangle();

    // slice refence
    let s6 = String::from("hello world");
    let word = first_word(&s6);
    println!("The word is => {}", word);
}

fn calulate_length(s : String) -> (String, usize) {
    let length : usize = s.len();
    (s, length)
}

fn calulate_length_reference(s : &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &string {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}