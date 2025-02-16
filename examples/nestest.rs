use nes_emulator::Rom;
use nes_emulator::CPU;

fn main() {
    //load the game
    let rom = Rom::from_file("roms/nestest.nes").unwrap();
    let mut cpu = CPU::new(rom);

    // run the game cycle
    cpu.run_with_callback(move |cpu, _| {
        let trace = cpu.trace();
        println!("{trace}");
    });
}
