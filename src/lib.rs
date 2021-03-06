pub mod exif {

    fn convert_to_hex(v: &Vec<u8>, start: usize, end: usize) -> Result<String, &'static str> {
        Ok(v.get(start..end)
            .unwrap()
            .iter()
            .map(|a| format!("{:02X}", a))
            .collect::<Vec<String>>()
            .join(""))
    }

    pub type ExifVec = Vec<u8>;
    pub trait Exif {
        fn convert_to_hex(&self, start: usize, end: usize) -> Result<String, &'static str>;
        fn process_image(&self) -> Result<(), &'static str>;
    }

    impl Exif for ExifVec {
        fn convert_to_hex(&self, start: usize, end: usize) -> Result<String, &'static str> {
            Ok(self
                .get(start..end)
                .unwrap()
                .iter()
                .map(|a| format!("{:02X}", a))
                .collect::<Vec<String>>()
                .join(""))
        }

        fn process_image(&self) -> Result<(), &'static str> {
            let mut is_app1_marker = false;

            if let &[255, 216] = self.get(0..2).expect("Couldn't get byte slice at index") {
                println!("SOI Marker");
            } else {
                println!("SOI Marker FFD8 not found");
                std::process::exit(-1);
            }

            if let &[255, 225] = self.get(2..4).expect("Couldn't get byte slice at index") {
                println!("APP1 Marker");
                is_app1_marker = true;
            } else {
                println!("APP1 Marker FFE1 not found");
                std::process::exit(-1);
            }

            if is_app1_marker {
                println!("Within APP1 Marker section");

                // Get the size of the APP1 segment
                let app1_size_hex =
                    format!("{:02X}{:02X}", self.get(4).unwrap(), self.get(5).unwrap());

                // This converts a Hex String (Base 16) into a usize
                // println!("{:#?}", usize::from_str_radix("FFBC", 16));
                // Deducting 2 to count for the size of the marker itself
                let app1_data_size = usize::from_str_radix(app1_size_hex.as_str(), 16).unwrap() - 2;
                let app1_data_end = app1_data_size + 6;

                println!("{:#?} {:#?}", app1_size_hex, app1_data_size);

                let app1_bytes: ExifVec = Vec::from(self.get(6..app1_data_end).unwrap());
                println!("Size of App1 Bytes Vector - {:#?}", app1_bytes.len());

                // Check if the Exif header is present - 45786969 0000
                // This translates to Exif0000

                let exif_header = app1_bytes.convert_to_hex(0, 6).unwrap();
                if exif_header == String::from("457869660000") {
                    println!("Exif Header is present");
                }
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Exif, ExifVec};
        use std::fs;
        use std::path::Path;

        #[test]
        fn test_img_file_read() {
            let bytes: ExifVec = fs::read("DSCF1197.JPG").unwrap();
            println!("{:#?}", bytes.len());

            // Format the vector as hexadecimal integers with leading zeros
            // println!("{:#04X?}", bytes);
        }

        #[test]
        fn test_buncha_stuff() {
            // u8 is 8 bit unsigned integer -> 1 byte of data
            let bytes: Vec<u8> = fs::read("DSCF1197.JPG").unwrap();
            let img_path = Path::new("DSCF1197.JPG");

            println!("Size of u8 Vec: {}", bytes.capacity());

            // Convert vector of u8s to vector of hex strings
            // Each String has length = 2 bytes.
            // So this format occupies atleast twice the memory of Vec<u8>
            let hex_bytes: Vec<String> = bytes.iter().map(|b| format!("{:X}", b)).collect();

            println!("Size of String Vec: {}", hex_bytes.capacity());

            println!("{:#?} {:#?}", bytes.get(0).unwrap(), bytes.get(1).unwrap());
            // 255 216 => FF D8
            println!("{:#?} {:#?}", bytes.get(2).unwrap(), bytes.get(3).unwrap());
            // 255 225 => FF E1

            // println!("{:#?}", hex_bytes.get(0..20).unwrap());

            let img_ext = img_path.extension().unwrap().to_str().unwrap();

            // Validate the file extension
            match img_ext {
                "JPG" | "jpg" | "JPEG" | "jpeg" => println!("Valid JPEG File"),
                _ => println!("Not a valid JPEG File"),
            }

            let mut pass_bytes: Vec<u8> = Vec::with_capacity(bytes.len());

            let (s1, s2) = (bytes.get(4).unwrap(), bytes.get(5).unwrap());
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
        fn test_process_image() {
            let img_path = Path::new("DSCF1197.JPG");
            let img_bytes: ExifVec = fs::read(img_path).unwrap();

            assert!(img_bytes.process_image().is_ok())
        }
    }
}
