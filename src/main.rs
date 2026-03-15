mod core;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

fn main() {
    let mut cpu = core::chip8::Chip8::new();
    cpu.load_rom("roms/test_opcode.ch8");

    let mut window = Window::new(
        "CHIP-8",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .unwrap();

    // 60hz
    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; WIDTH * SCALE * HEIGHT * SCALE];

    let mut frames_this_second = 0;
    let mut last_second = std::time::Instant::now();

    let keys = [
        Key::X,    // 0
        Key::Key1, // 1
        Key::Key2, // 2
        Key::Key3, // 3
        Key::Q,    // 4
        Key::W,    // 5
        Key::E,    // 6
        Key::A,    // 7
        Key::S,    // 8
        Key::D,    // 9
        Key::Z,    // A
        Key::C,    // B
        Key::Key4, // C
        Key::R,    // D
        Key::F,    // E
        Key::V,    // F
    ];

    let sound = core::sound::Sound::new();

    // while the window is opened and esc isn't pressed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, key) in keys.iter().enumerate() {
            cpu.keypad[i] = window.is_key_down(*key);
        }

        for _ in 0..10 {
            cpu.cycle();
        }

        frames_this_second += 1;

        if last_second.elapsed().as_secs() >= 1 {
            window.set_title(&format!("CHIP-8 ({} FPS)", frames_this_second));
            frames_this_second = 0;
            last_second = std::time::Instant::now();
        }

        // update timers
        if cpu.sound_timer > 0 {
            sound.beep();
            cpu.sound_timer -= 1;
        }

        if cpu.delay_timer > 0 {
            cpu.delay_timer -= 1;
        }

        // draw display to buffer
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = cpu.display[y * WIDTH + x];
                let color = if pixel { 0xFFFFFF } else { 0x000000 };

                // scale each pixel up
                for sy in 0..SCALE {
                    for sx in 0..SCALE {
                        buffer[(y * SCALE + sy) * WIDTH * SCALE + (x * SCALE + sx)] = color;
                    }
                }
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE)
            .unwrap();
    }

    println!("CHIP-8 initialized, PC: {:#X}", cpu.pc);
}
