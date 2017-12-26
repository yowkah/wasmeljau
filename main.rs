static mut CPU: Cpu = Cpu {
    memory_in: [0; 1024],
    memory_out: [0; 1024]
};

pub struct Cpu {
    pub memory_in: [u8; 1024],
    pub memory_out: [u8; 1024]
}

#[derive(Debug)]
#[derive(Clone)]
enum PathPart {
    Line(Vec<(f64, f64)>),
    Curve(Vec<(f64, f64)>),
}

impl PathPart {
    fn length(&self) -> f64{
        match self {
            &PathPart::Line(ref x) => ((x[0].0 - x[1].0) * (x[0].0 - x[1].0) + (x[0].1 - x[1].1) * (x[0].1 - x[1].1)).sqrt(),
            &PathPart::Curve(ref x) => lenb(x)
        }
    }
}

fn main() {
    unsafe {
        let temp = [77, 49, 48, 48, 44, 50, 53, 48, 32, 67, 49, 48, 48, 44, 49, 48, 48, 32, 52, 48, 48, 44, 49, 48, 48, 32, 52, 48, 48, 44, 50, 53, 48, 32, 76, 49, 48, 48, 44, 50, 53, 48];
        for i in 0..temp.len() {
            CPU.memory_in[i] = temp[i];
        }
        println!("{:?}", CPU.memory_in[0..25].to_vec());
        leftb(0.2);
        println!("{:?}", CPU.memory_out[0..25].to_vec());
    }
}

#[no_mangle]
pub fn leftb(part: f64) {
    unsafe {
        let (start_point, rest) = pre_parse();
        let mut paths = vec![];
        parse(start_point, rest, &mut paths);
        let result = parse_back(left(part, &paths));
        let mut index = 0;
        for byte in result.as_bytes() {
            CPU.memory_out[index] = *byte;
            index += 1;
        }
        CPU.memory_out[index] = 0;
    }
}

fn parse_back(paths: Vec<PathPart>) -> String {
    let mut result = String::from("");
    match paths[0] {
        PathPart::Line(ref x) | PathPart::Curve(ref x) => result.push_str(&format!("M{} {} ", x[0].0, x[0].1))
    }
    for path in paths {
        match path {
            PathPart::Line(x) => result.push_str(&format!("L{} {} ", x[1].0, x[1].1)),
            PathPart::Curve(x) => {
                result.push('C');
                for i in 1..4 {
                    result.push_str(&format!("{} {} ", x[i].0, x[i].1));
                }
            }
        }
    }
    result
}

fn left(part: f64, paths: &Vec<PathPart>) -> Vec<PathPart> {
    let (index, residual_part) = what_part(part, paths);
    let mut new_path = vec![];
    for i in 0..index {
        new_path.push(paths[i].clone());
    }
    new_path.push(left_residual_path(residual_part, &paths[index]));
    new_path
}

fn left_residual_path(part: f64, path: &PathPart) -> PathPart {
    match path {
        &PathPart::Line(ref x) => PathPart::Line(vec![x[0], (x[0].0 + part * (x[1].0 - x[0].0), x[0].1 + part * (x[1].1 - x[0].1))]),
        &PathPart::Curve(ref x) => {
            let t = newt(part, x);
            let result = newb(t, x);
            PathPart::Curve(leftbrez_to_vec(&result))
        }
    }
}

fn what_part(part: f64, paths: &Vec<PathPart>) -> (usize, f64) {
    let target_length = total_length(paths) * part;
    let mut length = 0.;
    let mut index = 0;
    while length < target_length {
        length += paths[index].length();
        index += 1;
    }
    index -= 1;
    let residual_length = target_length + paths[index].length() - length;
    let residual_part = residual_length / paths[index].length();
    (index, residual_part)
}

fn total_length(paths: &Vec<PathPart>) -> f64 {
    let mut length = 0.;
    for part in paths {
        length += part.length();
    }
    length
}

