pub struct SHA1{
    h0: u32,
    h1: u32,
    h2: u32,
    h3: u32,
    h4: u32,
    w: [u32; 80],
}

impl SHA1{
    pub fn new() -> Self {
        SHA1{}
    }

    pub fn hash(&mut self, data: &str) -> String {
        // pad the original message until it is 512 bit length or 64 byte
        let padded_msg = self.pad_msg(data);

        for chunk in padded_msg.chunks(64){
            // Fill the first 16 word from the original message it self.
            for (index, word) in chunk.chunks(4).enumerate(){
                //convert the byte array to u32 integer and add it to the word list
                self.w[index] = u32::from_be_bytes(word.try_into().unwrap());
            }

            // Compute W1....W15 from the padded message
            self.compute_16_word();
        }
    }

    fn compute_16_word(&mut self) {
        //For w's index 16 to 79
        //W(t) = S^1(W(t-3) XOR W(t-8) XOR W(t-14) XOR W(t-16))
        for index in 16..=79 {
            //1. (W(t-3) XOR W(t-8) XOR W(t-14) XOR W(t-16))
            let argument = self.w[index - 3] ^ self.w[index - 8] ^ self.w[index - 14] ^ self.w[index - 16];
            //2. S^n(X)  =  (X << n) OR (X >> 32-n)
            // S^1(argument)
            self.w[index] = (argument << 1) | (argument >> 32-1);
        }
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