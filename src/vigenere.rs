// vigenere algo

pub(crate) fn run_demo() {
    let msg = "Pelcgb Qnvfhxv";
    let ans = "Crypto Daisuki";
    let results = vigenere(msg);

    let mut ans_checked = false;
    for (idx, res) in results.iter().enumerate() {
        println!("Result of shift {} char: {}", idx + 1, res);

        if res == ans {
            ans_checked = true;
            println!("Answer found at shift {}", idx + 1);
        }
    }
    assert!(ans_checked);
}

fn vigenere(msg: &str) -> Vec<String> {
    (1..26)
        .map(|i| i as u32)
        .map(|i| msg.chars().map(|c| shift_caesar(c, i)).collect())
        .collect()
}

fn shift_caesar(c: char, offset: u32) -> char {
    let cap_a_digit = to_ascii('A');
    let cap_z_digit = to_ascii('Z');
    let a_digit = to_ascii('a');
    let z_digit = to_ascii('z');

    let c_digit = to_ascii(c);

    // CAPITAL LETTER
    if c_digit >= cap_a_digit && c_digit <= cap_z_digit {
        return char::from_u32(compute_new_digit(c_digit, offset, cap_a_digit, cap_z_digit))
            .unwrap();
    } else if c_digit >= a_digit && c_digit <= z_digit {
        return char::from_u32(compute_new_digit(c_digit, offset, a_digit, z_digit)).unwrap();
    } else {
        return c;
    }
}

fn to_ascii(x: char) -> u32 {
    // let hex_str = format!("{:x}", x as u32);
    // u32::from_str_radix(hex_str.as_str(), 16).unwrap()
    x as u32
}

fn compute_new_digit(c_digit: u32, offset: u32, lower_bound: u32, upper_bound: u32) -> u32 {
    lower_bound + (c_digit - lower_bound + offset) % (upper_bound - lower_bound + 1)
}