fn pre_parse() -> ((f64, f64), String) {
    unsafe {
        let input = String::from_utf8(CPU.memory_in.to_vec()).unwrap();
        let rest_index = input.find("\u{0}").unwrap();
        let (first_part, _) = input.split_at(rest_index);
        let (_, part) = first_part.split_at(1);
        let path_string = part.replace(",", " ");
        let mut path_iter = path_string.splitn(3, ' ');
        let start_point = (path_iter.next().unwrap().parse().unwrap(), path_iter.next().unwrap().parse().unwrap());
        (start_point, String::from(path_iter.next().unwrap()))
    }
}

fn parse(start_point: (f64, f64), rest: String, paths: &mut Vec<PathPart>) {
    let (path_type, command) = rest.split_at(1);
    match path_type {
        "C" => { 
            let mut path_iter = command.splitn(7, ' ');
            let mut path = vec![];
            path.push(start_point);
            for _ in 0..3 {
                path.push((path_iter.next().unwrap().parse().unwrap(), path_iter.next().unwrap().parse().unwrap()));
            }
            let start_point = path[3];
            paths.push(PathPart::Curve(path));
            match path_iter.next() {
                Some(x) => parse(start_point, String::from(x), paths),
                None => {}
            }
        },
        "L" => {
            let mut path_iter = command.splitn(3, ' ');
            let mut path = vec![];
            path.push(start_point);
            path.push((path_iter.next().unwrap().parse().unwrap(), path_iter.next().unwrap().parse().unwrap()));
            let start_point = path[1];
            paths.push(PathPart::Line(path));
            match path_iter.next() {
                Some(x) => parse(start_point, String::from(x), paths),
                None => {}
            }
        },
        _ => panic!()
    }
}

#[no_mangle]
pub fn get_memory_in() -> &'static[u8; 1024] {
    unsafe {
        &CPU.memory_in
    }
}

#[no_mangle]
pub fn get_memory_out() -> &'static[u8; 1024] {
    unsafe {
        &CPU.memory_out
    }
}

fn leftbrez_to_vec(b: &Vec<Vec<(f64, f64)>>) -> Vec<(f64, f64)> {
    vec![(b[0][0].0, b[0][0].1), (b[1][0].0, b[1][0].1), (b[2][0].0, b[2][0].1), (b[3][0].0, b[3][0].1)]
}

fn newb(t: f64, b: &Vec<(f64, f64)>) -> Vec<Vec<(f64, f64)>> {
    let n = b.len();
    let mut result = vec![vec![]];
    for bi in b {
        result[0].push(*bi);
    }
    for j in 1..n {
        result.push(vec![]);
        for i in 0..(n - j) {
            let res = (result[j - 1][i].0 * (1. - t) + result[j - 1][i + 1].0 * t, result[j - 1][i].1 * (1. - t) + result[j - 1][i + 1].1 * t);
            result[j].push(res);
        }
    }
    result
}

fn newt(part: f64, b: &Vec<(f64, f64)>) -> f64 {
    let dt = 0.001;
    let totlength = lenb(b) * part;
    let mut t = 0.;
    let mut length = 0.;
    let mut prevpoint = evalb(0., b);
    while length < totlength {
        t += dt;
        let point = evalb(t, b);
        length += ((point.0 - prevpoint.0) * (point.0 - prevpoint.0) + (point.1 - prevpoint.1) * (point.1 - prevpoint.1)).sqrt();
        prevpoint = point;
    }
    t
}

fn lenb(b: &Vec<(f64, f64)>) -> f64 {
    let dt = 0.001;
    let mut t = 0.;
    let mut length = 0.;
    let mut prevpoint = evalb(0., b);
    while t < 1. {
        t += dt;
        let point = evalb(t, b);
        length += ((point.0 - prevpoint.0) * (point.0 - prevpoint.0) + (point.1 - prevpoint.1) * (point.1 - prevpoint.1)).sqrt();
        prevpoint = point;
    }
    length
}

fn evalb(t: f64, b: &Vec<(f64, f64)>) -> (f64, f64) {
    (b[0].0 * (1. - t) * (1. - t) * (1. - t) + 3. * b[1].0 * (1. - t) * (1. - t) * t + 3. * b[2].0 * (1. - t) * t * t + b[3].0 * t * t * t,
        b[0].1 * (1. - t) * (1. - t) * (1. - t) + 3. * b[1].1 * (1. - t) * (1. - t) * t + 3. * b[2].1 * (1. - t) * t * t + b[3].1 * t * t * t)
}