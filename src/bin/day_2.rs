use std::fs::read_to_string;

#[derive(Debug)]
struct Policy {
    letter: char,
    min: usize,
    max: usize,
}

impl Policy {
    fn from_str(policy_str: &str) -> Self {
        let mut policy_split = policy_str.split(" ");
        let range_str = policy_split.next().expect("Error getting range");
        let letter_str = policy_split.next().expect("Error getting letter");
        assert_eq!(letter_str.len(), 1);

        let mut range = range_str.split("-");
        let min = range
            .next()
            .expect("Error getting min")
            .parse()
            .expect("Error parsing min");
        let max = range
            .next()
            .expect("Error getting max")
            .parse()
            .expect("Error parsing max");

        let letter = letter_str.chars().next().unwrap();

        Policy { letter, min, max }
    }

    fn check(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count();
        count >= self.min && count <= self.max
    }

    fn check_new(&self, password: &str) -> bool {
        //if self.min > password.len() || self.max > password.len() {
        //return false
        //}

        let char_1 = password
            .chars()
            .nth(self.min - 1)
            .expect("Error getting `min` char");
        let char_2 = password
            .chars()
            .nth(self.max - 1)
            .expect("Error getting `max` char");

        match (char_1 == self.letter, char_2 == self.letter) {
            (true, false) => true,
            (false, true) => true,
            _ => false,
        }
    }
}

fn main() {
    let passwords = read_to_string("inputs/day_2").expect("Error reading password file");
    let passwords = passwords.trim_end().split("\n");

    let mut valid_passwords_old = 0;
    let mut valid_passwords_new = 0;

    for policy_and_password in passwords {
        let mut policy_and_password = policy_and_password.split(":");
        let policy_str = policy_and_password.next().expect("No policy");
        let password = policy_and_password.next().expect("No password").trim();

        let policy = Policy::from_str(policy_str);
        if policy.check(password) {
            valid_passwords_old += 1;
        }
        if policy.check_new(password) {
            valid_passwords_new += 1;
        }
    }

    println!("Valid passwords using old method: {}", valid_passwords_old);
    println!("Valid passwords using new method: {}", valid_passwords_new);
}
