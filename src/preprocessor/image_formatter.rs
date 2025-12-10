use image::DynamicImage;

pub struct FormatImg {
    path: String,
    width: u16,
    invert: bool,
    contrast: u16,
    brightness: u16,
}
impl FormatImg {
    pub fn process(&self) -> Result<DynamicImage, image::ImageError> { // FIXME:
        let mut img: DynamicImage = image::open(&self.path)?;

        // invert
        if self.invert == true {
            img.invert();
        }
        let width: u16 = img.width() as u16; // FIXME
        let height: u16 = img.width() as u16; // FIXME
        // let aspect_ratio
        Ok(img); 
        aspect_ratio(width, height);
        println!("{aspect_ratio}"); // FIXME:

            
        // get aspect ratio
        pub fn aspect_ratio(mut w: u16, mut h: u16) -> (u16, u16) {
            while find_common_divisors(w, h) != 1 {
                w /= find_common_divisors(w, h);
                h /= find_common_divisors(w, h);
            }
            
            // Greatest_common_factor ( 1 - Absent )
            fn find_common_divisors(w: u16, h: u16) -> u16 {
                let divs_w: Vec<u16> = divs(w);
                let divs_h: Vec<u16> = divs(h);
                for i in divs_w.iter().rev() {
                    for j in divs_h.iter().rev() {
                        if i == j {
                            return *i;
                        }
                    }
                }
                return 1;
            }

            // find divs
            fn divs( value: u16 ) -> Vec<u16> {
                let mut result: Vec<u16> = Vec::new();
                for i in 1..(value as f64).sqrt() as u16 + 1 {
                    if value % i == 0 {
                        result.push(i);
                        result.push(value / i);
                    }
                }
            result
            }
        (w, h)
        }
    }
}
