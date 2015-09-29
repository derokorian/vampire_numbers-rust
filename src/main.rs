extern crate num;

use num::pow;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vampires = find_vampire_fangs(args[1].parse::<u8>().unwrap(), args[2].parse::<u8>().unwrap());
    print!("Found {} vampires\n", vampires.len());
    while let Some(vampire) = vampires.pop() {
        print!("{}={}\n", vampire.product, join_fangs(vampire.fangs));
    }
}

struct Vampire {
    product: u32,
    fangs: Vec<u32>
}

fn find_vampire_fangs(pl:u8, fc:u8) -> Vec<Vampire> {
    let fs = pl / fc;
    let mut p: u32;
    let mut vampires: Vec<Vampire> = vec![];
    let upper = num::pow(10, fs as usize) - 1;
    let lower = num::pow(10, (fs - 1) as usize);
    for x in lower..upper {
        for y in x..upper {
            // TODO: This should be dynamic based on fang count
            p = x * y;
            let mut ps = p.to_string().into_bytes();
            let mut fs = (x.to_string() + &y.to_string()).into_bytes();
            ps.sort();
            fs.sort();

            if ps.len() == pl as usize && ps == fs 
               && (x%10 != 0 || y%10 != 0) {
                vampires.push(Vampire{ product: p, fangs: vec![x,y]});
            }
        }
    }
    vampires.sort_by(|a,b| b.product.cmp(&a.product));
    return vampires;
}

fn join_fangs(fangs: Vec<u32>) -> String {
    let mut s = String::new();
    for i in fangs {
        s = s + &i.to_string() + "*";
    }
    s.pop();
    return s;
}

#[test]
fn test_find_vampire_fangs_four_two() {
    let vampires: Vec<Vampire> = find_vampire_fangs(4,2);
    let expected: Vec<Vampire> = vec![
        Vampire {product: 6880, fangs: vec![80,86]},
        Vampire {product: 2187, fangs: vec![27,81]},
        Vampire {product: 1827, fangs: vec![21,87]},
        Vampire {product: 1530, fangs: vec![30,51]},
        Vampire {product: 1435, fangs: vec![35,41]},
        Vampire {product: 1395, fangs: vec![15,93]},
        Vampire {product: 1260, fangs: vec![21,60]}
    ];
    assert_eq_vec_vampire(expected, vampires);
}

fn assert_eq_vec_vampire(mut expected: Vec<Vampire>, mut compared: Vec<Vampire>) {
    assert_eq!(expected.len(), compared.len());
    while let Some(expect) = expected.pop() {
        if let Some(compare) = compared.pop() {
            assert_eq!(expect.product, compare.product);
            assert_eq!(expect.fangs, compare.fangs);
        } else {
            assert!(false);
        }
    }
}


