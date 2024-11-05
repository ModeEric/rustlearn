use rand::Rng;
use num::integer::lcm;
use std::time::Instant;

fn efficient_exponentials_mod(m: u128, d: u128, n: u128) -> u128 {
    let mut base: u128 = m % n;
    let mut count = d;
    let mut result = 1;
    while count != 0 {
        if count & 1 == 1 {
            result = (result * base) % n
        }
        base = (base * base) % n;
        count >>= 1;
    }
    result
}

fn is_prime(n: usize, rounds: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d: u128 = (n as u128) - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();
    let newn: u128 = n as u128;
    'outer: for _ in 0..rounds {
        let a = rng.gen_range(2..(n - 1)) as u128;
        let mut x = efficient_exponentials_mod(a, d, newn);

        if x == 1 || x == newn - 1 {
            continue;
        }

        for _ in 0..(s - 1) {
            x = efficient_exponentials_mod(x, 2, newn);
            if x == newn - 1 {
                continue 'outer;
            }
        }

        return false;
    }

    true
}

fn generate_prime_rabin(n: usize, threshold: usize, rounds: usize) -> Option<usize> {
    let mut rng = rand::thread_rng();

    loop {
        let num = rng.gen_range(threshold..=n);
        let candidate = if num % 2 == 0 { num + 1 } else { num };

        if is_prime(candidate, rounds) {
            return Some(candidate);
        }
    }
}

fn modular_inverse(a: u128, n: u128) -> Option<u128> {
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (n as i64, a as i64);

    while new_r != 0 {
        let quotient = r / new_r;
        let temp_t = t - quotient * new_t;
        t = new_t;
        new_t = temp_t;

        let temp_r = r - quotient * new_r;
        r = new_r;
        new_r = temp_r;
    }

    if r > 1 {
        return None;
    }
    if t < 0 {
        t += n as i64;
    }

    Some(t as u128)
}

fn main() {
    // Start total timer
    let total_start = Instant::now();

    let threshold: usize = 2_usize.pow(15);
    let max: usize = 2_usize.pow(16) - 1;

    // Measure prime generation for p
    let start_p = Instant::now();
    let p: u128 = generate_prime_rabin(max, threshold, 20).unwrap() as u128;
    let duration_p = start_p.elapsed();
    println!("Prime p generated: {} in {:?}", p, duration_p);

    // Measure prime generation for q
    let start_q = Instant::now();
    let mut q: u128 = generate_prime_rabin(max, threshold, 20).unwrap() as u128;
    while p == q {
        q = generate_prime_rabin(max, threshold, 20).unwrap() as u128;
    }
    let duration_q = start_q.elapsed();
    println!("Prime q generated: {} in {:?}", q, duration_q);

    // Measure key setup
    let start_key = Instant::now();
    let n = p * q;
    let lambda = lcm(p - 1, q - 1);
    let g = n + 1; // for ease
    let mu = modular_inverse(
        (efficient_exponentials_mod(g, lambda, n.pow(2)) - 1) / n,
        n,
    )
    .unwrap()
        % n;
    let duration_key = start_key.elapsed();
    println!(
        "Key setup completed in {:?}. Public key: (n: {}, g: {}), Private key: (lambda: {}, mu: {})",
        duration_key, n, g, lambda, mu
    );

    // Measure encryption
    let start_enc = Instant::now();
    let m = 89;
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(2..=1000);
    let c = efficient_exponentials_mod(g, m, n.pow(2)) * efficient_exponentials_mod(r, n, n.pow(2));
    let duration_enc = start_enc.elapsed();
    println!("Encryption completed in {:?}. Ciphertext: {}", duration_enc, c);

    // Measure decryption
    let start_dec = Instant::now();
    let tmp = (efficient_exponentials_mod(c, lambda, n.pow(2)) - 1) / n % n;
    let m_check = ((tmp as u128 * mu as u128) % n) as u128;
    let duration_dec = start_dec.elapsed();
    println!(
        "Decryption completed in {:?}. Decrypted message: {}",
        duration_dec, m_check
    );

    // Verify decryption
    assert_eq!(m, m_check, "Decryption failed!");

    // Total duration
    let total_duration = total_start.elapsed();
    println!("Total execution time: {:?}", total_duration);
}
