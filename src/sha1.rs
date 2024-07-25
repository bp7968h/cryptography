pub fn hash(data: &str) -> String {
    //data in a bit string
    let binary_str = bit_string(data.as_bytes());

    binary_str

}

fn bit_string(arr_bytes: &[u8]) -> String {
    let mut binary_str = String::new();
    for byte in arr_bytes.iter(){
        binary_str.push_str(&format!("{:08b}", byte));
    }
    binary_str
}

fn pad(bit_str: &mut String) {
    //First append 1 to the original bit string
    bit_str.push('1');
    
    let append_length = bit_str.len();
    let zeros_to_append = "0".repeat(448 - bit_str.len());
    
    //Now pad remaining with zeros
    bit_str.push_str(&zeros_to_append);
}

fn get_2word(original_length: usize) -> (String, String){
    // A word is a 32 bit string, or 4byte string
    let first_word = String::with_capacity(4);
    let second_word = String::with_capacity(4);
    
    match original_length {
        length if length <= 2usize.pow(32) => {
            ("less".to_string(), "less".to_string())
        },
        _ => ("more".to_string(), "more".to_string())
    }
}