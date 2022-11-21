// rsa algorithm simplified
enum ErrModMulInv {
    InvNotExist,
}

pub(crate) fn run_demo() {
    let p: i128 = 61;
    let q: i128 = 53;
    let msg = 65;
    if let Ok((n, e, d)) = rsa(p, q) {
        let cipher = encrypt_rsa(msg, e, n);
        let recovered = decrypt_rsa(cipher, d, n);

        assert_eq!(recovered, msg);

        println!("the msg is: {}", msg);
        println!(
            "the encryption keys are: {:?}, and the decryption keys are {:?}",
            (n, e),
            (n, d)
        );
        println!("the cipher is: {}", cipher);
        println!("the recovered cipher is: {}", recovered);
    }
}

fn rsa(p: i128, q: i128) -> Result<(i128, i128, i128), ErrModMulInv> {
    let n = p * q;
    let totient_n = totient(p, q);
    let e = select_coprime(n);
    let d = mod_mul_inv(e, totient_n)?;
    Ok((n, e, d))
}

fn totient(p: i128, q: i128) -> i128 {
    let g = gcd(p - 1, q - 1);

    (p - 1) * (q - 1) / g
}

#[allow(unused_variables)]
fn select_coprime(n: i128) -> i128 {
    // to simplify the selection
    17
}

fn mod_mul_inv(a: i128, m: i128) -> Result<i128, ErrModMulInv> {
    let (g, x, _) = gcd_extended(a, m);

    if g != 1 {
        Err(ErrModMulInv::InvNotExist)
    } else {
        Ok((x % m + m) % m)
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (g, x, y) = gcd_extended(b, a % b);

    (g, y, x - a / b * y)
}

fn encrypt_rsa(m: i128, e: i128, n: i128) -> i128 {
    mod_pow(m, e, n)
}

fn decrypt_rsa(c: i128, d: i128, n: i128) -> i128 {
    mod_pow(c, d, n)
}

fn mod_pow(mut b: i128, mut e: i128, m: i128) -> i128 {
    if m == 1 {
        return 0;
    }
    let mut r = 1;
    b = b % m;
    while e > 0 {
        if e % 2 == 1 {
            r = r * b % m;
        }
        e = e >> 1;
        b = b * b % m
    }
    r
}
