mod question8 {
    pub fn solve(data: Vec<String>) -> String {
        // Picture consts
        const WIDTH: i32 = 25;
        const HEIGHT: i32 = 6;
        const LAYER_PIXELS: usize = (WIDTH * HEIGHT) as usize;

        // Parse input
        let raw_ints: Vec<_> = data[0].chars().map(|x| x.to_digit(10).unwrap()).collect();

        // Part 1: Find layer with fewest 0s.
        let mut lowest: i64 = std::i64::MAX;
        let mut lowest_score = 0;
        for layer in raw_ints.chunks(LAYER_PIXELS) {
            let zcount = layer.iter().filter(|&x| *x == 0).count();
            // Check if lower than current lowest
            if (zcount as i64) < lowest {
                lowest = zcount as i64;
                // Calculate score (1 count * 2 count)
                lowest_score = layer.iter().filter(|&x| *x == 1).count()
                    * layer.iter().filter(|&x| *x == 2).count();
            }
        }

        // Part 2: Render image from layer data
        // Merge layers
        let mut merged: [_; LAYER_PIXELS] = [0; LAYER_PIXELS];
        for i in 0..LAYER_PIXELS {
            let mut cur_pixel = 2;
            for layer in raw_ints.chunks(LAYER_PIXELS) {
                let l_pixel = layer[i];
                if cur_pixel == 2 {
                    cur_pixel = l_pixel;
                }
            }
            merged[i] = cur_pixel;
        }
        // Print image as chars
        let mut img = String::with_capacity(LAYER_PIXELS + (HEIGHT as usize));
        for (i, pixel) in merged.iter().enumerate() {
            let c = match pixel {
                0 => '█',
                1 => '░',
                2 => ' ',
                _ => unreachable!(),
            };
            img.push(c);
            if (i as i32) % WIDTH == WIDTH - 1 {
                img.push('\n');
            }
        }

        // Return solutions
        return format!("{} and Image:\n{}", lowest_score, img);
    }
}
