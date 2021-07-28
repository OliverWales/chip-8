extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

const SCREEN_COLS: u32 = 64;
const SCREEN_ROWS: u32 = 32;
const PIXEL_SIZE: u32 = 15;
const WIDTH: u32 = SCREEN_COLS * PIXEL_SIZE;
const HEIGHT: u32 = SCREEN_ROWS * PIXEL_SIZE;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct CpuState {
    pc: u16,
    sp: u8,
    i: u16,
    v: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    screen: [u64; 32],
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("CHIP-8", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    // CPU state
    let mut cpu_state = CpuState {
        pc: 0x200,
        sp: 0,
        i: 0,
        v: [0; 16],
        memory: [0; 4096],
        stack: [0; 16],
        screen: [0; 32],
    };

    // load default font at 0x0050
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
    for (idx, byte) in font.iter().enumerate() {
        cpu_state.memory[0x0050 + idx] = *byte;
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
    for (idx, word) in program.iter().enumerate() {
        cpu_state.memory[0x0200 + 2 * idx] = ((*word & 0xFF00) >> 8) as u8;
        cpu_state.memory[0x0200 + 2 * idx + 1] = (*word & 0x00FF) as u8;
    }

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        // update
        if let Some(_args) = e.update_args() {
            // fetch
            let instruction: u16 = (cpu_state.memory[cpu_state.pc as usize] as u16) << 8
                | cpu_state.memory[cpu_state.pc as usize + 1] as u16;
            cpu_state.pc += 2;

            // decode
            let x = ((instruction & 0x0F00) >> 8) as u8;
            let y = ((instruction & 0x00F0) >> 4) as u8;
            let n = (instruction & 0x000F) as u8;
            let nn = (instruction & 0x00FF) as u8;
            let nnn = instruction & 0x0FFF;

            // execute
            match instruction {
                0x00E0 => {
                    // Clear screen
                    println!("[0x{:04x}] - Clear screen", instruction);
                }
                0x1000..=0x1FFF => {
                    // Jump to NNN
                    println!("[0x{:04x}] - Jump to 0x{:03x}", instruction, nnn);
                    cpu_state.pc = nnn;
                }
                0x2000..=0x2FFF => {
                    // Call NNN
                    println!("[0x{:04x}] - Call 0x{:03x}", instruction, nnn);
                    cpu_state.sp += 1;
                    cpu_state.stack[cpu_state.sp as usize] = cpu_state.pc;
                    cpu_state.pc = nnn;
                }
                0x3000..=0x3FFF => {
                    // Skip next instruction if VX == NN
                    println!(
                        "[0x{:04x}] - Skip next instruction if V{} == {}",
                        instruction, x, nn
                    );

                    if cpu_state.v[x as usize] == nn {
                        cpu_state.pc += 2;
                    }
                }
                0x4000..=0x4FFF => {
                    // Skip next instruction if VX != NN
                    println!(
                        "[0x{:04x}] - Skip next instruction if V{} != {}",
                        instruction, x, nn
                    );

                    if cpu_state.v[x as usize] != nn {
                        cpu_state.pc += 2;
                    }
                }
                0x5000..=0x5FFF => {
                    // Skip next instruction if VX == VY
                    println!(
                        "[0x{:04x}] - Skip next instruction if V{} == V{}",
                        instruction, x, y
                    );

                    if cpu_state.v[x as usize] == cpu_state.v[y as usize] {
                        cpu_state.pc += 2;
                    }
                }
                0x6000..=0x6FFF => {
                    // Set register VX to NN
                    println!("[0x{:04x}] - Set register V{} to {}", instruction, x, nn);
                    cpu_state.v[x as usize] = nn;
                }
                0x7000..=0x7FFF => {
                    // Add NN to register VX
                    println!("[0x{:04x}] - Add {} to register V{}", instruction, nn, x);
                    cpu_state.v[x as usize] += nn;
                }
                0x8000..=0x8FFF => {
                    // TODO: Register ops
                }
                0x9000..=0x9FFF => {
                    // Skip next instruction if VX != VY
                    println!(
                        "[0x{:04x}] - Skip next instruction if V{} != V{}",
                        instruction, x, y
                    );

                    if cpu_state.v[x as usize] != cpu_state.v[y as usize] {
                        cpu_state.pc += 2;
                    }
                }
                0xA000..=0xAFFF => {
                    // Set index register to NNN
                    println!(
                        "[0x{:04x}] - Set index register to 0x{:03x}",
                        instruction, nnn
                    );
                    cpu_state.i = nnn;
                }
                0xD000..=0xDFFF => {
                    // Draw sprite of height N at (V{}, V{})
                    println!(
                        "[0x{:04x}] - Draw sprite at 0x{:03x} of height {} at ({}, {})",
                        instruction,
                        cpu_state.i,
                        n,
                        cpu_state.v[x as usize],
                        cpu_state.v[y as usize]
                    );

                    for h in 0..n {
                        cpu_state.screen[(h + cpu_state.v[y as usize]) as usize] |=
                            (cpu_state.memory[(cpu_state.i + h as u16) as usize] as u64)
                                << 63 - cpu_state.v[x as usize];
                        // TODO: set flag
                    }
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

        // render
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |_c, gl| {
                graphics::clear(BLACK, gl);
            });

            for row in 0..SCREEN_ROWS {
                let line = cpu_state.screen[row as usize];

                for col in 0..SCREEN_COLS {
                    if line >> (63 - col) & 1 == 1 {
                        let pixel = graphics::rectangle::square(
                            (col * PIXEL_SIZE) as f64,
                            (row * PIXEL_SIZE) as f64,
                            PIXEL_SIZE as f64,
                        );

                        gl.draw(args.viewport(), |c, gl| {
                            let transform = c.transform;
                            graphics::rectangle(WHITE, pixel, transform, gl);
                        });
                    }
                }
            }
        }
    }
}
