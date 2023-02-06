use crate::days::*;

fn increase_cycle(cycle: &mut i32, register: &i32, signal_strength: &mut i32) {
    *cycle += 1;
    if *cycle == 20 || (*cycle - 20) % 40 == 0 {
        *signal_strength += *register * *cycle;
        println!(
            "cycle: {}, register: {}, signal: {}, total: {}",
            cycle,
            register,
            *register * *cycle,
            signal_strength
        );
    }
}

fn draw_pixel(crt: &mut [char], cursor: &mut usize, sprite: &i32) {
    let local_cursor = (*cursor % 40) as i32;
    crt[*cursor] = if (*sprite..*sprite + 3).contains(&local_cursor) {
        '#'
    } else {
        '.'
    };
    *cursor += 1;
}

fn visualize_crt(crt: &[char; 240]) -> String {
    let mut s = "\n".to_string();
    for y in 0..6 {
        let row = &crt[40 * y..40 * (y + 1)];
        s.push_str(&format!("{}\n", row.iter().collect::<String>()));
    }
    s
}

impl Puzzle for Day10 {
    fn part_one(&self, data: &'static str) -> String {
        let mut cycle = 0;
        let mut register = 1;

        let signal_strength = data.lines().fold(0, |mut signal_strength, op| {
            increase_cycle(&mut cycle, &register, &mut signal_strength);

            if op.starts_with("addx") {
                let addx: i32 = op.strip_prefix("addx ").unwrap().parse().unwrap();
                increase_cycle(&mut cycle, &register, &mut signal_strength);
                register += addx;
            };

            signal_strength
        });
        signal_strength.to_string()
    }

    fn part_two(&self, data: &'static str) -> String {
        let mut cursor = 0;
        let mut crt = [' '; 240];
        let mut sprite = 0;

        data.lines().for_each(|op| {
            draw_pixel(&mut crt, &mut cursor, &sprite);

            if op.starts_with("addx") {
                let addx: i32 = op.strip_prefix("addx ").unwrap().parse().unwrap();
                draw_pixel(&mut crt, &mut cursor, &sprite);
                sprite += addx;
            };
        });

        visualize_crt(&crt)
    }
}
