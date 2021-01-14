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
    }
}
