// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
        let mut s1 = String::from("hello");
        let mut v = Vec::new();
    
        v.push(s1);  // s1's ownership is moved into the vector v
    
        // Concatenate to the string using dereferencing
        let s = &mut v[0];
        *s += ", world!";
        *s += " or someone else";
        // Print the modified string
        println!("{}", v[0]);  // Prints "hello, world!"
    


    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
