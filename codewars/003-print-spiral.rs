/* Print Spiral
   Level 1: 
*Main> printSpiral 10
    99 98 97 96 95 94 93 92 91 90
    64 63 62 61 60 59 58 57 56 89
    65 36 35 34 33 32 31 30 55 88
    66 37 16 15 14 13 12 29 54 87
    67 38 17  4  3  2 11 28 53 86
    68 39 18  5  0  1 10 27 52 85
    69 40 19  6  7  8  9 26 51 84
    70 41 20 21 22 23 24 25 50 83
    71 42 43 44 45 46 47 48 49 82
    72 73 74 75 76 77 78 79 80 81
    *Main>

 */


const SPIRAL_SIDE: u32 = 10;

fn main() {
    let border = (SPIRAL_SIDE / 2) as i32;
    let (min, max) = if SPIRAL_SIDE % 2 == 0 {
        (-border + 1, border + 1)
    } else {
        (-border, border)
    };

    for y in (min..max).rev() {
        (min..max)
            .into_iter()
            .for_each(|x| print!("{:>3} ", number_at(x, y)));
        println!();
    }
}

fn number_at(x: i32, y: i32) -> i32 {
    let distance_from_center = x.abs().max(y.abs());

    if y > -x {
        calculate_number_for(x, y, 2 * distance_from_center - 1)
    } else {
        calculate_number_for(-x, -y, 2 * distance_from_center)
    }
}

fn calculate_number_for(x: i32, y: i32, layer_id: i32) -> i32 {
    let bottom_right_num = layer_id.pow(2);
    let top_right_num = bottom_right_num + layer_id;

    let dist_from_top_right = (x - y).abs();

    if y <= x {
        top_right_num - dist_from_top_right
    } else {
        top_right_num + dist_from_top_right
    }
}
