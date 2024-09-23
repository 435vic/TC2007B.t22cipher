// Taken from random.org, which reportedly uses atmospheric noise to generate random numbers
const SUBSTITUTION_TABLE: [u8; 256] = [
    0xa0,     0x26,     0xb3,     0x46,     0x0c,     0x5b,     0x64,     0xf3,
    0x76,     0xce,     0x60,     0x50,     0x91,     0xa6,     0x87,     0xf1,
    0x67,     0x17,     0x68,     0xe4,     0x8e,     0x96,     0x69,     0x95,
    0x7d,     0x6d,     0x34,     0xda,     0x94,     0x1c,     0x14,     0x88,
    0xd5,     0x2a,     0xe9,     0xdd,     0x8b,     0xe6,     0x05,     0xde,
    0x3f,     0x86,     0x15,     0x35,     0x65,     0x9e,     0xeb,     0xaf,
    0x2b,     0x56,     0xe1,     0x61,     0x3b,     0xca,     0x52,     0xcd,
    0xd1,     0x80,     0xa8,     0x2e,     0x1a,     0x83,     0xc4,     0x48,
    0x8a,     0xdb,     0x06,     0xaa,     0x25,     0x40,     0x7c,     0x2d,
    0x77,     0xfd,     0x47,     0xdf,     0xf0,     0x5d,     0x1d,     0x97,
    0x53,     0x20,     0xf4,     0x2f,     0xd2,     0xdc,     0x23,     0xb6,
    0xe8,     0x6a,     0xf8,     0x10,     0x39,     0x24,     0x4a,     0x73,
    0x09,     0xc7,     0xc0,     0xb8,     0x6f,     0x99,     0x3d,     0xab,
    0xbd,     0xa9,     0x9f,     0x30,     0x44,     0x1b,     0x3e,     0x5a,
    0x38,     0x9a,     0xf2,     0x4c,     0xe2,     0xa5,     0xb9,     0xfb,
    0x6b,     0xa7,     0xc2,     0xd7,     0x98,     0x92,     0x79,     0x32,
    0xee,     0x07,     0x03,     0x90,     0x7e,     0x49,     0x93,     0xd8,
    0x6e,     0x8d,     0xe7,     0x89,     0xf9,     0xcf,     0xea,     0x4f,
    0x5c,     0xd9,     0xba,     0xc6,     0xd4,     0xf7,     0x21,     0xb4,
    0xe5,     0xfc,     0xac,     0x1f,     0xc3,     0x57,     0xb0,     0xd3,
    0x42,     0x81,     0xe3,     0x82,     0xef,     0x85,     0xe0,     0xff,
    0xa3,     0xcc,     0x70,     0xbf,     0xd6,     0x4b,     0xed,     0x63,
    0x59,     0x54,     0xb2,     0xbc,     0xec,     0x19,     0x9c,     0x28,
    0x5f,     0xae,     0xd0,     0x37,     0x84,     0x16,     0x78,     0x31,
    0x4e,     0x2c,     0x36,     0x33,     0x5e,     0x58,     0xf6,     0x8c,
    0x29,     0x7f,     0xa4,     0x72,     0x12,     0xb5,     0xad,     0x27,
    0xbb,     0x0e,     0x02,     0x11,     0x0a,     0xc8,     0xfa,     0x9b,
    0xfe,     0xb7,     0x00,     0x74,     0xf5,     0xb1,     0xbe,     0x0d,
    0x45,     0x51,     0x0f,     0x08,     0x3c,     0x41,     0x04,     0x22,
    0xc9,     0xc5,     0x0b,     0x9d,     0x01,     0x55,     0x75,     0x1e,
    0x8f,     0x18,     0xa1,     0x71,     0x7b,     0x66,     0x4d,     0xa2,
    0x62,     0x6c,     0x7a,     0xc1,     0x3a,     0xcb,     0x13,     0x43,
];

pub fn pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padded = data.to_vec();
    let padding = block_size - (data.len() % block_size);
    padded.extend(std::iter::repeat(padding as u8).take(padding));
    padded
}

pub fn unpad(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let padding = *data.last().unwrap() as usize;
    let unpadded = data.len() - padding;
    data[..unpadded].to_vec()
}

pub fn transpose(plaintext: &[u8], key: &str) -> Result<Vec<u8>, String> {
    if !key.chars().all(|c| c.is_lowercase()) {
        return Err("Key must be all lowercase".to_string());
    }

    if key.len() != 16 {
        return Err("Key must be 16 characters long".to_string());
    }

    if plaintext.len() % 16 != 0 {
        return Err("Plaintext must be a multiple of 16 bytes".to_string());
    }

    let keylen = key.len();

    let rowcount = (plaintext.len() + keylen - 1) / key.len();
    let mut table: Vec<Vec<u8>> = vec![vec![0; keylen]; rowcount];

    for (i, b) in plaintext.iter().enumerate() {
        table[i / keylen][i % keylen] = *b;
    }

    let mut reordered: Vec<(usize, char)> = key.chars().enumerate().collect();
    reordered.sort_by_key(|&(_, c)| c);

    let mut ciphertext = Vec::new();
    for (i, _) in reordered {
        for row in table.iter() {
            ciphertext.push(row[i]);
        }
    }

    Ok(ciphertext)
}

