static mut CPU: Cpu = Cpu {
    memory: [0; 8]
};

pub struct Cpu {
    pub memory: [u8; 8]
}

fn main() {
    unsafe {
        CPU.memory = [200, 200, 200, 100, 100, 100, 200, 200];
        leftb(0.25);
        println!("{:?}", CPU.memory);
    }
}

#[no_mangle]
pub fn leftb(part: f64) {
    unsafe {
        let b = byte_to_brez(&CPU.memory);
        // let t = newt(part, &b);
        // let result = newb(t, b);
        // for i in 0..8 {
        //     CPU.memory[i] = leftbrez_to_byte(&result)[i];
        // }
        CPU.memory[5] = 5;
    }
}

#[no_mangle]
pub fn get_memory() -> &'static[u8; 8] {
    unsafe {
        &CPU.memory
    }
}

// fn leftbrez_to_byte(b: &Vec<Vec<(f64, f64)>>) -> [f64; 8] {
//     [b[0][0].0, b[0][0].1, b[1][0].0, b[1][0].1, b[2][0].0, b[2][0].1, b[3][0].0, b[3][0].1]
// }

fn byte_to_brez( bytes: &[u8; 8]) -> Vec<(u8, u8)>{
    vec![(bytes[0],bytes[1]),(bytes[2],bytes[3]),(bytes[4],bytes[5]),(bytes[6],bytes[7])]
}

// fn newb(t: f64, b: Vec<(f64, f64)>) -> Vec<Vec<(f64, f64)>> {
//     let n = b.len();
//     let mut result = vec![vec![]];
//     for bi in b {
//         result[0].push(bi);
//     }
//     for j in 1..n {
//         result.push(vec![]);
//         for i in 0..(n - j) {
//             let res = (result[j - 1][i].0 * (1. - t) + result[j - 1][i + 1].0 * t, result[j - 1][i].1 * (1. - t) + result[j - 1][i + 1].1 * t);
//             result[j].push(res);
//         }
//     }
//     result
// }

// fn newt(part: f64, b: &Vec<(f64, f64)>) -> f64 {
//     let dt = 0.001;
//     let totlength = lenb(b) * part;
//     let mut t = 0.;
//     let mut length = 0.;
//     let mut prevpoint = evalb(0., b);
//     while length < totlength {
//         t += dt;
//         let point = evalb(t, b);
//         length += ((point.0 - prevpoint.0) * (point.0 - prevpoint.0) + (point.1 - prevpoint.1) * (point.1 - prevpoint.1)).sqrt();
//         prevpoint = point;
//     }
//     t
// }

// fn lenb(b: &Vec<(f64, f64)>) -> f64 {
//     let dt = 0.001;
//     let mut t = 0.;
//     let mut length = 0.;
//     let mut prevpoint = evalb(0., b);
//     while t < 1. {
//         t += dt;
//         let point = evalb(t, b);
//         length += ((point.0 - prevpoint.0) * (point.0 - prevpoint.0) + (point.1 - prevpoint.1) * (point.1 - prevpoint.1)).sqrt();
//         prevpoint = point;
//     }
//     length
// }

// fn evalb(t: f64, b: &Vec<(f64, f64)>) -> (f64, f64) {
//     (b[0].0 * (1. - t) * (1. - t) * (1. - t) + 3. * b[1].0 * (1. - t) * (1. - t) * t + 3. * b[2].0 * (1. - t) * t * t + b[3].0 * t * t * t,
//         b[0].1 * (1. - t) * (1. - t) * (1. - t) + 3. * b[1].1 * (1. - t) * (1. - t) * t + 3. * b[2].1 * (1. - t) * t * t + b[3].1 * t * t * t)
// }