use std::vec::Vec;
use rand::seq::SliceRandom; // For the choose_multiple method
use rand::thread_rng;
fn generate_primes_eratosthenes(n: usize, threshold: usize) -> Vec<usize>{ //O(nloglog(n))
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
    let mut returnval: Vec<usize> = Vec::new();
    for (index, val) in vec.iter().enumerate(){
        if index > threshold && *val==1 {
            returnval.push(index);
        }
    }
    returnval
}

fn efficient_exponentials_mod(m: u32, d: u32,n: u32) -> u32 { //O(d)
    let mut result: u32 = m;
    let mut count = d;
    while count!=1 {
        result = result * m % n;
        count-=1;
    }
    result
}

fn rsa() {
    let threshold: usize = 20;
    let max:usize = 30;
    let mut rng = thread_rng();
    let vec: Vec<usize> = generate_primes_eratosthenes(max,threshold);
    let primes: Vec<usize> = vec.choose_multiple(&mut rng,2).cloned().collect();
    let p: u32 = primes[0] as u32;
    let q: u32 = primes[1] as u32; 
    let n: u32 = p*q;
    let t : u32= (p-1)*(q-1);
    let e: u32 = 3;
    let mut d = 0;
    for i in 1..n{
        if (i*e) % t == 1 {
            d = i;
            break;
        }
    }
    println!("n={}",n.to_string());
    let m: u32 = 89; //Example
    println!(" public key: {}{}",n.to_string(),e.to_string());
    //Now, need to compute 89**e and send it. Computationally easy here.

    //Person sends back message based on public key
    let m_send: u32 = efficient_exponentials_mod(m,e,n);
    println!("Sending message back {}", m_send);
    //Now person receiving the message can decrypt it using d
    let m_d: u32 = efficient_exponentials_mod(m_send,d,n);

    println!("Decoded message: {}",m_d);




}

fn main(){
    rsa();
    //Following written by chatgpt to compare efficiency

}
