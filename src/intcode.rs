#[derive(Clone)]
pub struct VM {
    initial_program: Vec<i64>,
    program: Vec<i64>,
    program_pos: usize,
    relative_base: i64,
    input: Vec<i64>,
    input_pos: usize,
    output: Vec<i64>,
    output_pos: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum StepResult {
    Continue,
    Exit,
    InputRequired,
}

impl VM {
    pub fn reset(&mut self) {
        self.program.resize(self.initial_program.len(), 0);
        self.program.copy_from_slice(&self.initial_program);
        self.program_pos = 0;
        self.relative_base = 0;
        self.input.clear();
        self.input_pos = 0;
        self.output.clear();
        self.output_pos = 0;
    }

    pub fn peek_input(&self) -> &[i64] {
        &self.input[self.input_pos..]
    }

    pub fn set_memory(&mut self, index: usize, v: i64) {
        self.program[index] = v;
    }

    pub fn push_input(&mut self, v: i64) {
        self.input.push(v);
    }

    pub fn read_output(&mut self) -> &[i64] {
        let output_pos = self.output_pos;
        self.output_pos = self.output.len();

        &self.output[output_pos..]
    }

    pub fn output(&self) -> &[i64] {
        &self.output
    }

    fn resolve_addr(&self, addr: usize, mode: i32) -> usize {
        match mode {
            0 => self.program[addr] as usize,
            1 => addr,
            2 => (self.program[addr] + self.relative_base) as usize,
            _ => unreachable!(),
        }
    }

    fn get_addr(&mut self, addr: usize, mode: i32) -> usize {
        let addr = self.resolve_addr(addr, mode);
        if addr >= self.program.len() {
            self.program.resize(addr + 9, 0);
        }

        addr
    }

    fn read(&self, addr: usize) -> i64 {
        self.program[addr]
    }

    fn write(&mut self, addr: usize, v: i64) {
        self.program[addr] = v;
    }

