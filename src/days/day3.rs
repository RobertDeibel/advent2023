struct PlacedValue<T> {
    x: usize,
    x_end: Option<usize>,
    y: usize,
    y_end: Option<usize>,
    value: T,
}

impl<T> PlacedValue<T> {
    fn new(x: usize, x_end:  Option<usize>, y: usize, y_end:  Option<usize>, value: T) -> PlacedValue<T> {
        PlacedValue { x, x_end, y, y_end, value}
    }

}

pub fn run(args: Vec<String>) -> String {
    let lines = super::read_lines(&args[0]);

    // build vector of locations in grid
    let mut symbol_indices: Vec<PlacedValue<char>> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let nums = super::day1::nums_in_str(line);
        

        for (j, char) in line.chars().enumerate() {
            // symbol of interest is not numbers and not '.'
            if !char::is_ascii_digit(&char) && char != '.' {
                symbol_indices.push(PlacedValue::new(j, None, i, None, char));
            }
        }
    }

    // handle symbol locations and search for numbers
    super::make_output(&[])
}

// combines adjacent element that are described by a index vector
pub fn combine_adjacent<T>(values: (Vec<T>, Vec<usize>)) -> (Vec<T>, Vec<usize>, Vec<usize>) {
    for (i, &idx) in (&values.1).iter().enumerate() {

    }

    (values.0, values.1, values.1)
}
