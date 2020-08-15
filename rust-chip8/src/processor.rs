use crate::memory::*;
use crate::registers::*;
use crate::display::*;
use crate::keyboard::*;

pub struct Processor;
enum Action {
        Next,
        Skip,
        Jump(u16)
}

impl Processor  {


    pub fn create() {
        let mut m = Memory::new();
        let mut r = Register::new();
        let mut d = Display::new();
        let mut k = Keyboard::new();
    }

    //Jump to a machine code routine at nnn.
    //This instruction is only used on the old computers on which Chip-8 was originally implemented. 
    //It is ignored by modern interpreters.
    fn i0nnn() {}

    //Clear the display.
    pub fn i00e0(mut a: Display) {
        for j in 0..32 {
            for i in 0..64 {
                a.vram[i][j] = 0x0;
            }
        }
    }

    //Return from a subroutine.
    //The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn i00ee() {}

    //Jump to location nnn.
    //The interpreter sets the program counter to nnn.
    fn i1nnn(mut r: Register, nnn: u16) {
        Action::Jump(nnn);
        r.pc = nnn;
    }

    //Call subroutine at nnn.
    //The interpreter increments the stack pointer, then puts the current PC on the top of the stack. 
    //The PC is then set to nnn.
    fn i2nnn() {

    }

    //skip next instruction if Vx = kk.
    //The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn i3xkk(r: Register, x: u8, kk: u8) {
        
    }

    //Skip next instruction if Vx != kk.
    //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn i4xkk() {}

    //Skip next instruction if Vx = Vy.
    //The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn i5xy0() {}

    //Set Vx = kk.
    //The interpreter puts the value kk into register Vx.
    fn i6xkk() {}

    //Set Vx = kk.
    //The interpreter puts the value kk into register Vx.
    fn i7xkk() {}

    //Set Vx = Vy.
    //Stores the value of register Vy in register Vx.
    fn i8xy0() {}

    //Set Vx = Vx OR Vy.
    //Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. 
    //A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then 
    //the same bit in the result is also 1. Otherwise, it is 0.
    fn i8xy1() {}

    //Set Vx = Vx AND Vy.
    //Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. 
    //A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, 
    //then the same bit in the result is also 1. Otherwise, it is 0.
    fn i8xy2() {}

    //Set Vx = Vx XOR Vy.
    //Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    //An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, 
    //then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn i8xy3() {}

    //Set Vx = Vx + Vy, set VF = carry.
    //The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) 
    //VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn i8xy4() {}

    //Set Vx = Vx - Vy, set VF = NOT borrow.
    //If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn i8xy5() {}

    //Set Vx = Vx SHR 1.
    //If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn i8xy6() {}

    //Set Vx = Vy - Vx, set VF = NOT borrow.
    //If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn i8xy7() {}

    //Set Vx = Vx SHL 1.
    //If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn i8xyE() {}

    //Skip next instruction if Vx != Vy.
    //The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn i9xy0() {}

    //Set I = nnn.
    //The value of register I is set to nnn.
    fn iAnnn() {}

    //Jump to location nnn + V0.
    //The program counter is set to nnn plus the value of V0.
    fn iBnnn() {}

    //Set Vx = random byte AND kk.
    //The interpreter generates a random number from 0 to 255, 
    //which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn iCxkk() {}

    //Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    //The interpreter reads n bytes from memory, starting at the address stored in I. 
    //These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). 
    //Sprites are XORed onto the existing screen. If this causes any pixels to be erased, 
    //VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the 
    //coordinates of the display, it wraps around to the opposite side of the screen. 
    //See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    fn iDxyn() {}

    //Skip next instruction if key with the value of Vx is pressed.
    //Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, 
    //PC is increased by 2.
    fn iEx9E() {}

    //Skip next instruction if key with the value of Vx is not pressed.
    //Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    fn iExA1() {}

    //Set Vx = delay timer value.
    //The value of DT is placed into Vx.
    fn iFx07() {}

    //Wait for a key press, store the value of the key in Vx.
    //All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn iFx0A() {}

    //Set delay timer = Vx.
    //DT is set equal to the value of Vx.
    fn iFx15() {}

    //Set sound timer = Vx.
    //ST is set equal to the value of Vx.
    fn iFx18() {}

    //Set I = I + Vx.
    //The values of I and Vx are added, and the results are stored in I
    fn iFx1E() {}

    //Set I = location of sprite for digit Vx.
    //The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. 
    //See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    fn iFx29() {}

    //Store BCD representation of Vx in memory locations I, I+1, and I+2.
    //The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, 
    //the tens digit at location I+1, and the ones digit at location I+2.
    fn iFx33() {}

    //Store registers V0 through Vx in memory starting at location I.
    //The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn iFx55() {}

    //Read registers V0 through Vx from memory starting at location I.
    //The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn iFx65() {}
}