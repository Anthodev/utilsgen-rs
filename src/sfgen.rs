use rand::Rng;

pub fn gen_symfony_secret() -> String {
    let mut secret = String::new();
    let mut rng = rand::thread_rng();
    let charset: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~".chars().collect();
    let charset_len = charset.len();

    for _ in 0..32 {
        let idx = rng.gen_range(0..charset_len);
        secret.push(charset[idx]);
    }

    return secret;
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_gen_symfony_secret() {
        let secret = gen_symfony_secret();
        assert_eq!(secret.len(), 32);
        assert!(Regex::new(r"^[A-Za-z0-9-._~]{32}$").unwrap().is_match(secret.as_str()));
    }
}