pub fn untranspose(ciphertext: &[u8], key: &str) -> Result<Vec<u8>, String> {
    if !key.chars().all(|c| c.is_lowercase()) {
        return Err("Key must be all lowercase".to_string());
    }

    if key.len() != 16 {
        return Err("Key must be 16 characters long".to_string());
    }

    if ciphertext.len() % 16 != 0 {
        return Err("Ciphertext must be a multiple of 16 bytes".to_string());
    }

    let keylen = key.len();
    let rowcount = ciphertext.len() / keylen;

    let mut reordered: Vec<(usize, char)> = key.chars().enumerate().collect();
    reordered.sort_by_key(|&(_, c)| c);

    let mut table: Vec<Vec<u8>> = vec![vec![0; keylen]; rowcount];
    let mut index = 0;
    for (i, _) in reordered {
        for row in table.iter_mut() {
            row[i] = ciphertext[index];
            index += 1;
        }
    }

    let mut plaintext = Vec::new();
    for row in table {
        plaintext.extend(row);
    }

    Ok(plaintext)
}

fn substitute(plaintext: &[u8], key: &str) -> Result<Vec<u8>, String> {
    if !key.chars().all(|c| c.is_lowercase()) {
        return Err("Key must be all lowercase".to_string());
    }

    if key.len() != 16 || plaintext.len() != 16 {
        return Err("Key and plaintext must be 16 characters long".to_string());
    }

    let mut ciphertext = Vec::new();
    for (p, k) in plaintext.iter().zip(key.chars()) {
        let p = *p as usize;
        let k = k as usize;
        let c = SUBSTITUTION_TABLE[p ^ k];
        ciphertext.push(c);
    }

    Ok(ciphertext)
}

fn unsubstitute(ciphertext: &[u8], key: &str) -> Result<Vec<u8>, String> {
    // ... (same input validation as substitute function)

    let mut plaintext = Vec::new();
    for (c, k) in ciphertext.iter().zip(key.chars()) {
        let c = *c;
        let k = k as u8 as usize;
        let p = SUBSTITUTION_TABLE.iter().position(|&x| x == c).unwrap();
        plaintext.push((p ^ k) as u8);
    }

    Ok(plaintext)
}

pub fn shift(plaintext: &[u8], amt: usize) -> Vec<u8> {
    plaintext.iter()
        .cycle()
        .skip(amt)
        .take(plaintext.len())
        .cloned()
        .collect()
}

pub fn unshift(ciphertext: &[u8], amt: usize) -> Vec<u8> {
    let len = ciphertext.len();
    ciphertext.iter()
        .cycle()
        .skip(len - (amt % len))
        .take(len)
        .cloned()
        .collect()
}

fn encrypt(plaintext: &[u8], key: &str) -> Result<Vec<u8>, String> {
    if !key.chars().all(|c| c.is_lowercase()) {
        return Err("Key must be all lowercase".to_string());
    }

    // Split the padded text into 16-character chunks
    let chunks: Vec<Vec<u8>> = pad(plaintext, 16)
        .chunks(16)
        .map(|chunk| chunk.to_vec())
        .collect();

    let halfpoint = chunks.len() / 2;
    let mut flipped: Vec<Vec<u8>> = Vec::new();
    flipped.extend_from_slice(&chunks[halfpoint..]);
    flipped.extend_from_slice(&chunks[..halfpoint]);

    let transposed = transpose(&flipped.concat(), key)?;
    let mut ciphertext: Vec<u8> = Vec::new();

    for chunk in transposed.chunks(16) {
        let mut chunk = chunk.to_vec();
        for keychar in key.chars() {
            let substituted = substitute(&chunk, &keychar.to_string())?;
            let shift_by = (keychar as u8 - b'a') as usize;
            chunk = shift(&substituted, shift_by);
        }

        // final XOR
        chunk = chunk.into_iter()
            .zip(key.chars())
            .map(|(b, k)| b ^ (k as u8))
            .collect();

        ciphertext.extend_from_slice(&chunk);
    }

    Ok(ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test reversability of the transpose funcoin
    #[test]
    fn test_transpose() {
        use rand::Rng;
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let plaintext: Vec<u8> = (0..64).map(|_| rng.gen()).collect();
            let key: String = (0..16)
                .map(|_| rng.sample(rand::distributions::Uniform::new(b'a', b'z' + 1)) as char)
                .collect();

            let encrypted = transpose(&plaintext, &key).unwrap();
            let decrypted = untranspose(&encrypted, &key).unwrap();

            assert_eq!(plaintext, decrypted);
        }
    }

    #[test]
    fn test_shift() {
        assert_eq!(shift(b"abcdef", 0), b"abcdef");
        assert_eq!(shift(b"abcdef", 1), b"bcdefa");
        assert_eq!(shift(b"abcdef", 2), b"cdefab");
        assert_eq!(shift(b"abcdef", 6), b"abcdef");
        assert_eq!(shift(b"abcdef", 7), b"bcdefa");
    }

    #[test]
    fn test_unshift() {
        assert_eq!(unshift(b"abcdef", 0), b"abcdef");
        assert_eq!(unshift(b"bcdefa", 1), b"abcdef");
        assert_eq!(unshift(b"cdefab", 2), b"abcdef");
        assert_eq!(unshift(b"abcdef", 6), b"abcdef");
        assert_eq!(unshift(b"bcdefa", 7), b"abcdef");
    }

    #[test]
    fn test_shift_unshift_inverse() {
        let plaintext = b"Hello, World! This is a test.";
        for amt in 0..50 {
            let shifted = shift(plaintext, amt);
            let unshifted = unshift(&shifted, amt);
            assert_eq!(plaintext, &unshifted[..], "Failed at shift amount: {}", amt);
        }
    }

    #[test]
    fn test_shift_unshift_random() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let len = rng.gen_range(1..100);
            let plaintext: Vec<u8> = (0..len).map(|_| rng.gen()).collect();
            let amt = rng.gen_range(0..200);

            let shifted = shift(&plaintext, amt);
            let unshifted = unshift(&shifted, amt);

            assert_eq!(plaintext, unshifted, "Failed with len: {}, amt: {}", len, amt);
        }
    }
}
