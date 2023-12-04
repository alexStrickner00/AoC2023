use std::thread::current;

fn main() {
    let schematic = read_input("input.txt");
}

struct SchematicNumber {
    value: u32,
    row: u32,
    col: u32,
}

struct Schematic {
    raw_map: Vec<String>,
    numbers: Vec<SchematicNumber>,
}

fn read_input(path: &str) -> Schematic {
    let content = std::fs::read_to_string(path).expect("input file not found");
    let lines = content.lines().map(String::from).collect();
    let schematic = Schematic {
        raw_map: lines,
        numbers: Vec::new(),
    };
    let nums = schematic
        .raw_map
        .iter()
        .enumerate()
        .map(|(row, line)| todo!(""));
    schematic
}
