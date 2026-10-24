use std::{println, usize};
use rand::prelude::*;

const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

pub struct State{
    pc: u16,
    mem: [u8; 4096],
    idx: u16,
    stack: [u16; 16],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    keypad: [bool; 16],
    display: [bool; 64*32]
}

impl State{
    pub fn new() -> Self{
        let mut new_state: State = State{ pc: START_ADDR,
             mem: [0; 4096], 
             idx: 0, 
             stack: [0; 16], 
             sp: 0,
             delay_timer: 0, 
             sound_timer: 0, 
             registers: [0; 16],
             keypad: [false; 16], 
             display: [false; 64*32],
            };

        new_state.mem[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_state
    }

    pub fn stack_push(&mut self, val:u16){
        self.stack[self.sp as usize] = val;
        self.sp+=1;
    }
    
    pub fn stack_pop(&mut self)->u16{
        self.sp -=1;

        println!("{:?}, {}", self.stack, self.sp);
        self.stack[self.sp as usize]
    }

    pub fn get_display(&mut self)->&[bool]{
        &self.display
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keypad[idx] = pressed;
    }

    pub fn load_prog(&mut self, data: &[u8]){
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.mem[0x200 + i] = byte;
            } else {
                break;
            }
        }

    }

    pub fn cycle(&mut self){
        let instr = self.fetch();
        self.execute(instr);

        self.delay_decr();
        self.sound_decr();
    }


    fn delay_decr(&mut self){
        if self.delay_timer > 0{
            self.delay_timer -=1;

        }
    }

    fn sound_decr(&mut self){
        if self.sound_timer > 0{
            println!("Beep");
            self.sound_timer -=1;

        }
    }

    fn fetch(&mut self)->u16{
        let instr1 = self.mem[self.pc as usize] as u16;
        let instr2 = self.mem[(self.pc + 1) as usize] as u16;

        let instr_ret = (instr1 << 8)|instr2;
        self.pc +=2;
        instr_ret
    }

    fn reg_alu(&mut self, reg1: usize, reg2: usize, dig:u8){
        match dig{
            0 => self.registers[reg1] = self.registers[reg2],
            1 => self.registers[reg1] |= self.registers[reg2],
            2 => self.registers[reg1] &= self.registers[reg2],
            3 => self.registers[reg1] ^= self.registers[reg2],
            4 => {
                let (result, of) = self.registers[reg1].overflowing_add(self.registers[reg2]);
                self.registers[0xF] = if of {1} else {0};
                self.registers[reg1] = result;
            },
            5 => {
                let (result, of) = self.registers[reg1].overflowing_sub(self.registers[reg2]);
                self.registers[0xF] = if of {1} else {0};
                self.registers[reg1] = result;
            },
            6 => {
                let carry_out = self.registers[reg1] & 0x01;
                self.registers[0xF] = carry_out;
                self.registers[reg1] = self.registers[reg1] >> 1;
            },
            7 => {
                let (result, of) = self.registers[reg2].overflowing_sub(self.registers[reg1]);
                self.registers[0xF] = if of {1} else {0};
                self.registers[reg1] = result;
            },
            0xE => {
                let carry_out = self.registers[reg1]>>7 & 0x01;
                self.registers[0xF] = carry_out;
                self.registers[reg1] = self.registers[reg1] << 1;
            },
            _ => {}
        }
    }

