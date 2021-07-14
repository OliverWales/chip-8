fn main() {
    // CPU state
    let mut pc: u16 = 0x200;
    let mut sp: u8 = 0;
    let mut i: u16 = 0;
    let mut v: [u8; 16] = [0; 16];
    let mut memory: [u8; 4096] = [0; 4096];
    let mut stack: [u16; 16] = [0; 16];
    let mut screen: [u64; 32] = [0; 32];

    // load font
    let font: [u8; 80] = [
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
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
    for (i, byte) in font.iter().enumerate() {
        memory[0x0050 + i] = *byte;
    }

    // load program (IBM logo)
    let program: [u16; 66] = [
        0x00E0, 0xA22A, 0x600C, 0x6108, 0xD01F, 0x7009, 0xA239, 0xD01F, 0xA248, 0x7008, 0xD01F,
        0x7004, 0xA257, 0xD01F, 0x7008, 0xA266, 0xD01F, 0x7008, 0xA275, 0xD01F, 0x1228, 0xFF00,
        0xFF00, 0x3C00, 0x3C00, 0x3C00, 0x3C00, 0xFF00, 0xFFFF, 0x00FF, 0x0038, 0x003F, 0x003F,
        0x0038, 0x00FF, 0x00FF, 0x8000, 0xE000, 0xE000, 0x8000, 0x8000, 0xE000, 0xE000, 0x80F8,
        0x00FC, 0x003E, 0x003F, 0x003B, 0x0039, 0x00F8, 0x00F8, 0x0300, 0x0700, 0x0F00, 0xBF00,
        0xFB00, 0xF300, 0xE300, 0x43E0, 0x00E0, 0x0080, 0x0080, 0x0080, 0x0080, 0x00E0, 0x00E0,
    ];
    for (i, word) in program.iter().enumerate() {
        memory[0x0200 + 2 * i] = ((*word & 0xFF00) >> 8) as u8;
        memory[0x0200 + 2 * i + 1] = (*word & 0x00FF) as u8;
    }

    loop {
        // fetch
        let instruction: u16 = (memory[pc as usize] as u16) << 8 | memory[pc as usize + 1] as u16;
        pc += 2;

        // decode
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        // execute
        match instruction {
            0x0000 => {
                println!("[0x{:04x}] - Exit", instruction);
                break;
            }
            0x00E0 => {
                // Clear screen
                println!("[0x{:04x}] - Clear screen", instruction);
                draw_screen(&screen);
            }
            0x1000..=0x1FFF => {
                // Jump to NNN
                println!("[0x{:04x}] - Jump to 0x{:03x}", instruction, nnn);
                pc = nnn;

                if pc == nnn {
                    println!("Self-jump detected, exiting");
                    break;
                }
            }
            0x6000..=0x6FFF => {
                // Set register Vx to NN
                println!("[0x{:04x}] - Set register V{} to {}", instruction, x, nn);
                v[x as usize] = nn;
            }
            0x7000..=0x7FFF => {
                // Add NN to register Vx
                println!("[0x{:04x}] - Add {} to register V{}", instruction, nn, x);
                v[x as usize] += nn;
            }
            0xA000..=0xAFFF => {
                // Set index register to NNN
                println!(
                    "[0x{:04x}] - Set index register to 0x{:03x}",
                    instruction, nnn
                );
                i = nnn;
            }
            0xD000..=0xDFFF => {
                // Draw sprite of height N at (V{}, V{})
                println!(
                    "[0x{:04x}] - Draw sprite at 0x{:03x} of height {} at ({}, {})",
                    instruction, i, n, v[x as usize], v[y as usize]
                );

                for h in 0..n {
                    screen[(h + v[y as usize]) as usize] |=
                        (memory[(i + h as u16) as usize] as u64) << 63 - v[x as usize];
                    // TODO: set flag
                }
                draw_screen(&screen);
            }
            _ => {
                // Instruction not yet implemented
                println!(
                    "[0x{:04x}] - Instruction 0x{:04x} not implemented",
                    instruction, instruction
                );
            }
        }
    }
}

fn draw_screen(screen: &[u64; 32]) {
    for line in screen {
        for bit in 0..64 {
            if line >> (63 - bit) & 1 == 1 {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
