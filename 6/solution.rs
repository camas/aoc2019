pub mod question6 {
    use std::collections::{HashMap, HashSet};

    pub fn solve(data: Vec<String>) -> String {
        // Parse input
        let mut nodes: HashSet<_> = HashSet::new();
        let mut orbits: HashMap<_, _> = HashMap::new();
        for line in &data {
            let mut line_split = line.split(')');
            let n1 = line_split.next().unwrap();
            let n2 = line_split.next().unwrap();

            nodes.insert(n1);
            nodes.insert(n2);

            orbits.insert(n2, n1);
        }

        // Part 1: Count total orbits
        let mut total_orbits: i64 = 0;
        for node in &nodes {
            let mut cur = node;
            loop {
                if !orbits.contains_key(cur) {
                    break;
                }
                total_orbits += 1;
                cur = &orbits[cur];
            }
        }

        // Part 2: Find shortest path between YOU and SAN

        // Returns a vector of orbits from start mass to COM
        fn walk<'a>(start: &'a str, orbits: &'a HashMap<&str, &str>) -> Vec<&'a str> {
            let mut chain: Vec<_> = Vec::new();
            let mut cur = start;
            loop {
                if !orbits.contains_key(cur) {
                    break;
                }
                cur = &orbits[cur];
                chain.push(cur);
            }

            return chain;
        }
        let w1 = walk("YOU", &orbits);
        let w2 = walk("SAN", &orbits);

        let mut walk_dist = 0;
        for mass in &w1 {
            if w2.contains(mass) {
                let w1_dist = w1.iter().position(|&x| &x == mass).unwrap();
                let w2_dist = w2.iter().position(|&x| &x == mass).unwrap();
                walk_dist = w1_dist + w2_dist;
                break;
            }
        }

        // Return solutions
        return format!("{} {}", total_orbits, walk_dist);
    }
}
