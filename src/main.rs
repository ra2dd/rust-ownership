fn main() {
    {
                            // s is not valid here, it's not yet declared
        let s = "hello";    // s is valid from this point forward

        // do stuff with s
    }                       // this scope is over, and s is no longer valid

    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    {
        let s1 = String::from("hello");
        let s2 = s1;
        // s1 was moved into s2
        // s1 is no longer valid

        let s3 = s2.clone();
        println!("s2 = {s2}, s3 = {s3}");
    }

    {
        let x = 5;
        let y = x;

        // x is stored on a stack (copies are quick to make) 
        // it has a Copy trait
        // so it's still valid after being assigned to y
        println!("x = {x}, y = {y}");
    }

    {
        let s = String::from("hello");      // s comes into scope              
        takes_ownership(s);                 // s's value moves into function scope
                                            // and it's not longer valid here

        let x = 10;                         // x comes into scope
        makes_copy(x);                      // x would move into a function
                                            // but i32 has a Copy trait
                                            // so it's ok to use x afterwards
    } // x goes out of scope than s, but because s's value was moved nothing special happens

    fn takes_ownership(some_string: String) { // some_string comes into scope
    } // some_string goes out of scope, drop is called, memory is freed
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
    } // some_integer goes out of scope
}
