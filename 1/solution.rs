pub mod question1 {
    pub fn solve(data: Vec<String>) -> String {
        // Parse input
        let masses: Vec<i64> = data.iter().map(|x| x.parse().unwrap()).collect();

        // Part 1
        let mut total1: i64 = 0;
        for mass in &masses {
            let mut fuel: i64 = mass / 3 - 2;
            if fuel < 0 {
                fuel = 0;
            }
            total1 += fuel;
        }

        // Part 2
        let mut total2: i64 = 0;
        for mass in &masses {
            let mut cur = *mass;
            loop {
                let fuel = cur / 3 - 2;
                if fuel < 0 {
                    break;
                }
                total2 += fuel;
                cur = fuel;
            }
        }

        println!("Result: {} {}", total1, total2);
        return format!("TODO");
    }
}
