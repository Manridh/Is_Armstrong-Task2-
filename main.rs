pub fn armstrong_no(num: u32) -> bool {
    let length = ((num + 1) as f32).log10().ceil() as u32;
    let str = num.to_string();
    let ans_armstrong: u32 = str.chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .map(|digit| digit.pow(length))
        .sum();
    ans_armstrong == num
}