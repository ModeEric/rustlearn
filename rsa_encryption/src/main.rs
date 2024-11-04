use std::vec::Vec;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

fn generate_primes_eratosthenes(n: usize, threshold: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![1; n];
    vec[0] = 0;
    vec[1] = 0;
    let maxqt: usize = f64::sqrt(n as f64) as usize + 1;
    for j in 2..maxqt {
        if vec[j] != 0 {
            let mut val = j * j;
            while val < n {
                vec[val] = 0;
                val += j;
            }
        }
    }
    let mut returnval: Vec<usize> = Vec::new();
    for (index, val) in vec.iter().enumerate() {
        if index > threshold && *val == 1 {
            returnval.push(index);
        }
    }
    returnval
}

fn efficient_exponentials_mod(m: u64, d: u64, n: u64) -> u64 {
    let mut base: u64 = m % n;
    let mut count = d;
    let mut result=1;
    while count !=0 {
        if count & 1 == 1 {
            result = (result*base) % n
        }
        base = (base*base) %n;
        count >>=1;
    }
    result
}

fn calculate_efficiency_metrics(p: u64, q: u64, n: u64, e: u64) {
    const STANDARD_KEY_SIZE: u64 = 2048;
    const STANDARD_PUBLIC_EXPONENT: u64 = 65537;
    const MIN_PRIME_SIZE: u64 = 1024;

    let n_bits = 64 - n.leading_zeros();
    let p_bits = 64 - p.leading_zeros();
    let q_bits = 64 - q.leading_zeros();

    let key_size_efficiency = (n_bits as f64 / STANDARD_KEY_SIZE as f64) * 100.0;
    let prime_balance = ((p_bits as i32 - q_bits as i32).abs()) as u64;
    let prime_size_efficiency = (p_bits.min(q_bits) as f64 / MIN_PRIME_SIZE as f64) * 100.0;
    let public_exp_security = if e > 3 { 100.0 } else { (e as f64 / STANDARD_PUBLIC_EXPONENT as f64) * 100.0 };

    let overall_security_score = 
        key_size_efficiency * 0.4 +
        (100.0 - prime_balance as f64) * 0.2 +
        prime_size_efficiency * 0.2 +
        public_exp_security * 0.2;

    println!("\nEfficiency Metrics:");
    println!("--------------------");
    println!("Key Size Efficiency: {:.2}%", key_size_efficiency);
    println!("Prime Balance (bits): {} (smaller is better)", prime_balance);
    println!("Prime Size Efficiency: {:.2}%", prime_size_efficiency);
    println!("Public Exponent Security: {:.4}%", public_exp_security);
    println!("Overall Security Score: {:.2}%", overall_security_score);
    
    println!("\nSecurity Assessment:");
    println!("-------------------");
    if overall_security_score < 5.0 {
        println!("⚠️  EXTREMELY WEAK - For educational purposes only!");
        println!("   - Key size is dangerously small");
        println!("   - Vulnerable to factoring attacks");
    } else if overall_security_score < 50.0 {
        println!("⚠️  WEAK - Not suitable for production use");
        println!("   - Consider increasing key size");
        println!("   - Use larger prime numbers");
    } else if overall_security_score < 90.0 {
        println!("📊 MODERATE - Improvements needed");
        println!("   - Consider standard RSA parameters");
    } else {
        println!("✅ STRONG - Meets standard security requirements");
    }
}

fn main() {
    // First implementation
    println!("First run:");
    let start_custom = Instant::now();
    
    // Modified parameters for 16-bit numbers
    let threshold: usize = 100;      // Reduced threshold
    let max: usize = 65535;          // Maximum 16-bit unsigned value
    
    let mut rng = thread_rng();
    let vec: Vec<usize> = generate_primes_eratosthenes(max, threshold);
    let primes: Vec<usize> = vec.choose_multiple(&mut rng, 2).cloned().collect();
    let p: u64 = primes[0] as u64;
    let q: u64 = primes[1] as u64;
    let n: u64 = p * q;
    let t: u64 = (p - 1) * (q - 1);
    let e: u64 = 17;  // Smaller public exponent for faster computation
    let mut d = 0;
    let mut curr = 0;
    for i in 1..t {
        curr = (curr + e) % t;
        if curr == 1 {
            d = i;
            break;
        }
    }
    println!("Custom RSA Parameters:");
    println!("p={}, q={}", p, q);
    println!("n={}", n);
    println!("Size of n in bits: {}", 64 - n.leading_zeros());
    
    let message = "Secret message";
    let m: u64 = message.bytes().next().unwrap() as u64;
    println!("Original message (first byte): {}", m);
    println!("Public key: ({}, {})", n, e);

    let m_send: u64 = efficient_exponentials_mod(m, e, n);
    println!("Sending encrypted message: {}", m_send);

    let m_d: u64 = efficient_exponentials_mod(m_send, d, n);
    println!("Decoded message: {}", m_d);

    calculate_efficiency_metrics(p, q, n, e);
    
    let duration_custom = start_custom.elapsed();
    println!("\nPerformance Metrics:");
    println!("------------------");
    println!("Implementation took: {:?}", duration_custom);

    // Second run with different random primes
    println!("\n-----------------------------------\n");
    println!("Second run:");
    let start_custom2 = Instant::now();
    
    let primes2: Vec<usize> = vec.choose_multiple(&mut rng, 2).cloned().collect();
    let p2: u64 = primes2[0] as u64;
    let q2: u64 = primes2[1] as u64;
    let n2: u64 = p2 * q2;
    let t2: u64 = (p2 - 1) * (q2 - 1);
    let mut d2 = 0;
    let mut curr2 = 0;
    for i in 1..t2 {
        curr2 = (curr2 + e) % t2;
        if curr2 == 1 {
            d2 = i;
            break;
        }
    }
    
    println!("Custom RSA Parameters:");
    println!("p={}, q={}", p2, q2);
    println!("n={}", n2);
    println!("Size of n in bits: {}", 64 - n2.leading_zeros());
    
    let m2: u64 = message.bytes().next().unwrap() as u64;
    println!("Original message (first byte): {}", m2);
    println!("Public key: ({}, {})", n2, e);

    let m_send2: u64 = efficient_exponentials_mod(m2, e, n2);
    println!("Sending encrypted message: {}", m_send2);

    let m_d2: u64 = efficient_exponentials_mod(m_send2, d2, n2);
    println!("Decoded message: {}", m_d2);

    calculate_efficiency_metrics(p2, q2, n2, e);
    
    let duration_custom2 = start_custom2.elapsed();
    println!("\nPerformance Metrics:");
    println!("------------------");
    println!("Implementation took: {:?}", duration_custom2);
}