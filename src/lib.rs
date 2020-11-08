use std::fs::File;
use std::io::Read;

mod display;
use crate::display::Display;

type Instruction = u16;

// Some byte/nibble manipulation helpers to avoid code duplication
fn get_byte_from_u16(input: u16, index: isize) -> Result<u8, &'static str> {
    match index {
        0 => Ok((input >> 8) as u8),
        1 => Ok((input % (1 << 8)) as u8),
        _ => Err("Index out of range"),
    }
}

fn get_nibble_from_byte(input: u8, index: isize) -> Result<u8, &'static str> {
    match index {
        0 => Ok(input >> 4),
        1 => Ok(input % (1 << 4)),
        _ => Err("Index out of range"),
    }
}

pub struct Chip8Cpu {
    display: Display,
    program_counter: usize,
    ram: [u8; 4096],
    index_register: u16,
    register: [u8; 16],
    stack: Vec<u16>,
}

impl Chip8Cpu {
    pub fn new(file: File) -> Result<Chip8Cpu, std::io::Error> {
        let mut chip8_cpu = Chip8Cpu {
            display: Display::new(),
            program_counter: 200,
            ram: [0; 4096],
            index_register: 0,
            register: [0; 16],
            stack: Vec::new(),
        };

        match chip8_cpu.load_rom(file) {
            Ok(_) => Ok(chip8_cpu),
            Err(error) => Err(error),
        }
    }

    fn load_rom(&mut self, mut file: File) -> Result<(), std::io::Error> {
        // Starting index of rom in ram is for compatibility with older roms
        match file.read(&mut self.ram[200..]) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn fetch(&self) -> Instruction {
        // Instructions are two bytes but the RAM is represented as u8, so
        // assemble it.
        let instruction: Instruction =
            ((self.ram[self.program_counter] as u16) << 8) +
            (self.ram[self.program_counter + 1] as u16);

        return instruction;
    }

    // This covers the contents of the function and allows us to use canonical
    // chip-8 names for the instruction parts without triggering warnings.
    #[allow(non_snake_case)]
    fn decode(&mut self, instruction: Instruction) {
        // Split the instruction into all possible parts from the start. Note
        // that some of them are overlapping because the instructions have
        // different syntaxes. Using the standard chip-8 notations
        // X:   nibble 2
        // Y:   nibble 3
        // N:   nibble 4
        // NN:  nibbles 3-4 (byte 2)
        // NNN: nibbles 2-3-4
        let instruction_type =
            get_nibble_from_byte(
                get_byte_from_u16(instruction, 0).unwrap(),
                0).unwrap();
        let X =
            get_nibble_from_byte(
                get_byte_from_u16(instruction, 0).unwrap(),
                1).unwrap() as usize;
        let Y =
            get_nibble_from_byte(
                get_byte_from_u16(instruction, 1).unwrap(),
                0).unwrap() as usize;
        let N =
            get_nibble_from_byte(
                get_byte_from_u16(instruction, 1).unwrap(),
                1).unwrap();
        let NN =
            get_byte_from_u16(instruction, 1).unwrap();
        let NNN =
            ((X as u16) << 8) +
            (NN as u16);

        // TODO: Do I need to manually implement wrapping for instructions that
        //       might cause over/under-flows?
        match instruction_type {
            0x0 => self.display.clear(),
            0x1 => self.program_counter = NNN as usize,
            0x6 => self.register[X] = NN,
            0x7 => self.register[X] += NN,
            0xa => self.index_register = NNN,
            0xd => {
                let index_register = self.index_register as usize;
                let N = N as usize;
                self.display.draw_sprite(
                    &self.ram[index_register..index_register + N],
                    self.register[X] as usize % crate::display::WIDTH,
                    self.register[Y] as usize % crate::display::HEIGHT);
            },
            _ => panic!("Unhandled instruction"),
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            let instruction = self.fetch();
            self.program_counter += 2;

            self.decode(instruction);
        }
    }
}
