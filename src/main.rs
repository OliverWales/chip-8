use std::convert::TryInto;

fn main() {
    // while not halted
    // fetch
    let instruction: u16 = 0x00E0;

    // decode
    let x: u8 = ((instruction & 0x0F00) >> 8).try_into().unwrap();
    let y: u8 = ((instruction & 0x00F0) >> 4).try_into().unwrap();
    let n: u8 = (instruction & 0x000F).try_into().unwrap();
    let nn: u8 = (instruction & 0x00FF).try_into().unwrap();
    let nnn: u16 = instruction & 0x0FFF;

    // execute
    match instruction {
        0x00E0 => {
            // Clear screen]
            println!("Clear screen");
        }
        0x1000..=0x1FFF => {
            // Jump to NNN
            println!("Jump to {}", nnn);
        }
        0x6000..=0x6FFF => {
            // Set register VX to NN
            println!("Set register V{} to {}", x, nn);
        }
        0x7000..=0x7FFF => {
            // Add NN to register VX
            println!("Add {} to register V{}", nn, x);
        }
        0xA000..=0xAFFF => {
            // Set index register to NNN
            println!("Set index register to {}", nnn);
        }
        0xD000..=0xDFFF => {
            // Draw sprite of height N at (V{}, V{})
            println!("Draw sprite at I of height {} at (V{}, V{})", n, x, y);
        }
        _ => {
            // Instruction not yet implemented
            println!("Instruction not yet implemented");
        }
    }
}
