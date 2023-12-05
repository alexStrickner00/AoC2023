fn main() {
    let schematic = read_input("input.txt");
    let mut sum = 0;
    for num in &schematic.numbers {
        println!("{:?}", num);
        sum += num.value as u64;
        println!("sum: {sum}");
    }
    let sum_part_nums: u64 = schematic.numbers.iter().map(|n| n.value as u64).sum();
    println!("sum part nums: {sum_part_nums}");
}

#[derive(Debug)]
struct SchematicNumber {
    value: u32,
    row: usize,
    col: usize,
}

impl SchematicNumber {
    fn digits_count(&self) -> usize {
        (self.value as f64).log10().ceil() as usize
    }
}

struct Schematic {
    raw_map: Vec<String>,
    numbers: Vec<SchematicNumber>,
}

impl Schematic {
    fn has_adjacent_symbol(&self, number: &SchematicNumber) -> bool {
        let digit_count = number.digits_count();
        let left_bound = 0.max(number.col as i32 - 1) as usize;
        // check upper row
        if number.row > 0 {
            let row: Vec<char> = self.raw_map[number.row - 1].chars().collect();
            let right_bound = (row.len()).min(number.col + digit_count + 1);
            if row
                .iter()
                .take(right_bound)
                .skip(left_bound)
                .any(|c| is_symbol(*c))
            {
                return true;
            }
        }
        let mut center_chars = self.raw_map[number.row].chars();
        // left
        if number.col > 0 && is_symbol(center_chars.nth(number.col - 1).unwrap()) {
            return true;
        }
        let mut center_chars = center_chars.skip(digit_count);
        // right
        if let Some(c) = center_chars.next() {
            if is_symbol(c) {
                return true;
            }
        }

        //bottom row
        if number.row < self.raw_map.len() - 1 {
            let row: Vec<char> = self.raw_map[number.row + 1].chars().collect();
            let right_bound = (row.len() - 1).min(number.col + digit_count + 1);
            if row
                .iter()
                .take(right_bound)
                .skip(left_bound)
                .any(|c| is_symbol(*c))
            {
                return true;
            }
        }

        false
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn read_input(path: &str) -> Schematic {
    let content = std::fs::read_to_string(path).expect("input file not found");
    let lines = content.lines().map(String::from).collect();
    let mut schematic = Schematic {
        raw_map: lines,
        numbers: Vec::new(),
    };
    let mut temp_nums = Vec::new();
    schematic
        .raw_map
        .iter()
        .enumerate()
        .for_each(|(row, line)| {
            let mut current: Option<SchematicNumber> = None;
            for (col, c) in line.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    current = current.map_or(
                        Some(SchematicNumber {
                            value: digit,
                            row,
                            col,
                        }),
                        |mut current| {
                            current.value = current.value * 10 + digit;
                            Some(current)
                        },
                    )
                } else if let Some(num_to_push) = current {
                    temp_nums.push(num_to_push);
                    current = None;
                }
            }
            if let Some(num_to_push) = current {
                temp_nums.push(num_to_push);
            }
        });
    schematic.numbers = temp_nums
        .into_iter()
        .filter(|n| schematic.has_adjacent_symbol(n))
        .collect();
    schematic
}
