#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    // use crate::hex_to_u32;

    #[test]
    fn test_img_file_read() {
        let bytes: Vec<u8> = fs::read("DSCF1197.JPG").unwrap();
        println!("{:#?}", bytes.len());

        // Format the vector as hexadecimal integers with leading zeros
        // println!("{:#04X?}", bytes);
    }

    #[test]
    fn test_vector_sizes() {
        // u8 is 8 bit unsigned integer -> 1 byte of data
        let bytes: Vec<u8> = fs::read("DSCF1197.JPG").unwrap();

        println!("Size of u8 Vec: {}", bytes.capacity());

        // Convert vector of u8s to vector of hex strings
        // Each String has length = 2 bytes.
        // So this format occupies atleast twice the memory of Vec<u8>
        let hex_bytes: Vec<String> = bytes.iter().map(|b| format!("{:X}", b)).collect();

        println!("Size of String Vec: {}", hex_bytes.capacity());
    }

    #[test]
    fn test_find_exif_marker() {
        let bytes = fs::read("DSCF1197.JPG").unwrap();

        println!("{:#?} {:#?}", bytes.get(0).unwrap(), bytes.get(1).unwrap());
        // 255 216 => FF D8
        println!("{:#?} {:#?}", bytes.get(2).unwrap(), bytes.get(3).unwrap());
        // 255 225 => FF E1

        // println!("{:#?}", hex_bytes.get(0..20).unwrap());
    }

    #[test]
    fn test_first_pass() {
        let img_path = Path::new("DSCF1197.JPG");
        let img_bytes = fs::read("DSCF1197.JPG").unwrap();

        let img_ext = img_path.extension().unwrap().to_str().unwrap();

        // Validate the file extension
        match img_ext {
            "JPG" | "jpg" | "JPEG" | "jpeg" => println!("Valid JPEG File"),
            _ => println!("Not a valid JPEG File"),
        }

        let mut pass_bytes: Vec<u8> = Vec::with_capacity(img_bytes.len());

        let (s1, s2) = (img_bytes.get(4).unwrap(), img_bytes.get(5).unwrap());
        println!("{:02X}{:02X}", s1, s2);

        // for x in (1..60).step_by(2) {
        //     let (prev, curr) = (img_bytes.get(x - 1).unwrap(), img_bytes.get(x).unwrap());

        //     match (prev, curr) {
        //         (255, 216) => println!("SOI"),
        //         (255, 225) => println!("APP1"),
        //         (p, c) => {
        //             println!("{:02X} {:02X}    |    {} {}", p, c, p, c);
        //             pass_bytes.push(*p);
        //             pass_bytes.push(*c);
        //         }
        //     }
        // }

        println!("{:#?}", u32::from_str_radix("FFBC", 16));
    }

    #[test]
    fn test_maintain_state() {
        let img_path = Path::new("DSCF1197.JPG");
        let img_bytes = fs::read(img_path).unwrap();

        let mut current_data_size: usize = 0; // Indicated by the marker (FFBC => 65468)
        let mut current_marker_start: usize = 0; // Where in the iteration did we start
        let mut is_app1_marker = false;

        let mut marker_bytes: Vec<u8> = Vec::new();

        // Go by steps of 2 since the markers are 2 bytes wide
        for x in (1..img_bytes.len()).step_by(2) {
            let (prev, curr) = (img_bytes.get(x - 1).unwrap(), img_bytes.get(x).unwrap());

            match (prev, curr) {
                (255, 216) => println!("FFD8 - SOI Marker @ {}", x),
                (255, 225) => {
                    println!("FFE1 - APP1 Marker @ {}", x);
                    is_app1_marker = true;
                }
                (255, y) if is_app1_marker => {
                    println!("Within APP1 marker");

                    let marker_hex_string = format!("{:02X}{:02X}", prev, y);

                    // This converts a Hex String (Base 16) into a u32
                    // println!("{:#?}", u32::from_str_radix("FFBC", 16));
                    // Deducting 2 to count for the size of the marker itself
                    let marker_size =
                        u32::from_str_radix(marker_hex_string.as_str(), 16).unwrap() - 2;
                    current_data_size = marker_size as usize;
                    current_marker_start = x;

                    // if x == current_data_size + current_marker_start {}

                    println!("{:#?} {:#?}", marker_hex_string, marker_size);
                }
                (p, c) if is_app1_marker => {
                    if x + current_marker_start >= current_data_size {
                        is_app1_marker = false;
                        println!("Size of marker bytes: {}", marker_bytes.len());
                    }

                    // println!("{:02X}{:02X}", p, c);
                    marker_bytes.push(*p);
                    marker_bytes.push(*c);
                }
                _ => {}
            }
        }
    }
}
