use std::println;


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
        let temp_sp = self.sp;
        self.sp -=1;
        self.stack[temp_sp as usize]
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
        let instr2 = self.mem[self.pc as usize] as u16;

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
            (0, 0, 0xE, 0) => self.display = [false; 64*32], //Clear Display
            (0, 0, 0xE, 0xE) =>{ //Pop and ret
                let ret_addr = self.stack_pop();
                self.pc = ret_addr;
            },

            (1, _, _, _) => {
                let jump_addr: u16 = (digit2<<8)|(digit3<<4)|digit4; //Jump
                self.pc = jump_addr;
            },

            (2, _, _, _) => {
                let jump_addr: u16 = (digit2<<8)|(digit3<<4)|digit4; //Call
                self.stack_push(self.pc);
                self.pc = jump_addr;
            },

            (3, _, _, _) => {
                let jump_addr: u16 = self.pc + 4; 
                let target_reg = digit2 as usize;
                let target_val: u8 = ((digit3 as u8) <<4)|digit4 as u8;
                if self.registers[target_reg] == target_val{
                    self.pc = jump_addr;

                }
            },

            (4, _, _, _) => {
                let jump_addr: u16 = self.pc + 4; 
                let target_reg = digit2 as usize;
                let target_val: u8 = ((digit3 as u8) <<4)|digit4 as u8;
                if self.registers[target_reg] != target_val{
                    self.pc = jump_addr;

                }
            },

            (5, _, _, _) => {
                let jump_addr: u16 = self.pc + 4; 
                let reg1 = digit2 as usize;
                let reg2: usize = digit3 as usize;
                if self.registers[reg1] == self.registers[reg2]{
                    self.pc = jump_addr;
                }
            },

            (6, _, _, _)=>{
                let target_reg = digit2 as usize; //R1 <- Val
                let target_val: u8 = ((digit3 as u8) <<4)|digit4 as u8;
                self.registers[target_reg] = target_val;
            },

            (7, _, _, _)=>{
                let target_reg = digit2 as usize; // R1 <- R1 + Val
                let target_val: u8 = ((digit3 as u8) <<4)|digit4 as u8;
                self.registers[target_reg] += target_val;
            },

            (8, _, _, _)=>self.reg_alu(digit2 as usize, digit3 as usize, digit4 as u8),
        

            (9, _, _, _) => {
                let jump_addr: u16 = self.pc + 4; 
                let reg1 = digit2 as usize;
                let reg2: usize = digit3 as usize;
                if self.registers[reg1] == self.registers[reg2]{
                    self.pc = jump_addr;
                }
            },

            (0xA, _, _, _) => self.idx = (digit2<<8)|(digit3<<4)|digit4, // I <- Val

            (0xB, _, _, _) => {
                let nnn = instr & 0xFFF;
                self.pc = (self.registers[0] as u16) + nnn;
            },
            
            (_, _, _, _) => {
                println!("Unimplemented");
                return
            }
        }
    }


}   