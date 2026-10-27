use std::{thread::sleep, time::Duration};

const WIDTH: usize = 80;
const HEIGHT: usize = 24;

fn main() {
    let cube = [
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, 1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, 1.0],
        [1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0],
    ];
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    let mut angle_x: f32 = 0.0;
    let mut angle_y: f32 = 0.0;

    loop {
        let mut screen = [[' '; WIDTH]; HEIGHT];
        let mut proj = Vec::with_capacity(8);

        for &[x, y, z] in &cube {
            let x1 = x * angle_y.cos() + z * angle_y.sin();
            let z1 = -x * angle_y.sin() + z * angle_y.cos();

            let y2 = y * angle_x.cos() - z1 * angle_x.sin();
            let z2 = y * angle_x.sin() + z1 * angle_x.cos();

            let dist = z2 + 3.5;

            let px = (x1 / dist * 32.0 + 40.0).round() as i32;
            let py = (y2 / dist * 16.0 + 12.0).round() as i32;

            proj.push((px, py));
        }

        for &(i1, i2) in &edges {
            let (x1, y1) = proj[i1];
            let (x2, y2) = proj[i2];

            let dx = x2 - x1;
            let dy = y2 - y1;
            let steps = dx.abs().max(dy.abs());

            if steps == 0 {
                continue;
            }

            for p in 0..=steps {
                let t = p as f32 / steps as f32;

                let cur_x = (x1 as f32 + t * dx as f32).round() as i32;
                let cur_y = (y1 as f32 + t * dy as f32).round() as i32;

                if cur_x >= 0 && cur_x < WIDTH as i32 && cur_y >= 0 && cur_y < HEIGHT as i32 {
                    screen[cur_y as usize][cur_x as usize] = 'Z';
                }
            }
        }
        print!("\x1B[2J\x1B[H");
        for row in screen {
            let line: String = row.iter().collect();
            println!("{}", line);
        }

        angle_x += 0.03;
        angle_y += 0.05;

        sleep(Duration::from_millis(33));
        println!("end");
    }
}
