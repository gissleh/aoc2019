use common::aoc::{load_input, run_many, print_time, print_result, print_result_multiline};

fn main() {
    let input = load_input("day08");

    let (image, dur_parse) = run_many(1000, || Image::parse(&input, 25, 6));
    let (res_part1, dur_part1) = run_many(1000, || image.best_layer_checksum());
    let (res_part2, dur_part2) = run_many(1000, || image.render());

    print_result("P1", res_part1);
    print_result_multiline("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

const ZERO_BYTE: i32 = '0' as i32;

struct Image {
    width: usize,
    height: usize,
    layers: Vec<Vec<i32>>,
}

impl Image {
    fn add_layer(&mut self, data: &[i32]) {
        self.layers.push(data.to_vec());
    }

    fn best_layer_checksum(&self) -> usize {
        let mut fewest_zeroes = self.width * self.height;
        let mut checksum = 0;

        for layer in self.layers.iter() {
            let mut counts = [0; 3];

            for pixel in layer.iter() {
                counts[*pixel as usize] += 1;
            }

            if counts[0] < fewest_zeroes {
                fewest_zeroes = counts[0];
                checksum = counts[1] * counts[2];
            }
        }

        checksum
    }

    fn render(&self) -> String {
        let mut render = self.layers.first().unwrap().clone();

        for layer in self.layers.iter().skip(1) {
            for (i, layer_pixel) in layer.iter().enumerate() {
                if render[i] == 2 {
                    render[i] = *layer_pixel;
                }
            }
        }

        let mut result = String::with_capacity((self.width * self.height) + self.height);

        for i in 0..self.height {
            let pos = i * self.width;
            for pixel in &render[pos..pos + self.width] {
                result.push(match *pixel {
                    0 => '.',
                    1 => '#',
                    2 => ' ',
                    _ => panic!("invalid pixel {}", pixel),
                });
            }

            result.push('\n');
        }

        result
    }

    fn new(width: usize, height: usize) -> Image {
        Image{
            width, height,
            layers: Vec::with_capacity(16),
        }
    }

    fn parse(str: &str, width: usize, height: usize) -> Image {
        let mut image = Self::new(width, height);
        let pixel_count = width * height;

        let mut current_layer: Vec<i32> = Vec::with_capacity(width * height);
        for ch in str.chars() {
            current_layer.push(ch as i32 - ZERO_BYTE);

            if current_layer.len() == pixel_count {
                image.add_layer(&current_layer);
                current_layer.clear();
            }
        }

        image
    }
}