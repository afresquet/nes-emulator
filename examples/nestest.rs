use nes_emulator::Rom;
use nes_emulator::CPU;

fn main() {
    //load the game
    let bytes = std::fs::read("roms/nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let mut cpu = CPU::new(rom);
    cpu.program_counter = 0xC000;

    let expected_logs = std::fs::read_to_string("nestest.log").unwrap();
    let mut expected_lines = expected_logs.lines();

    let mut i = 0;

    // run the game cycle
    cpu.run_with_callback(move |cpu| {
        let trace = cpu.trace();
        assert_eq!(
            trace.to_string(),
            expected_lines
                .next()
                .expect("log ended but execution didnt"),
            "line {i} didn't match [left == actual | right == expected]"
        );
        println!("{trace}");
        i += 1;
    });

    if expected_logs.len() != i {
        panic!("execution ended before logs did");
    }
}