    pub fn print_next(&self) {
        let position = self.program_pos;
        let (opcode, m1, m2, m3) = parse_opcode(self.program[position] as i32);

        let chars = ['&', '=', 'r'];

        match opcode {
            1 => println!("{}{} + {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            2 => println!("{}{} * {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            3 => println!("read({}{})",
                          chars[m1 as usize], self.program[position+1],
            ),
            4 => println!("write({}{})",
                          chars[m1 as usize], self.program[position+1],
            ),
            5 => println!("jit({}{}, {}{})",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
            ),
            6 => println!("jif({}{}, {}{})",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
            ),
            7 => println!("{}{} < {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            8 => println!("{}{} == {}{} => {}{}",
                          chars[m1 as usize], self.program[position+1],
                          chars[m2 as usize], self.program[position+2],
                          chars[m3 as usize], self.program[position+3],
            ),
            9 => println!("relative({}{})",
                          chars[m1 as usize], self.program[position+1],
            ),
            99 => println!("exit"),
            _ => panic!("unknown opcode {}", opcode),
        }
    }

    pub fn run(&mut self) -> StepResult {
        loop {
            let result = self.step();
            if result != StepResult::Continue {
                return result;
            }
        }
    }

    pub fn quick_run(&mut self, input: &[i64]) -> i64 {
        self.reset();
        for v in input {
            self.push_input(*v);
        }
        self.run();

        *self.output.last().unwrap()
    }

    pub fn step(&mut self) -> StepResult {
        let position = self.program_pos;
        let (opcode, m1, m2, m3) = parse_opcode(self.program[position] as i32);

        match opcode {
            1 => {
                self.program_pos += 4;
                let addr1 = self.get_addr(position + 1, m1);
                let addr2 = self.get_addr(position + 2, m2);
                let addr3 = self.get_addr(position + 3, m3);

                self.write(addr3, self.read(addr2) + self.read(addr1));

                StepResult::Continue
            }
            2 => {
                self.program_pos += 4;
                let addr1 = self.get_addr(position + 1, m1);
                let addr2 = self.get_addr(position + 2, m2);
                let addr3 = self.get_addr(position + 3, m3);

                self.write(addr3, self.read(addr2) * self.read(addr1));

                StepResult::Continue
            }
            3 => {
                if self.input_pos == self.input.len() {
                    StepResult::InputRequired
                } else {
                    self.program_pos += 2;
                    let addr1 = self.get_addr(position + 1, m1);

                    self.write(addr1, self.input[self.input_pos]);
                    self.input_pos += 1;

                    if self.input_pos == self.input.len() {
                        self.input_pos = 0;
                        self.input.clear();
                    }

                    StepResult::Continue
                }
            }
            4 => {
                if self.output_pos > 0 && self.output_pos == self.output.len() {
                    self.output_pos = 0;
                    self.output.clear();
                }

                self.program_pos += 2;
                let addr1 = self.get_addr(position + 1, m1);

                self.output.push(self.read(addr1));

                StepResult::Continue
            }
            5 => {
                let addr1 = self.get_addr(position + 1, m1);

                if self.read(addr1) != 0 {
                    let addr2 = self.get_addr(position + 2, m2);

                    self.program_pos = self.read(addr2) as usize;
                } else {
                    self.program_pos += 3;
                }

                StepResult::Continue
            }
            6 => {
                let addr1 = self.get_addr(position + 1, m1);

                if self.read(addr1) == 0 {
                    let addr2 = self.get_addr(position + 2, m2);

                    self.program_pos = self.read(addr2) as usize;
                } else {
                    self.program_pos += 3;
                }

                StepResult::Continue
            }
            7 => {
                self.program_pos += 4;
                let addr1 = self.get_addr(position + 1, m1);
                let addr2 = self.get_addr(position + 2, m2);
                let addr3 = self.get_addr(position + 3, m3);

                self.write(addr3,
                    (self.read(addr1) < self.read(addr2)) as i64,
                );

                StepResult::Continue
            }
            8 => {
                self.program_pos += 4;
                let addr1 = self.get_addr(position + 1, m1);
                let addr2 = self.get_addr(position + 2, m2);
                let addr3 = self.get_addr(position + 3, m3);

                self.write(addr3,
                    (self.read(addr1) == self.read(addr2)) as i64,
                );

                StepResult::Continue
            }
            9 => {
                self.program_pos += 2;
                let addr1 = self.get_addr(position + 1, m1);

                self.relative_base += self.read(addr1);

                StepResult::Continue
            }
            99 => {
                StepResult::Exit
            }
            _ => panic!("Unknown opcode {}", opcode)
        }
    }

    pub fn new(initial_program: &[i64]) -> VM {
        VM{
            initial_program: initial_program.to_vec(),
            program: initial_program.to_vec(),
            program_pos: 0,
            relative_base: 0,
            input: Vec::with_capacity(16),
            input_pos: 0,
            output: Vec::with_capacity(16),
            output_pos: 0,
        }
    }

    pub fn parse(program_data: &str) -> VM {
        let data: Vec<i64> = program_data.split(',').map(|t| t.parse::<i64>().unwrap()).collect();
        Self::new(&data)
    }
}

pub fn parse_opcode(code: i32) -> (i32, i32, i32, i32) {
    (code % 100, ((code / 100) % 10), ((code / 1000) % 10), ((code / 10000) % 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm() {
        let mut vm = VM::parse("103,13,1001,13,5,13,1002,13,14,14,4,14,99,5,5");
        vm.reset();

        assert_eq!(vm.step(), StepResult::InputRequired);

        vm.push_input(5);

        assert_eq!(vm.step(), StepResult::Continue);
        assert_eq!(vm.step(), StepResult::Continue);
        assert_eq!(vm.step(), StepResult::Continue);
        assert_eq!(vm.step(), StepResult::Continue);
        assert_eq!(vm.step(), StepResult::Exit);

        assert_eq!(vm.output().len(), 1);
        assert_eq!(vm.output()[0], 140);
    }

    #[test]
    fn test_day05_part2() {
        let mut vm1 = VM::parse("3,9,8,9,10,9,4,9,99,-1,8");
        assert_eq!(vm1.quick_run(&[8]), 1);
        assert_eq!(vm1.quick_run(&[7]), 0);
        assert_eq!(vm1.quick_run(&[9]), 0);

        let mut vm2 = VM::parse("3,9,7,9,10,9,4,9,99,-1,8");
        assert_eq!(vm2.quick_run(&[8]), 0);
        assert_eq!(vm2.quick_run(&[7]), 1);
        assert_eq!(vm2.quick_run(&[9]), 0);

        let mut vm3 = VM::parse("3,3,1108,-1,8,3,4,3,99");
        assert_eq!(vm3.quick_run(&[8]), 1);
        assert_eq!(vm3.quick_run(&[7]), 0);
        assert_eq!(vm3.quick_run(&[9]), 0);

        let mut vm4 = VM::parse("3,3,1107,-1,8,3,4,3,99");
        assert_eq!(vm4.quick_run(&[8]), 0);
        assert_eq!(vm4.quick_run(&[7]), 1);
        assert_eq!(vm4.quick_run(&[9]), 0);

        let mut vm5 = VM::parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        assert_eq!(vm5.quick_run(&[0]), 0);
        assert_eq!(vm5.quick_run(&[-1]), 1);
        assert_eq!(vm5.quick_run(&[1]), 1);

        let mut vm6 = VM::parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        assert_eq!(vm6.quick_run(&[0]), 0);
        assert_eq!(vm6.quick_run(&[-1]), 1);
        assert_eq!(vm6.quick_run(&[1]), 1);

        let mut vm7 = VM::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        assert_eq!(vm7.quick_run(&[-4]), 999);
        assert_eq!(vm7.quick_run(&[8]), 1000);
        assert_eq!(vm7.quick_run(&[14]), 1001);
    }

    #[test]
    fn test_day09_part1() {
        let mut vm1 = VM::parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        assert_eq!(vm1.run(), StepResult::Exit);
        assert_eq!(vm1.output(), &[109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);

        let mut vm3 = VM::parse("1102,34915192,34915192,7,4,7,99,0");
        assert!(vm3.quick_run(&[]) >= 1000000000000000);

        let mut vm3 = VM::parse("104,1125899906842624,99");
        assert_eq!(vm3.quick_run(&[]), 1125899906842624);
    }

    #[test]
    fn test_read_output() {
        let mut vm = VM::parse("104,1,104,2,104,3,99");

        assert_eq!(vm.run(), StepResult::Exit);
        assert_eq!(vm.read_output(), &[1, 2, 3]);
        assert_eq!(vm.read_output(), &[]);
    }

    #[test]
    fn test_parse_opcode() {
        assert_eq!(parse_opcode(99), (99, 0, 0, 0));
        assert_eq!(parse_opcode(199), (99, 1, 0, 0));
        assert_eq!(parse_opcode(10102), (2, 1, 0, 1));
        assert_eq!(parse_opcode(1102), (2, 1, 1, 0));
        assert_eq!(parse_opcode(1003), (3, 0, 1, 0));
        assert_eq!(parse_opcode(10004), (4, 0, 0, 1));
    }
}