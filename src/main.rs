
mod ppm;
use ppm::Image;

fn main() {
    let mut img = Image::read("tubular.ppm");
    for x in 0..32 {
        for y in 0..32 {
            img.set(x, y, (0, 0, 0));
        }
    }

    img.write("vagina.ppm");
}
