mod question12 {
    use num_integer::Integer;
    use std::collections::HashSet;
    use std::ops;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Vector {
        x: i32,
        y: i32,
        z: i32,
    }

    impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector {
        Vector {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    });
    impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector {
        Vector {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    });

    impl Vector {
        fn new() -> Vector {
            Vector { x: 0, y: 0, z: 0 }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Body {
        position: Vector,
        velocity: Vector,
    }

    impl Body {
        fn get_energy(&self) -> i32 {
            (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
                * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
        }
    }

    struct Space {
        bodies: Vec<Body>,
    }

    impl Space {
        fn from_input(data: &[&str]) -> Space {
            let mut space = Space { bodies: vec![] };
            for line in data {
                let xyz_strings: Vec<_> = line[1..line.len() - 1].split(',').collect();
                let x: i32 = xyz_strings[0].split('=').nth(1).unwrap().parse().unwrap();
                let y: i32 = xyz_strings[1].split('=').nth(1).unwrap().parse().unwrap();
                let z: i32 = xyz_strings[2].split('=').nth(1).unwrap().parse().unwrap();
                let body = Body {
                    position: Vector { x: x, y: y, z: z },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                };
                space.bodies.push(body);
            }
            space
        }

        fn step(&mut self) {
            // First apply gravity
            // Setup diff store
            let mut diffs = Vec::with_capacity(self.bodies.len());
            for _ in 0..self.bodies.len() {
                diffs.push(Vector::new());
            }
            // Get every pair of bodies
            for i in 0..self.bodies.len() {
                for j in i + 1..self.bodies.len() {
                    // Get position difference
                    let body1 = &self.bodies[i];
                    let body2 = &self.bodies[j];
                    let diff = &body2.position - &body1.position;

                    // Store changed velocity so not writing to bodies while looping
                    if diff.x < 0 {
                        diffs[i].x -= 1;
                        diffs[j].x += 1;
                    } else if diff.x > 0 {
                        diffs[i].x += 1;
                        diffs[j].x -= 1;
                    }
                    if diff.y < 0 {
                        diffs[i].y -= 1;
                        diffs[j].y += 1;
                    } else if diff.y > 0 {
                        diffs[i].y += 1;
                        diffs[j].y -= 1;
                    }
                    if diff.z < 0 {
                        diffs[i].z -= 1;
                        diffs[j].z += 1;
                    } else if diff.z > 0 {
                        diffs[i].z += 1;
                        diffs[j].z -= 1;
                    }
                }
            }
            // Update from diffs
            for (i, diff) in diffs.iter().enumerate() {
                self.bodies[i].velocity = &self.bodies[i].velocity + diff;
            }

            // Apply velocity
            for body in &mut self.bodies {
                body.position = &body.position + &body.velocity;
            }
        }

        fn steps(&mut self, count: i32) {
            for _ in 0..count {
                self.step();
            }
        }

        fn get_total_energy(&self) -> i32 {
            let mut total = 0;
            for body in &self.bodies {
                total += body.get_energy();
            }
            total
        }

        fn calc_loop(&mut self) -> u64 {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut x_hashes = HashSet::new();
            let mut y_hashes = HashSet::new();
            let mut z_hashes = HashSet::new();
            let mut x_count: u64 = 0;
            let mut y_count = 0;
            let mut z_count = 0;
            let mut count = 0;
            loop {
                let mut x_hasher = DefaultHasher::new();
                let mut y_hasher = DefaultHasher::new();
                let mut z_hasher = DefaultHasher::new();
                for body in &self.bodies {
                    body.position.x.hash(&mut x_hasher);
                    body.velocity.x.hash(&mut x_hasher);
                    body.position.y.hash(&mut y_hasher);
                    body.velocity.y.hash(&mut y_hasher);
                    body.position.z.hash(&mut z_hasher);
                    body.velocity.z.hash(&mut z_hasher);
                }
                let x_hash = x_hasher.finish();
                let y_hash = y_hasher.finish();
                let z_hash = z_hasher.finish();
                let x_new = x_hashes.insert(x_hash);
                let y_new = y_hashes.insert(y_hash);
                let z_new = z_hashes.insert(z_hash);
                if !x_new && x_count == 0 {
                    x_count = count;
                }
                if !y_new && y_count == 0 {
                    y_count = count;
                }
                if !z_new && z_count == 0 {
                    z_count = count;
                }
                if !x_new && !y_new && !z_new {
                    break;
                }
                self.step();
                count += 1;
            }
            let result: u64 = x_count.lcm(&y_count).lcm(&z_count);
            result
        }
    }

    pub fn solve(data: Vec<&str>) -> String {
        let mut space = Space::from_input(&data);
        space.steps(1000);
        let part1 = space.get_total_energy();
        let mut space = Space::from_input(&data);
        let part2 = space.calc_loop();
        format!("{} {}", part1, part2)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_data_parse() {
            let data = vec!["<x=-100, y=1412, z=24>"];
            let space = Space::from_input(&data);
            let body = &space.bodies[0];
            assert_eq!(body.position.x, -100);
            assert_eq!(body.position.y, 1412);
            assert_eq!(body.position.z, 24);
        }

        #[test]
        fn test_known_answers() {
            let data = vec![
                "<x=-1, y=0, z=2>",
                "<x=2, y=-10, z=-7>",
                "<x=4, y=-8, z=8>",
                "<x=3, y=5, z=-1>",
            ];
            let mut space = Space::from_input(&data);
            space.steps(10);
            assert_eq!(space.get_total_energy(), 179);

            let mut space = Space::from_input(&data);
            assert_eq!(space.calc_loop(), 2772);

            let data = vec![
                "<x=-8, y=-10, z=0>",
                "<x=5, y=5, z=10>",
                "<x=2, y=-7, z=3>",
                "<x=9, y=-8, z=-3>",
            ];
            let mut space = Space::from_input(&data);
            space.steps(100);
            assert_eq!(space.get_total_energy(), 1940);

            let mut space = Space::from_input(&data);
            assert_eq!(space.calc_loop(), 4_686_774_924);
        }
    }
}
