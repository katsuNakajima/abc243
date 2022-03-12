use std::collections::BTreeMap;

#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let n = parse_line!(usize);
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _i in 0..n {
        let (xx, yy) = parse_line!(i32, i32);
        x.push(xx);
        y.push(yy);
    }
    let s = parse_line!(String);
    let schar: Vec<char> = s.chars().collect();
    let mut map = BTreeMap::new();
    for i in 0..n {
        let xx = x[i];
        let yy = y[i];
        let c = schar[i];
        map.entry(yy).or_insert_with(Vec::new).push((xx, c));
    }

    for (_, xc) in map {
        let min_r = xc.iter().filter(|&&(_, c)| c == 'R').map(|&(x, _)| x).min();
        let max_l = xc.iter().filter(|&&(_, c)| c == 'L').map(|&(x, _)| x).max();
        match (min_r, max_l) {
            (Some(r), Some(l)) => {
                if r < l {
                    println!("Yes");
                    return;
                }
            }
            _ => {}
        }
    }
    println!("No");
}
