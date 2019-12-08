use crate::solver::Solver;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u8>;
    type Output1 = usize;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.bytes().flatten().map(|b| b - b'0').collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let img = Img::from_array(25, 6, input);

        let (_, c1, c2) = img
            .layers
            .iter()
            .map(|layer| count(&layer))
            .min_by_key(|&(c0, _, _)| c0)
            .unwrap();
        c1 * c2
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let img = Img::from_array(25, 6, input);
        img.display();

        // visual solution
        String::from("BCPZB")
    }
}

fn count(pixels: &[u8]) -> (usize, usize, usize) {
    let mut c = [0, 0, 0];
    for &p in pixels {
        c[p as usize] += 1;
    }
    (c[0], c[1], c[2])
}

struct Img {
    layers: Vec<Vec<u8>>,
    w: usize,
    h: usize,
}

impl Img {
    fn from_array(w: usize, h: usize, pixels: &[u8]) -> Self {
        let layer_size = w * h;
        let layers = pixels
            .chunks(layer_size)
            .map(|layer| layer.into())
            .collect();
        Self { layers, w, h }
    }

    fn display(&self) {
        let pixels = self.rasterize();
        pixels.chunks(self.w).for_each(|row| {
            for p in row {
                let c = if *p == 1 { '▮' } else { ' ' };
                print!("{}", c);
            }
            println!();
        });
    }

    fn rasterize(&self) -> Vec<u8> {
        let size = self.w * self.h;
        (0..size)
            .map(|i| {
                self.layers
                    .iter()
                    .map(|layer| layer[i])
                    .find(|&e| e != 2)
                    .unwrap()
            })
            .collect()
    }
}
