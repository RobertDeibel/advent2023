use std::collections::HashMap;

struct PlacedValue<T> {
    x: usize,
    y: usize,
    len: usize,
    value: T,
}

impl<T> PlacedValue<T> {
    fn new(x: usize, y: usize, len: usize, value: T) -> PlacedValue<T> {
        PlacedValue { x, y, len, value}
    }
}

pub fn run(args: Vec<String>) -> String {
    let lines = super::read_lines(&args[0]);

    let mut numbers = HashMap::new(); // this could be an array
    // build map of number locations in grid
    let mut symbols: Vec<PlacedValue<char>> = vec![];
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            // symbol of interest is not numbers and not '.'
            if !char::is_ascii_digit(&char) && char != '.' {
                symbols.push(PlacedValue::new(j, i, 1, char));
            }

        }
        // get numbers and split into contained vectors
        let all_numbers = super::day1::nums_in_str(&line);
        let all_numbers = split_non_adjacent(all_numbers);
        // store numbers by their indices
        let mut inner_map = HashMap::new();
        for (number, indices) in all_numbers {
            // build build full number from vector
            let number = super::day2::reduce_to_num(number);
            for &idx in indices.iter() {
                inner_map.insert(idx, PlacedValue::new(indices[0], i, indices.len(), number));
            }
        }
        numbers.insert(i,inner_map);
    }

    let mut sum = 0;
    let mut  gear_sum = 0;
    for symbol in symbols {
        let mut found: Vec<i32> = vec![];
        let offsets = [-1, 0, 1];
        for y_offset in offsets {
            let query_line = symbol.y as i32 + y_offset;
            if query_line < 0 || query_line == (lines.len() as i32){
                continue;
            }

            let entry = numbers.get_mut(&(query_line as usize))
                .expect("No Line found?");
            for x_offset in offsets {
                let hit = entry.remove(&((symbol.x as i32 + x_offset) as usize));

                let number = match &hit {
                    Some(value) => {
                        for others in value.x..(value.x+value.len) {
                            entry.remove(&others);
                        }
                        if symbol.value == '*' {
                            found.push(value.value);
                        }
                        value.value
                    },
                    None => 0,
                };
                sum += number;
            }
        }
        if found.len() == 2 {
            gear_sum += found[0] * found[1];
        }
    }

    let result1 = format!("{}", sum);
    let result2 = format!("{}", gear_sum);

    // handle symbol locations and search for numbers
    super::make_output(&[result1, result2])
}


// Splits value and index vectors when not adjacent
pub fn split_non_adjacent<T: Copy>(values: (Vec<T>, Vec<usize>)) -> Vec<(Vec<T>, Vec<usize>)> {
    let mut split = vec![(vec![], vec![])];
    let mut adjacent: (Vec<T>, Vec<usize>) = (vec![], vec![]);
    let mut last = 0;
    for (&element, &index) in (&values.0).iter().zip(&values.1) {
        if index - last > 1 && adjacent.0.len() > 0 {
            split.push(adjacent);
            adjacent = (vec![], vec![]);
        }
        last = index;
        adjacent.0.push(element);
        adjacent.1.push(index);
    }
    split.push(adjacent);

    split
}