fn execute(&mut self, instr: u16){

        let digit1 = (instr & 0xF000) >> 12;
        let digit2 = (instr & 0x0F00) >> 8;
        let digit3 = (instr & 0x00F0) >> 4;
        let digit4 = instr & 0x000F;

        match (digit1, digit2, digit3, digit4){
            (0, 0, 0xE, 0) => {
                println!("CLS (Clear Display)");
                self.display = [false; 64*32];
            },
            
            (0, 0, 0xE, 0xE) => { 
                println!("RET (Return from subroutine)");
                let ret_addr = self.stack_pop();
                self.pc = ret_addr;
            },

            (1, _, _, _) => {
                let jump_addr: u16 = (digit2<<8)|(digit3<<4)|digit4;
                println!("JP {:#06X}", jump_addr);
                self.pc = jump_addr;
            },

            (2, _, _, _) => {
                let jump_addr: u16 = (digit2<<8)|(digit3<<4)|digit4;
                println!("CALL {:#06X}", jump_addr);
                self.stack_push(self.pc);
                self.pc = jump_addr;
            },

            (3, _, _, _) => {
                let jump_addr: u16 = self.pc + 2; 
                let target_reg = digit2 as usize;
                let target_val: u8 = ((digit3 as u8) << 4) | digit4 as u8;
                println!("SE V{:X}, {:#04X}", target_reg, target_val);
                if self.registers[target_reg] == target_val {
                    self.pc = jump_addr;
                }
            },

            (4, _, _, _) => {
                let jump_addr: u16 = self.pc + 2; 
                let target_reg = digit2 as usize;
                let target_val: u8 = ((digit3 as u8) << 4) | digit4 as u8;
                println!("SNE V{:X}, {:#04X}", target_reg, target_val);
                if self.registers[target_reg] != target_val {
                    self.pc = jump_addr;
                }
            },

            (5, _, _, _) => {
                let jump_addr: u16 = self.pc + 2; 
                let reg1 = digit2 as usize;
                let reg2: usize = digit3 as usize;
                println!("SE V{:X}, V{:X}", reg1, reg2);
                if self.registers[reg1] == self.registers[reg2] {
                    self.pc = jump_addr;
                }
            },

            (6, _, _, _) => {
                let target_reg = digit2 as usize;
                let target_val: u8 = ((digit3 as u8) << 4) | digit4 as u8;
                println!("LD V{:X}, {:#04X}", target_reg, target_val);
                self.registers[target_reg] = target_val;
            },

            (7, _, _, _) => {
                let target_reg = digit2 as usize;
                let target_val: u8 = ((digit3 as u8) << 4) | digit4 as u8;
                println!("ADD V{:X}, {:#04X}", target_reg, target_val);
                self.registers[target_reg] += target_val;
            },

            (8, _, _, _) => {
                println!("ALU Instruction 8{:X}{:X}{:X}", digit2, digit3, digit4);
                self.reg_alu(digit2 as usize, digit3 as usize, digit4 as u8);
            },

            (9, _, _, _) => {
                let jump_addr: u16 = self.pc + 2; 
                let reg1 = digit2 as usize;
                let reg2: usize = digit3 as usize;
                println!("SNE V{:X}, V{:X}", reg1, reg2);
                if self.registers[reg1] == self.registers[reg2] {
                    self.pc = jump_addr;
                }
            },

            (0xA, _, _, _) => {
                let val = (digit2 << 8) | (digit3 << 4) | digit4;
                println!("LD I, {:#06X}", val);
                self.idx = val;
            },

            (0xB, _, _, _) => {
                let nnn = instr & 0xFFF;
                println!("JP V0, {:#06X}", nnn);
                self.pc = (self.registers[0] as u16) + nnn;
            },

            (0xC, _, _, _) => {
                let reg1 = digit2 as usize;
                let val: u8 = digit3 as u8;
                println!("RND V{:X}, {:#04X}", reg1, val);
                let rand_num = rand::rng().random_range::<u8, _>(..255);
                self.registers[reg1] = rand_num & val;
            },

            (0xD, _, _, _) => {
                println!("DRW V{:X}, V{:X}, {:#X}", digit2, digit3, digit4);
                let mut v_y = (self.registers[digit3 as usize] & 31) as u16;
                let n_rows = digit4;



                self.registers[0xF] = 0;

                for row in 0..n_rows {
                    let byte_addr = self.idx + row;
                    let byte_arr = self.mem[byte_addr as usize];
                    let mut v_x: u16 = (self.registers[digit2 as usize] & 63) as u16;

                    for px in 0..8 {
                        println!("Draw at {v_x}, {v_y}");

                        if (byte_arr & (0b10000000 >> px)) != 0 {
                            if self.display[(v_y * 64 + v_x) as usize] {
                                self.display[(v_y * 64 + v_x) as usize] = false;
                                self.registers[0xF] = 1;
                            }
                            else 
                            {
                            self.display[(v_y * 64 + v_x) as usize] = true;
                            }
                        } 
                        v_x = (v_x + 1) & 63;
                    }
                    v_y = (v_y + 1) & 31;
                }
            },

            (0xE, _, 9, 0xE) => {
                let key = digit2 as usize;
                println!("SKP V{:X}", key);
                println!("{:?} {}", self.keypad, self.registers[key]);
                if self.keypad[self.registers[key] as usize] {
                    self.pc += 2;
                }
            },

            (0xE, _, 0xA, 1) => {
                let key = digit2 as usize;
                println!("SKNP V{:X}", key);
                if !self.keypad[key] {
                    self.pc += 2;
                }
            },
            
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                println!("LD V{:X}, DT", x);
                self.registers[x] = self.delay_timer;
            },

            (0xF, _, 0, 0xA) => {
                let x = digit2;
                println!("LD V{:X}, K", x);
                for i in 0..=16 {
                    if self.keypad[i as usize] {
                        self.registers[x as usize] = i as u8;
                        return;
                    }
                }
                self.pc -= 2;
            },

            (0xF, _, 1, 5) => {
                let x: usize = digit2 as usize;
                println!("LD DT, V{:X}", x);
                self.delay_timer = self.registers[x];
            },

            (0xF, _, 1, 8) => {
                let x: usize = digit2 as usize;
                println!("LD ST, V{:X}", x);
                self.sound_timer = self.registers[x];
            },

            (0xF, _, 1, 0xE) => {
                let x: usize = digit2 as usize;
                println!("ADD I, V{:X}", x);
                self.idx = self.idx.wrapping_add(self.registers[x] as u16);
            },

            (0xF, _, 2, 9) => {
                let x: usize = digit2 as usize;
                println!("LD F, V{:X}", x);
                self.idx = 5 * (self.registers[x] as u16);
            },

            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                println!("LD B, V{:X}", x);
                let vx: f32 = self.registers[x] as f32;
                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;
                self.mem[self.idx as usize] = hundreds;
                self.mem[(self.idx + 1) as usize] = tens;
                self.mem[(self.idx + 2) as usize] = ones;
            },

            (0xF, _, 5, 5) => {
                let x = digit2;
                println!("LD [I], V{:X}", x);
                for reg in 0..=x {
                    self.mem[(self.idx + reg) as usize] = self.registers[reg as usize];
                }
            },

            (0xF, _, 6, 5) => {
                let x = digit2;
                println!("LD V{:X}, [I]", x);
                for reg in 0..=x {
                    self.registers[reg as usize] = self.mem[(self.idx + reg) as usize];
                }
            },

            (_, _, _, _) => {
                println!("Unimplemented instruction: {:#06X}", instr);
                return;
            }
        }
    }

}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state_initialization() {
        let state = State::new();
        assert_eq!(state.pc, START_ADDR);
        assert_eq!(state.sp, 0);
        assert_eq!(state.idx, 0);
        // Verify fontset is correctly copied into memory
        assert_eq!(state.mem[0], 0xF0);
        assert_eq!(state.mem[4], 0xF0);
        assert_eq!(state.mem[5], 0x20);
    }

    #[test]
    fn test_stack_push_and_pop() {
        let mut state = State::new();
        state.stack_push(0x300);
        state.stack_push(0x400);

        assert_eq!(state.sp, 2);
        assert_eq!(state.stack_pop(), 0x400);
        assert_eq!(state.stack_pop(), 0x300);
        assert_eq!(state.sp, 0);
    }

    #[test]
    fn test_register_load_and_add() {
        let mut state = State::new();
        
        // Instruction 0x6123: V1 = 0x23
        state.execute(0x6123);
        assert_eq!(state.registers[1], 0x23);

        // Instruction 0x7105: V1 = V1 + 0x05 -> 0x28
        state.execute(0x7105);
        assert_eq!(state.registers[1], 0x28);
    }

    #[test]
    fn test_alu_add_with_overflow() {
        let mut state = State::new();
        state.registers[1] = 0xFF;
        state.registers[2] = 0x02;

        // Instruction 0x8124: V1 = V1 + V2 (triggers overflow)
        state.execute(0x8124);
        assert_eq!(state.registers[1], 0x01);
        assert_eq!(state.registers[0xF], 1); // Carry flag set
    }

    #[test]
    fn test_jump_call_and_return() {
        let mut state = State::new();

        // Instruction 0x1250: JP 0x250
        state.execute(0x1250);
        assert_eq!(state.pc, 0x250);

        // Instruction 0x2300: CALL 0x300
        state.execute(0x2300);
        assert_eq!(state.pc, 0x300);
        assert_eq!(state.sp, 1);
        assert_eq!(state.stack[0], 0x250);

        // Instruction 0x00EE: RET
        state.execute(0x00EE);
        assert_eq!(state.pc, 0x250);
        assert_eq!(state.sp, 0);
    }

    #[test]
    fn test_timers() {
        let mut state = State::new();
        state.registers[1] = 42;

        // Instruction 0xF115: delay_timer = V1
        state.execute(0xF115);
        assert_eq!(state.delay_timer, 42);

        // Clear V1 and read back delay timer via 0xF107: V1 = delay_timer
        state.registers[1] = 0;
        state.execute(0xF107);
        assert_eq!(state.registers[1], 42);
    }
}