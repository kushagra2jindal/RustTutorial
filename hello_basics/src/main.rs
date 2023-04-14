
fn main() {

    // variables 
    // By default the variables are immutble, therefore before updating we need to mention mut keyword
    let mut a = 5; 
    println!("Vale of a is {a}");
    a = 6; 
    println!("Value of a changed to {a}");


    // constants
    const B : u32 = 3;
    println!("Value of b is {B}");


    // shawdowing
    let mut x = 5;
    x = x + 1;  // 6
    {
        let x = x * 2;  // 12
        println!("The value of x in the inner scope is: {x}");  // 12
    }
    println!("The value of x is: {x}"); // 6


    // Scalar Data Type --> Integer, Float, Boolean and Charater
    let abc : i32 = "42000".parse().expect("Not a number!");
    println!("The value of abc is: {abc}");

    let n_lit : u32 = 42_000;
    println!("The value of Number Literal is: {n_lit}");

    let f : f32 = 32.7432;
    println!("The value of Float Literal is: {f}");

    let check : bool = true;
    println!("The value of Boolean Literal is: {check}");

    let c : char = 'h';
    println!("The value of Character Literal is: {c}");


    // Compound Data Type --> Tuple and Array
    let tup : (u8, u8, u8) = (1, 2, 3);
    // let (x, y, z) = tup;    // destructuring
    // println!("The tuple 2nd element is {y}");
    let y = tup.1;
    println!("The tuple 2nd element is {y}");

    let a = [1,2,3,4,5];
    let y = a[1];
    println!("The array 2nd element is {y}");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let y = a[1];
    println!("The array 2nd element is {y}");

    // Calling a function
    println!("Electricty bill is {}", electricity_bill(1200));

    // Loops 
    let mut counter : u8 = 0;
    'test: loop { 
        counter = counter + 1;
        if counter <= 5 {
            println!("From the loop");
        } else if counter == 6 {
            continue;
        } else if counter == 7 {
            println!("From the loop {counter}");
        } else {
            break 'test;
        }
    }

    let a = [10, 20, 30, 40];
    let mut i = 0;
    while i < 4 {
        println!("{}", a[i]);
        i = i + 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn electricity_bill(uints : u32) -> f32 {
    let mut bill : f32;
    if uints < 500 {
        bill = 0.0;
    } else if uints >= 500 && uints < 1000 {
        // 2 bucks per unit above 500
        bill = ((uints-500)*2) as f32;
    } else {
        // 1.5 bucks per unit above 1000
        bill = 1000 as f32 + ((uints-1000) as f32)*1.5;
    }
    // Service charges!!
    bill = bill + 150.0;
    bill
}


// Random Testing!!
// fn main() {
//     let mut counter = 0;

//     let result : u32 = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }