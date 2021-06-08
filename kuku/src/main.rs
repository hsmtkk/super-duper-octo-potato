fn kuku(){
    for i in 1..10 {
        for j in 1..10 {
            print!("{:2} ", i * j);
        }
        println!();
    }
}

fn main() {
    kuku();
}
