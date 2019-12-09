use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_input(filename: Option<&str>) -> impl Iterator<Item = String> {
    let filename: &str = filename.unwrap_or("input.txt");
    let file = File::open(filename);

    if let Ok(file) = file {
        let reader = io::BufReader::new(file);
        reader.lines().map(|l| l.unwrap())
    } else {
        panic!("Could not open file: {}", filename)
    }
}

fn main() {
    let filename = env::args().nth(1);
    let input = read_input(filename.as_ref().map(String::as_str));
    challenge(input);
}

fn challenge(mut input: impl Iterator<Item = String>) {
    if let Some(line) = input.next() {
        let program: Vec<u32> =
            line.split(',').map(|s| s.parse::<u32>().unwrap()).collect();

        for noun in 0..99 {
            for verb in 0..99 {
                let mut program_copy = program.clone();
                program_copy[1] = noun;
                program_copy[2] = verb;

                let result = interpret(program_copy);
                if result[0] == 19690720 {
                    println!(
                        "noun: {}, verb: {}, answer: {}",
                        noun,
                        verb,
                        100 * noun + verb
                    );
                    return;
                }
            }
        }
        let result = interpret(program);
    }
}

pub enum OpCode {
    Add(Vec<u32>),
    Mul(Vec<u32>),
    Halt,
}

impl OpCode {
    pub fn new(pc: usize, program: &Vec<u32>) -> OpCode {
        let opcode = program[pc];
        match opcode {
            1 => OpCode::Add(OpCode::pack_args(pc, 4, program)),
            2 => OpCode::Mul(OpCode::pack_args(pc, 4, program)),
            99 => OpCode::Halt,
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }

    fn nargs(&self) -> usize {
        match self {
            OpCode::Add(_) => 4,
            OpCode::Mul(_) => 4,
            OpCode::Halt => 0,
        }
    }

    fn pack_args(pc: usize, capacity: usize, memory: &Vec<u32>) -> Vec<u32> {
        let mut args = vec![0; capacity];
        args.copy_from_slice(&memory[pc..pc + capacity]);
        args
    }

    pub fn exec(&self, memory: &mut Vec<u32>) {
        match self {
            OpCode::Add(args) => {
                if let &[_, arg1, arg2, ret] = args.as_slice() {
                    memory[ret as usize] =
                        memory[arg1 as usize] + memory[arg2 as usize];
                }
            }
            OpCode::Mul(args) => {
                if let &[_, arg1, arg2, ret] = args.as_slice() {
                    memory[ret as usize] =
                        memory[arg1 as usize] * memory[arg2 as usize];
                }
            }
            _ => {}
        }
    }
}

fn interpret(mut program: Vec<u32>) -> Vec<u32> {
    let mut pc = 0;
    loop {
        match OpCode::new(pc, &program) {
            OpCode::Halt => break,
            opcode => {
                opcode.exec(&mut program);
                pc += opcode.nargs();
            }
        }
    }

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        println!("{:?}", input);
        assert_eq!(interpret(input), result);
    }

    #[test]
    fn test2() {
        let input = vec![1, 0, 0, 0, 99];
        let result = vec![2, 0, 0, 0, 99];
        println!("{:?}", input);
        assert_eq!(interpret(input), result);
    }

    #[test]
    fn test3() {
        let input = vec![2, 3, 0, 3, 99];
        let result = vec![2, 3, 0, 6, 99];
        println!("{:?}", input);
        assert_eq!(interpret(input), result);
    }

    #[test]
    fn test4() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let result = vec![2, 4, 4, 5, 99, 9801];
        println!("{:?}", input);
        assert_eq!(interpret(input), result);
    }

    #[test]
    fn test5() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        println!("{:?}", input);
        assert_eq!(interpret(input), result);
    }
}
