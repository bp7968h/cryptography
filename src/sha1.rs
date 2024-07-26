pub struct SHA1;

impl SHA1{
    pub fn new() -> Self {
        SHA1{}
    }

    pub fn hash(&mut self, data: &str) -> String {
        // pad the original message until it is 512 bit length or 64 byte
        let padded_msg = self.pad_msg(data);
    }

    fn pad_msg(&self, original_msg: &str) -> Vec<u8> {
        let mut bytes: Vec<u8> = original_msg.as_bytes().to_vec();
        let original_bit_len = bytes.len() as usize * 8;

        bytes.push(0b10000000);

        while (bytes.len() * 8) % 512 != 448 {
            bytes.push(0);
        }

        bytes.extend_from_slice(&(original_bit_len).to_be_bytes());

        bytes
    }
}