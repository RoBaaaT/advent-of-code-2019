use std::fs;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

struct ImageLayer {
    pixels: Vec<u32>
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut layers = Vec::new();
    let mut current_layer = ImageLayer { pixels: Vec::new() };
    let mut pixels_read = 0;
    for pixel in input.chars() {
        if let Some(pixel) = pixel.to_digit(10) {
            current_layer.pixels.push(pixel);
            pixels_read += 1;
            if pixels_read == IMAGE_WIDTH * IMAGE_HEIGHT {
                pixels_read = 0;
                layers.push(current_layer);
                current_layer = ImageLayer { pixels: Vec::new() };
            }
        }
    }

    let mut fewest_zero_digits = std::u32::MAX;
    let mut fewest_zero_layer = 0;
    for (i, layer) in layers.iter().enumerate() {
        let mut zero_count = 0;
        for pixel in &layer.pixels {
            if *pixel == 0 {
                zero_count += 1;
            }
        }
        if zero_count < fewest_zero_digits {
            fewest_zero_digits = zero_count;
            fewest_zero_layer = i;
        }
    }

    let mut one_count = 0;
    let mut two_count = 0;
    for pixel in &layers[fewest_zero_layer].pixels {
        match *pixel {
            1 => one_count += 1,
            2 => two_count += 1,
            _ => {}
        }
    }
    println!("Part 1: {}", one_count * two_count);

    let mut image = ImageLayer { pixels: vec![0; IMAGE_WIDTH * IMAGE_HEIGHT] };
    for pixel in 0..IMAGE_WIDTH * IMAGE_HEIGHT {
        for layer in &layers {
            match layer.pixels[pixel] {
                0 => { image.pixels[pixel] = 0; break; },
                1 => { image.pixels[pixel] = 1; break; },
                2 => {},
                _ => panic!("Invalid image data")
            }
        }
    }

    for (i, pixel) in image.pixels.iter().enumerate() {
        match pixel {
            0 => print!(" "),
            1 => print!("#"),
            _ => panic!("Invalid image data")
        }
        if i % IMAGE_WIDTH == IMAGE_WIDTH - 1 {
            print!("\n");
        }
    }
}