use nes_emulator::Rom;
use nes_emulator::CPU;

fn main() {
    //load the game
    let bytes = std::fs::read("roms/nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let mut cpu = CPU::new(rom);

    // run the game cycle
    cpu.run_with_callback(move |cpu| {
        let trace = cpu.trace();
        println!("{trace}");
    });
}
