
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub struct Image {
    data: Vec<(u8, u8, u8)>,
    res: (usize, usize),
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            data: vec![(0, 0, 0); width * height],
            res: (width, height),
        }
    }
    pub fn read(path: &str) -> Image {
        let file = BufReader::new(File::open(path).unwrap());
        let (mut i, mut d) = (0, 0);
        let (mut width, mut height, mut img) = (0, 0, Image::new(0, 0));
        let (mut pixel, mut p) = ([0, 0, 0], 0);

        for line in file.lines() {
            for value in line.unwrap().split_whitespace() {
                if value.chars().nth(0).unwrap() == '#' {
                    break;
                }
                match i {
                    0 => (),
                    1 => width = value.parse::<usize>().unwrap(),
                    2 => height = value.parse::<usize>().unwrap(),
                    3 => img = Image::new(width, height),
                    _ => {
                        pixel[p] = value.parse::<u8>().unwrap();
                        if p == 2 {
                            img.data[d] = (pixel[0], pixel[1], pixel[2]);
                            d += 1;
                            p = 0;
                        } else {
                            p += 1;
                        }
                    }
                }
                i += 1;
            }
        }
        return img;
    }
    pub fn write(&self, path: &str) {
        let mut file = File::create(path).unwrap();

        let _ = write!(&mut file, "P3\n");
        let _ = write!(&mut file, "{} {} 255\n", self.res.0, self.res.1);

        for pixel in self.data.iter() {
            let _ = write!(&mut file, "{} {} {} ", pixel.0, pixel.1, pixel.2);
        }
    }
    pub fn get(&self, x: usize, y: usize) -> (u8, u8, u8) {
        self.data[y * self.res.0 + x]
    }
    pub fn set(&mut self, x: usize, y: usize, val: (u8, u8, u8)) {
        self.data[y * self.res.0 + x] = val;
    }
    pub fn width(&self) -> usize {
        self.res.0
    }
    pub fn height(&self) -> usize {
        self.res.1
    }
}
