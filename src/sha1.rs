pub fn hash(data: &str) -> String {
    //data in a bit string
    let binary_str = bit_string(data.as_bytes());

    binary_str

}

fn bit_string(arr_bytes: &[u8]) -> String {
    let mut binary_str = String::new();
    for byte in arr_bytes.iter(){
        binary_str.push_str(&format!("{:08b} ", byte));
    }
    binary_str
}