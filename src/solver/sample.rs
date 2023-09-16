use rand::Rng;
use crate::reverse_polish_notation::rpn;
use crate::frac::Frac;

struct Generator {
    rng: rand::rngs::ThreadRng,
    op_list: Vec<char>,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            rng: rand::thread_rng(),
            op_list: vec!['+', '-', '*', '/'],
        }
    }

    fn gen_range(&mut self, min: usize, max: usize) -> usize {
        self.rng.gen_range(min..max)
    }

    fn generate_num_char(&mut self, mut nums: Vec<u32>) -> (char, Vec<u32>) {
        let index = self.gen_range(0, nums.len());
        let num = nums.remove(index);
        ((num as u8 + '0' as u8) as char, nums)
    }

    fn generate_op_char(&mut self) -> char {
        let op = self.gen_range(0, 4);
        self.op_list[op]
    }

    fn generate_num_and_op_char(&mut self, nums: Vec<u32>) -> (char, Vec<u32>) {
        let num = self.gen_range(0, 2);
        if num == 0 {
            let (c, nums) = self.generate_num_char(nums);
            (c, nums)
        } else {
            (self.generate_op_char(), nums)
        }
    }

    pub fn generate(&mut self, mut nums: Vec<u32>) -> String {
        let mut puzzle = String::new();
        let mut num_count = 0;
        let mut op_count = 0;

        let size = nums.len();
        let op_max = size - 1;

        while num_count < size || op_count < op_max {
            if num_count == size {
                puzzle.push(self.generate_op_char());
                op_count += 1;
            } else if num_count < op_count + 2 {
                let (c, temp) = self.generate_num_char(nums);
                nums = temp;
                puzzle.push(c);
                num_count += 1;
            } else {
                let (c, temp) = self.generate_num_and_op_char(nums);
                nums = temp;
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

pub fn solver(nums: Vec<u32>, ans: Frac) -> String {
    let mut generator = Generator::new();
    for _ in 0..1000 {
        let puzzle = generator.generate(nums.clone());
        let frac = match rpn(&puzzle) {
            Ok(frac) => frac,
            Err(_) => continue,
        };
        if ans == frac {
            return puzzle;
        }
    }
    return String::new();
}
