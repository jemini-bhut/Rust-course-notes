#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    //{
        let area = area_of(width, height);
    //}

    println!("Area is {}", area);

    println!("Volume is {}", volume(width, height, depth));
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x*y*z
}

fn area_of(x: i32, y: i32) -> i32 {
    //return x*y;
    x*y // The last line/ tail expression returns the value no need to specify 'return' keyword. 
    //            Hint: `cargo clippy` will warn you about this exact thing.
}
