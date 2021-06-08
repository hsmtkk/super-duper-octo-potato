mod fizzbuzz;

fn main() {
    for i in 1..100 {
        let s = fizzbuzz::fizzbuzz(i);
        println!("{}", s);
    }
}
