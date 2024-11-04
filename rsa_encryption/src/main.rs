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

fn efficient_exponentials_mod(m: u32, d: u32, n: u32) -> u32 {
    let mut result: u32 = m % n;
    let mut count = d;
    while count > 1 {
        result = ((result as u64 * m as u64) % n as u64) as u32;
        count -= 1;
    }
    result
}

fn calculate_efficiency_metrics(p: u32, q: u32, n: u32, e: u32) {
    const STANDARD_KEY_SIZE: u32 = 2048;
    const STANDARD_PUBLIC_EXPONENT: u32 = 65537;
    const MIN_PRIME_SIZE: u32 = 1024;

    // Calculate bit lengths
    let n_bits = 32 - n.leading_zeros();
    let p_bits = 32 - p.leading_zeros();
    let q_bits = 32 - q.leading_zeros();

    // Calculate efficiency metrics
    let key_size_efficiency = (n_bits as f64 / STANDARD_KEY_SIZE as f64) * 100.0;
    let prime_balance = ((p_bits as i32 - q_bits as i32).abs()) as u32;
    let prime_size_efficiency = (p_bits.min(q_bits) as f64 / MIN_PRIME_SIZE as f64) * 100.0;
    let public_exp_security = if e > 3 { 100.0 } else { (e as f64 / STANDARD_PUBLIC_EXPONENT as f64) * 100.0 };

    // Calculate overall security score
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
    
    // Print security assessment
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

fn rsa() {
    let threshold: usize = 40000;
    let max: usize = 65000;
    
    let mut rng = thread_rng();
    let vec: Vec<usize> = generate_primes_eratosthenes(max, threshold);
    let primes: Vec<usize> = vec.choose_multiple(&mut rng, 2).cloned().collect();
    let p: u32 = primes[0] as u32;
    let q: u32 = primes[1] as u32;
    let n: u32 = p * q;
    let t: u32 = (p - 1) * (q - 1);
    let e: u32 = 3;
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
    println!("Size of n in bits: {}", 32 - n.leading_zeros());
    
    let message = "Secret message";
    let m: u32 = message.bytes().next().unwrap() as u32;
    println!("Original message (first byte): {}", m);
    println!("Public key: ({}, {})", n, e);

    let m_send: u32 = efficient_exponentials_mod(m, e, n);
    println!("Sending encrypted message: {}", m_send);

    let m_d: u32 = efficient_exponentials_mod(m_send, d, n);
    println!("Decoded message: {}", m_d);

    // Calculate and display efficiency metrics
    calculate_efficiency_metrics(p, q, n, e);
}

fn main() {
    let start_custom = Instant::now();
    rsa();
    let duration_custom = start_custom.elapsed();
    println!("\nPerformance Metrics:");
    println!("------------------");
    println!("Custom RSA implementation took: {:?}", duration_custom);

    println!("\n-----------------------------------\n");

    use rsa::{RsaPrivateKey, RsaPublicKey};
    use rsa::pkcs1v15::Pkcs1v15Encrypt;
    use rand::rngs::OsRng;

    let mut rng = OsRng;

    let start_standard_keygen = Instant::now();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    let duration_standard_keygen = start_standard_keygen.elapsed();

    let data = b"Secret message";
    let start_standard_encryption = Instant::now();
    let enc_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("Failed to encrypt");
    let duration_standard_encryption = start_standard_encryption.elapsed();

    let start_standard_decryption = Instant::now();
    let dec_data = private_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("Failed to decrypt");
    let duration_standard_decryption = start_standard_decryption.elapsed();

    println!("Standard RSA implementation:");
    println!("Key generation took: {:?}", duration_standard_keygen);
    println!("Encryption took: {:?}", duration_standard_encryption);
    println!("Decryption took: {:?}", duration_standard_decryption);
    println!("Decrypted message: {}", String::from_utf8(dec_data).unwrap());
}