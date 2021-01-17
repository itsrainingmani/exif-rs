#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_img_file_read() {
        let bytes: Vec<u8> = fs::read("DSCF1197.JPG").unwrap();
        println!("{:#?}", bytes.len());

        // Format the vector as hexadecimal integers with leading zeros
        // println!("{:#04X?}", bytes);
    }

    #[test]
    fn test_find_exif_marker() {
        let bytes: Vec<u8> = fs::read("DSCF1197.JPG").unwrap();

        // Convert vector of u8s to vector of hex strings
        let hex_bytes: Vec<String> = bytes.iter().map(|b| format!("{:X}", b)).collect();

        println!(
            "{:#?} {:#?}",
            hex_bytes.get(0).unwrap(),
            hex_bytes.get(1).unwrap()
        );

        println!("{:#?}", hex_bytes.get(0..20).unwrap());
    }
}
