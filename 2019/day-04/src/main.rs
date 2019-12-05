const LOW: u32 = 156218;
const HIGH: u32 = 652527;

fn is_password(mut pwd: u32) -> bool {
    let mut prev = 10;
    let mut dup = false;

    while pwd > 0 {
        let digit = pwd % 10;
        if digit > prev {
            return false;
        }

        if digit == prev {
            dup = true;
        }

        prev = digit;
        pwd /= 10;
    }
    dup
}

fn is_password_2(mut pwd: u32) -> bool {
    if !is_password(pwd) {
        return false;
    }
    let mut prev = 10;
    let mut count = 0;
    while (pwd > 0) {
        let digit = pwd % 10;
        if digit == prev {
            if count == 0 {
                count = 2;
            } else {
                count += 1;
            }
        } else {
            if count == 2 {
                return true;
            }
            count = 0;
        }
        prev = digit;
        pwd /= 10;
    }
    count == 2
}

fn main() {
    let count = (LOW..=HIGH).filter(|&p| is_password(p)).count();
    println!("part 1: {}", count);
    let count = (LOW..=HIGH).filter(|&p| is_password_2(p)).count();
    println!("part 2: {}", count);
}
