extern crate docopt;
extern crate rustc_serialize;
extern crate rand;

use std::collections::HashMap;

use docopt::Docopt;
use rand::random;

mod ppm;
use ppm::Image;

const USAGE: &'static str = "
Markov Image

Usage:
  markov_image <input> <output>
  markov_image \
                             (-h | --help)

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_input: String,
    arg_output: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    let img_in = Image::read(&args.arg_input);

    // Generate statistics
    let stats = {
        let mut stats = HashMap::new();
        for i in 0..((img_in.width() * img_in.height()) - 1) {
            let (x1, y1) = (i % img_in.width(), i / img_in.width());
            let (x2, y2) = ((i + 1) % img_in.width(), (i + 1) / img_in.width());

            let val1 = img_in.get(x1, y1);
            let val2 = img_in.get(x2, y2);

            *stats.entry(val1)
                  .or_insert_with(|| HashMap::new())
                  .entry(val2)
                  .or_insert(0) += 1;
        }
        stats
    };

    // Generate hilarious image
    let mut img_out = Image::new(img_in.width(), img_in.height());
    let mut current_val = (0, 0, 0);
    for pix_i in 0..(img_in.width() * img_in.height()) {
        let (x, y) = (pix_i % img_in.width(), pix_i / img_in.width());
        if let Some(pix_stats) = stats.get(&current_val) {
            let n = random::<usize>() % pix_stats.values().fold(0, |acc, n| acc + *n);
            let mut i = 0;
            for (val, count) in pix_stats.iter() {
                i += *count;
                if i > n {
                    current_val = *val;
                    break;
                }
            }
        } else {
            let n = random::<usize>() % stats.len();
            current_val = *stats.keys().nth(n).unwrap();
        }
        img_out.set(x, y, current_val);
    }

    img_out.write(&args.arg_output);
}
