use rand::Rng;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

const LEVEL: [char; 10] = [' ', '.', '-', '=', 'c', 'o', 'a', 'A', '@', '#'];
const LEVEL_COUNT: usize = LEVEL.len() - 1;

const ALPHA_N: f32 = 0.028;
const ALPHA_M: f32 = 0.147;

const B1: f32 = 0.278;
const B2: f32 = 0.365;
const D1: f32 = 0.267;
const D2: f32 = 0.445;

const DT: f32 = 0.01;

fn gen_grid(width: usize, height: usize) -> Vec<Vec<f32>> {
    let grid = vec![vec![0.0; width]; height];
    grid
}

fn gen_random_grid(width: usize, height: usize) -> Vec<Vec<f32>> {
    let mut grid = gen_grid(width, height);
    let mut rng = rand::thread_rng();
    let w = width / 3;
    let h = height / 3;
    println!("{w}, {h}");
    for dy in 0..h {
        for dx in 0..w {
            let x = dx + width / 2 - w / 2;
            let y = dy + height / 2 - h / 2;
            grid[y][x] = rng.gen_range(0.0..1.0);
        }
    }
    grid
}

fn display_grid(grid: &Vec<Vec<f32>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = LEVEL[(grid[y][x] * LEVEL_COUNT as f32) as usize];
            print!("{} ", c);
        }
        println!("");
    }
}

fn emod(a: i32, b: i32) -> usize {
    ((a % b + b) % b) as usize
}

fn sigma(x: f32, a: f32, alpha: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-(x - a) * 4.0 / alpha))
}

fn sigma_n(x: f32, a: f32, b: f32) -> f32 {
    sigma(x, a, ALPHA_N) * (1.0 - sigma(x, b, ALPHA_N))
}

fn sigma_m(x: f32, y: f32, m: f32) -> f32 {
    let s = sigma(m, 0.5, ALPHA_M);
    x * (1.0 - s) + y * s
}

fn s(n: f32, m: f32) -> f32 {
    sigma_n(n, sigma_m(B1, D1, m), sigma_m(B2, D2, m))
}

fn compute_grid_diff(grid: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut grid_diff = gen_grid(WIDTH, HEIGHT);

    let ra: i32 = 11;
    let ri: f32 = ra as f32 / 3.0;
    // let cx: i32 = 0;
    // let cy: i32 = 0;
    for cx in 0..HEIGHT {
        for cy in 0..WIDTH {
            let mut m: f32 = 0.0;
            let mut n: f32 = 0.0;
            let mut _m = 0;
            let mut _n = 0;

            for dy in -(ra - 1)..ra {
                for dx in -(ra - 1)..ra {
                    let x = emod(cx as i32 + dx, WIDTH as i32);
                    let y = emod(cy as i32 + dy, HEIGHT as i32);
                    if (dx * dx + dy * dy) as f32 <= ri * ri {
                        m += grid[y][x];
                        _m += 1;
                    } else if (dx * dx + dy * dy) <= ra * ra {
                        n += grid[y][x];
                        _n += 1;
                    }
                }
            }
            m /= _m as f32;
            n /= _n as f32;
            let q = s(n, m);
            grid_diff[cy][cx] = 2.0 * q - 1.0;
        }
    }
    grid_diff
}

fn clamp(x: &mut f32, l: f32, h: f32) {
    if *x < l {
        *x = l;
    }
    if *x > h {
        *x = h;
    }
}

pub fn run() {
    let mut grid = gen_random_grid(WIDTH, HEIGHT);
    display_grid(&grid);
    println!("");

    loop {
        let grid_diff = compute_grid_diff(&grid);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                grid[y][x] += DT * grid_diff[y][x];
                clamp(&mut grid[y][x], 0.0, 1.0);
            }
        }

        display_grid(&grid);
    }
    // println!("{:?}", grid);
}
