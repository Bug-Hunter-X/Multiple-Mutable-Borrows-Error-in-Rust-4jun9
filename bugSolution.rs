fn main() {
    let mut x = 5;
    { //Scope 1
        let y = &mut x; 
        *y = 6; 
    }
    { //Scope 2
        let z = &mut x; 
        *z = 7;
    }
    println!("x = {}", x);
} 