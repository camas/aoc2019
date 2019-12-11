mod question9 {
    use crate::common::machine::Machine;
    pub fn solve(data: Vec<&str>) -> String {
        // Parse input
        let mem: Vec<_> = data[0].split(',').map(|x| x.parse().unwrap()).collect();

        // Part 1: Run boost program in test mode
        let mut machine = Machine::new(&mem);
        let mut output = Vec::new();
        machine.run(|| 1, |x| output.push(x));
        let boost_keycode = output.pop().unwrap();

        // Part 2: Run boost program in sensor mode
        let mut machine = Machine::new(&mem);
        let mut output = Vec::new();
        machine.run(|| 2, |x| output.push(x));
        let coords = output.pop().unwrap();

        // Return solutions
        return format!("{} {}", boost_keycode, coords);
    }
}
