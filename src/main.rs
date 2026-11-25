use image::{GenericImageView};

fn get_string_ascii(intent: u8)-> &'static str {
    let ascii = [" ",".",",","-","+","=","@"];
    // Map 0..=255 evenly onto 0..ascii.len()
    let index = intent as usize * ascii.len() / 256;
    ascii[index]
}

fn get_image (dir: &str, scale: u32){
    println!("This is the Image process");
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0{
                let pixel = img.get_pixel(x, y);
                let mut intent = pixel[0]/3 + pixel[1]/3 + pixel[2]/3;
                if pixel[3] == 0{
                    intent = 0;
                }
                print!("{}", get_string_ascii(intent));
            }
        }
        if y % (scale * 2) == 0 {
            println!();
        }
    }
}
fn main() {
    println!("This is the Main step");
    get_image("./image.png", 6);
}
