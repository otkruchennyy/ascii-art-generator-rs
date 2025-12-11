use image::DynamicImage;

pub struct FormatImg<'a> { // format img. determines which values are standard
    pub path: &'a str,
    // pub width: u16,
    pub invert: &'a bool,
    // pub contrast: u16,
    // pub brightness: u16,
}
impl<'a> FormatImg<'a> {
    pub fn new(path: &'a str, invert: &'a bool) -> Self {
        FormatImg{ path, invert }
    }

    pub fn process(&self) -> Result<DynamicImage, image::ImageError> { // FIXME:
        let mut img: DynamicImage = image::open(&self.path)?;

        // invert
        if *self.invert {
            img.invert();
        }
        let img_width: u16 = img.width() as u16; // FIXME: The value may exceed the limit in rare cases
        let img_height: u16 = img.height() as u16; // FIXME: The value may exceed the limit in rare cases
        // let user_width = self.width;
        // let user_contrast = self.contrast;
        // let user_brightness = self.brightness;
        // println!("{user_width}, {user_contrast}, {user_brightness}");

        // aspect ratio
        let ar_w: u16 = aspect_ratio(img_width, img_height).1;
        let ar_h: u16 = aspect_ratio(img_width, img_height).0;
        println!("{ar_w}, {ar_h}"); // FIXME:
            
        // get aspect ratio <u16>, <u16>
        Ok(img)
        }
}

    // return [16, 9], [1, 1], [5, 3]. [width, height]
fn aspect_ratio(mut w: u16, mut h: u16) -> (u16, u16) { // return [16, 9], [1, 1], [5, 3]. [width, height]
    while find_common_divisors(w, h) != 1 {
        w /= find_common_divisors(w, h);
        h /= find_common_divisors(w, h);
    }
    (w, h)
}

    // return greatest common factor [1 - common divs unfinded] <u16>, <u16>
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

    // find all divs fom value <u16>
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