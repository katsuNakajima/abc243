use std::collections::HashSet;

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
    let a = parse_vec!(i64);
    let b = parse_vec!(i64);
    let mut cnt = 0;
    let aset: HashSet<&i64> = a.iter().collect();
    let bset: HashSet<&i64> = b.iter().collect();
    for i in 0..n {
        if a[i] == b[i] {
            cnt += 1;
        }
    }
    let cnt2 = aset.intersection(&bset).collect::<Vec<&&i64>>().len();
    println!("{}", cnt);
    println!("{}", cnt2 - cnt);
}
