use 
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string().chars().collect::<Vec<char>>();
    let digits_count = digits.len();
    let mut sum = 0;
    for digit in digits {
        let digit_value = digit.to_digit(10).unwrap();
        let digit_value = digit_value as u32;
        let digit_value = digit_value.pow(digits_count as u32);
        sum += digit_value;
    }
    sum == num
}
