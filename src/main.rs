use std::io::{self, Write};

/// A simplified Zero-Knowledge Proof demonstration.
/// Prover: Knows the secret 'x' such that g^x mod p = y.
/// Verifier: Verified that the Prover knows 'x' without learning 'x'.

fn main() {
    println!("--- Zero-Knowledge Proof (Schnorr-style) Demo ---");
    
    // Public parameters
    let g: u64 = 2; // Generator
    let p: u64 = 101; // Large Prime

    println!("Public Generator (g): {}", g);
    println!("Public Prime (p): {}", p);

    // 1. Prover chooses a secret 'x'
    let x: u64 = 42; 
    let y = power(g, x, p);
    println!("Prover's Public Key (y = g^x mod p): {}", y);

    // --- PROOF PROTOCOL START ---

    // 2. Prover chooses a random 'v' and sends 't = g^v mod p' to Verifier
    let v: u64 = 15;
    let t = power(g, v, p);
    println!("Prover sends Commitment (t = g^v mod p): {}", t);

    // 3. Verifier sends a challenge 'c'
    let c: u64 = 7;
    println!("Verifier sends Challenge (c): {}", c);

    // 4. Prover computes 'r = v - c*x' and sends it to Verifier
    // (In modular arithmetic, this is handled differently, but here's the concept)
    let r = (v + (p - 1) - (c * x % (p - 1))) % (p - 1);
    println!("Prover sends Response (r = v - c*x mod p-1): {}", r);

    // 5. Verifier checks if g^r * y^c mod p == t
    let check = (power(g, r, p) * power(y, c, p)) % p;
    println!("Verifier checks: (g^r * y^c mod p) = {}", check);

    if check == t {
        println!("✅ PROOF VERIFIED: Prover knows the secret!");
    } else {
        println!("❌ PROOF FAILED!");
    }
}

fn power(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 { res = (res * base) % modulus; }
        base = (base * base) % modulus;
        exp /= 2;
    }
    res
}