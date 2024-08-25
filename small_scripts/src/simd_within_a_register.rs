fn eight_digit_parse(slice: &str) -> usize {
    let mut acc : usize = 0;    

    for ch in slice.bytes() {
        acc *= 10;
        println!(" ch -> {} ", ch);
        acc += (ch - b'0') as usize
    }
    
    acc
} 

fn eight_digit_parse_simd(slice: &str) -> usize {
    let as64bit_word = u64::from_ne_bytes(slice.as_bytes().try_into().unwrap());
    println!("64 bit : {} ", as64bit_word);
    let digits = as64bit_word - 0x3030303030303030;
    digits.try_into().unwrap()
}

pub fn test() {
    let digits = "12345678";
    
    let parsed1 = eight_digit_parse(&digits);
    let parsed2 = eight_digit_parse_simd(&digits);

    println!("Parsed 1: {} \nParsed 2: {}", parsed1, parsed2);
} 
