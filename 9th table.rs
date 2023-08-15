fn main() {
    println!("The Table of 9");
    table9();
}

fn table9(){
    for n in 1..=10 {
        println!("9*{}={}",n,9*n);
    }
}
