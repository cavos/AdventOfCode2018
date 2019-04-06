pub fn solve() {
    let serial_number = 3031;

    // compute grid power levels
    let mut grid = [[0; 301]; 301];
    for y in 1..301 {
        for x in 1..301 {
            let rack_id = x + 10;
            let mut power_lever: i32 = (rack_id * y + serial_number) * rack_id;
            power_lever = (power_lever % 1000) / 100;
            grid[y as usize][x as usize] = power_lever - 5;
        }
    }

    {
        // find largest total power (part1)
        let mut max_power = -999i32;
        let mut coords = (0 as usize, 0 as usize);
        for y in 1..299 {
            for x in 1..299 {
                let mut power = 0;
                for y_off in 0..3 {
                    for x_off in 0..3 {
                        power += grid[y + y_off][x + x_off];
                    }
                }

                if power > max_power {
                    max_power = power;
                    coords = (x, y);
                }
            }
        }
        println!(
            "Day 11 part 1: max power level {} at [{:?}]",
            max_power, coords
        );
    }

    {
        let mut max_power = -999;
        let mut coords: (usize, usize) = (0, 0);
        let mut best_size = 1;
        let mut size = 2;
        let mut cache = grid.clone();
        //   1 2 3
        // 1 c c x
        // 2 c c x
        // 3 y y y
        while size <= 300 {
            for y in 1..300 {
                for x in 1..300 {
                    if y + size <= 300 && x + size <= 300 {
                        let mut power_level = cache[y][x]; // c
                        for y_off in 0..(size - 1) {
                            power_level += grid[y + y_off][x + size - 1]; // x
                        }
                        for x_off in 0..size {
                            power_level += grid[y + size - 1][x + x_off]; // y
                        }

                        if power_level > max_power {
                            max_power = power_level;
                            coords = (x, y);
                            best_size = size;
                        }

                        cache[y][x] = power_level;
                    }
                }
            }

            size += 1;
        }

        println!(
            "Day 11 part 2: max power level {} at [{:?}] with size {}",
            max_power, coords, best_size
        );
    }
}
