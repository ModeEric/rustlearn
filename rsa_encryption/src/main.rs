use std::vec::Vec;
fn generate_primes_eratosthenes(n: usize) {
    let mut vec : Vec<usize> = vec![1;n];
    vec[0] = 0;
    vec[1] = 0;
    let maxqt: usize = f64::sqrt(n as f64) as usize + 1;
    for j in 2..maxqt {
        if *vec.get(j as usize).unwrap()!=0{
            let mut val = j*j;
            while val < n {
                vec[val as usize] = 0;
                val+=j;
            }
        }

    }
    for (index, val) in vec.iter().enumerate(){
        if *val==1 {
            println!("{} is prime!",index);
        }
    }
}
fn main() {
    generate_primes_eratosthenes(30);
}
