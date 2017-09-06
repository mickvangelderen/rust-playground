pub fn byte_count(input: &[u8]) -> [u32; 0x100] {
    let mut output = [0u32; 0x100];
    for &byte in input.iter() {
        // A bound check should not be necessary since byte is a u8 and output.len() equals u8::max_value() + 1. 
        output[byte as usize] = output[byte as usize] + 1;
    }
    output
}

// Byte count can also be implemented using `fold`. 
// The following implementation makes me wonder if the accumulator gets copied every time.
pub fn byte_count_fold(input: &[u8]) -> [u32; 0x100] {
    input.iter().fold([0u32; 0x100], |mut counts, &byte| {
        counts[byte as usize] = counts[byte as usize] + 1;
        counts
    })
}

// Another question: Should `byte_count` return a statically allocated array or a dynamic vector? Is there a way to let the user choose?

#[cfg(test)]
mod tests {
    use super::byte_count;

    fn eq(a: &[u32], b: &[u32]) -> bool {
        a.iter().zip(b.iter()).all(|(a, b)| a == b)
    }

    #[test]
    fn empty() {
        assert!(eq(&byte_count(&[]), &[0u32; 0x100]));
    }

    #[test]
    fn one_minimal_byte() {
        let mut output = [0u32; 0x100];
        output[0x00] = 1;
        assert!(eq(&byte_count(&[ 0u8 ]), &output));
    }

    #[test]
    fn one_maximal_byte() {
        let mut output = [0u32; 0x100];
        output[0xff] = 1;
        assert!(eq(&byte_count(&[ 0xffu8 ]), &output));
    }

    #[test]
    fn multiple_bytes() {
        let input = [ 0u8, 1, 2, 3, 4, 1, 2, 100, 100, 120 ];
        let mut output = [0u32; 0x100];
        output[0] = 1;
        output[1] = 2;
        output[2] = 2;
        output[3] = 1;
        output[4] = 1;
        output[100] = 2;
        output[120] = 1;
        assert!(eq(&byte_count(&input), &output));
    }
}
