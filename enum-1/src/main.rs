

#[derive(Debug)]
enum MediaType {
    PNG,
    JPG { quality_percentage: u32 },
    MPV,
    GIF { is_animated: bool },
    AVI(u32)
}

impl MediaType {
    fn is_image(self: &Self) -> bool {
        let is_image: bool = if matches!(self, MediaType::PNG) || matches!(self, MediaType::JPG { .. }) {
            true
        } else {
            false
        };
        is_image
    }
    fn is_image_match(&self) -> bool {
        let is_image = match self {
            MediaType::PNG => true,
            MediaType::JPG { .. } => true,
            MediaType::GIF { .. } => true,
            MediaType::AVI(x) => false,
            other => false
        };
        is_image
    }
    fn convert_with(self: &Self) -> Option<&u32> {
        let result: Option<&u32> = match self {
            MediaType::JPG { quality_percentage} => Some(quality_percentage),
            //MediaType::PNG => None,
            MediaType::GIF { is_animated} => None,
            MediaType::AVI(x) => Some(x),
            other => None
        };
        result
    }
}

fn main() {
    //let image_type = MediaType::JPG { quality_percentage: 65 };
    let image_type = MediaType::AVI(98);
    let mut is_image = image_type.is_image();
    if (is_image) {
        println!("YES, it is an image, {image_type:?}");
    } else {
        println!("NO, it is not an image, {image_type:?}");
    }
    is_image = image_type.is_image_match();
    if (is_image) {
        println!("YES, it is an image, {image_type:?}");
    } else {
        println!("NO, it is not an image, {image_type:?}");
    }
    let conversion_result = image_type.convert_with();
    if conversion_result.is_some() {
        let result = conversion_result.unwrap();
        println!("Conversion done with quality {result}");
    } else {
        println!("Conversion donw with no quality settings");
    }
}
