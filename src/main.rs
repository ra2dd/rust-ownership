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

    references();
    slices();
}

fn references() {
    {
        let s1 = String::from("hello");
        // s1 reference is passed to function
        // references are immutable by default
        let len = calculate_length(&s1);
        // s1 is still valid here
        println!("\nThe length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize { // s is reference to a String
        s.len() 
    } // s goes out of scope, the value that reference was pointing at is not dropped yet

    {
        let mut s2 = String::from("Hello");
        // passing mutable reference to a function
        // can only have one mutable reference at a time
        let ref_s2 = &mut s2;
        change(ref_s2);

        println!("{s2}");
    }

    {
        let mut s3 = String::from("Hello");

        // can have multiple immutable references
        // must have multiple immutable or one mutable reference
        // once mutable reference is used, use of immutable references will fail to compile
        let ref_1 = &s3;
        let ref_2 = &s3;
        println!("{ref_1}");
        println!("{ref_2}");

        let ref_3 = &mut s3;
        change(ref_3);
        println!("{ref_3}");
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world!");
    }

    // reference is only valid when the value it's referencing
    // is in scope
}

fn slices() {
    let mut s = String::from("hello world");
    // creating a reference to certain part of the String
    // first value is an starting index 
    // second value - first value is a length
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("\n{} {}", hello, world);

    // when creating slices one can omit indexes
    // when its first or last character in a String 
    let s2 = String::from("new string");
    let new = &s2[..3];
    let string = &s2[4..];
    println!("{} {}", new, string);

    // [..] references entire String
    let new_string = &s2[..];
    println!("{}", new_string);

    // function for returning reference to a word
    // in a String until a first space
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let word = first_word(&s);
    println!("The fist word using a function is:{}", word);

    s = String::from("string changed");
    // once value the slice is referencing changes
    // the slice cannot longer be used
    // because the program will fail to compile

    let new_fw = first_word(&s[0..8]);
    println!("The first word passing slice of the string as a parameter:{}", new_fw);

    let string_literal = "immutable string";
    let word = first_word(&string_literal[0..13]);
    println!("The first word from string literal is:{}", word);

    let word = first_word(string_literal);
    println!(
        "Passing string literal to a function accepting 
        a reference to a string still works, 
        because string literals are string 
        slices already."
    );

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // using pretty print :? to display slice
    println!("Number array slice:{:?}", slice);
}