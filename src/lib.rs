const ALPHABETS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn encode(num: usize) -> String {
    let base: usize = ALPHABETS.to_string().len();

    if num == 0 {
        return ALPHABETS.chars().next().unwrap().to_string();
    }

    let mut div = num;
    let mut short_url = String::new();

    while div > 0 {
        let rem = div % base;
        short_url.push(ALPHABETS.chars().nth(rem).unwrap());
        div = div / base;
    }

    short_url.chars().rev().collect()
}

pub fn decode(str: String) -> usize {
    let base: usize = ALPHABETS.to_string().len();

    let mut i: usize = 0;

    for c in str.chars() {
        i = (i * base) + ALPHABETS.find(c).unwrap();
    }

    i
}
