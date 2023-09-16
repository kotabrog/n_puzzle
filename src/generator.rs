extern crate rand;

use rand::Rng;
use crate::reverse_polish_notation::rpn;

struct PuzzleGenerator {
    rng: rand::rngs::ThreadRng,
    op_list: Vec<char>,
}

impl PuzzleGenerator {
    pub fn new() -> PuzzleGenerator {
        PuzzleGenerator {
            rng: rand::thread_rng(),
            op_list: vec!['+', '-', '*', '/'],
        }
    }

    fn gen_range(&mut self, min: usize, max: usize) -> usize {
        self.rng.gen_range(min..max)
    }

    fn generate_num_char(&mut self) -> char {
        (self.gen_range(1, 10) as u8 + '0' as u8) as char
    }

    fn generate_op_char(&mut self) -> char {
        let op = self.gen_range(0, 4);
        self.op_list[op]
    }

    fn generate_num_and_op_char(&mut self) -> char {
        let num = self.gen_range(0, 2);
        if num == 0 {
            self.generate_num_char()
        } else {
            self.generate_op_char()
        }
    }

    pub fn generate(&mut self, size: usize) -> String {
        let mut puzzle = String::new();
        let mut num_count = 0;
        let mut op_count = 0;

        let op_max = size - 1;

        while num_count < size || op_count < op_max {
            if num_count == size {
                puzzle.push(self.generate_op_char());
                op_count += 1;
            } else if num_count < op_count + 2 {
                puzzle.push(self.generate_num_char());
                num_count += 1;
            } else {
                let c = self.generate_num_and_op_char();
                if c.is_numeric() {
                    num_count += 1;
                } else {
                    op_count += 1;
                }
                puzzle.push(c);
            }
        }
        puzzle
    }
}

pub fn generator(num: usize) -> String {
    let mut generator = PuzzleGenerator::new();
    for _ in 0..1000 {
        let puzzle = generator.generate(num);
        let frac = match rpn(&puzzle) {
            Ok(frac) => frac,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };
        if frac.den == 1 && frac.num >= 0 {
            return format!("{} = {}", puzzle, frac.num)
        }
        println!("{} = {}/{}", puzzle, frac.num, frac.den)
    }
    "None".to_string()
}
