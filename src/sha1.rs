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
        SHA1{
            h0: 0x67452301,
            h1: 0xEFCDAB89,
            h2: 0x98BADCFE,
            h3: 0x10325476,
            h4: 0xC3D2E1F0,
            w: [0;80],
        }
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

            // For t = 0 to 79 do
            // TEMP = S^5(A) + f(t;B,C,D) + E + W(t) + K(t);
            // E = D;  D = C;  C = S^30(B);  B = A; A = TEMP;
            let (mut a, mut b, mut c, mut d, mut e) = (self.h0, self.h1, self.h2, self.h3, self.h4);

            for index in 0..=79 {
                let (sequence_logical_value, sequence_constant) = self.get_sequence(index, b, c, d);

                //TEMP = S^5(A) + f(t;B,C,D) + E + W(t) + K(t);
                let temp: u32 =  ((a << 5) | (a >> 32-5)).wrapping_add(sequence_logical_value)
                                    .wrapping_add(e)
                                    .wrapping_add(self.w[index])
                                    .wrapping_add(sequence_constant);

                // E = D;  D = C;  C = S^30(B);  B = A; A = TEMP;
                e = d; d = c; c = (b << 30) | (b >> 32 - 30); b = a; a = temp;
            }

            // Let H0 = H0 + A, H1 = H1 + B, H2 = H2 + C, H3 = H3 + D, H4 = H4 + E
            self.h0 = self.h0.wrapping_add(a);
            self.h1 = self.h1.wrapping_add(b);
            self.h2 = self.h2.wrapping_add(c);
            self.h3 = self.h3.wrapping_add(d);
            self.h4 = self.h4.wrapping_add(e);
        }

        let hashed_str = self.get_hashed_str();

        hashed_str
    }

    fn get_hashed_str(&self) -> String {
        let hash_arr = [self.h0.to_be_bytes(), self.h1.to_be_bytes(), self.h2.to_be_bytes(), self.h3.to_be_bytes(), self.h4.to_be_bytes()];

        let flat_bytes: Vec<u8> = hash_arr.iter().flatten().copied().collect();

        // let hashed_str = String::from_utf8(flat_bytes).unwrap();
        flat_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()

    }

    fn get_sequence(&self, index: usize, b: u32, c: u32, d: u32) -> (u32, u32) {
        match index {
            0..=19 => {
                // (B AND C) OR ((NOT B) AND D)
                ((b & c) | ((!b) & d), 0x5A827999)
            },
            20..=39 => {
                // B XOR C XOR D
                ((b ^ c ^ d),0x6ED9EBA1)
            },
            40..=59 => {
                // (B AND C) OR (B AND D) OR (C AND D)
                ((b & c) | (b & d) | (c & d), 0x8F1BBCDC)
            },
            _ => {
                // B XOR C XOR D
                ((b ^ c ^ d), 0xCA62C1D6)
            },
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

        //Add single one along with zeros
        bytes.push(0b10000000);

        while (bytes.len() * 8) % 512 != 448 {
            //0 is same as 00000000
            bytes.push(0);
        }

        bytes.extend_from_slice(&(original_bit_len).to_be_bytes());

        bytes
    }
}