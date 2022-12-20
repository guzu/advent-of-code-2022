use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

//use itertools::Itertools;

fn print(v: &Vec<i64>, map: &Vec<usize>) {
    let mut v2 = vec![0i64; v.len()];

    for (i,j) in map.into_iter().enumerate() {
        v2[*j] = v[i];
    }
    println!(" => {:?}", v2);
}

fn apply_map(v: &Vec<i64>, map: &Vec<usize>) -> Vec<i64> {
    let mut v2 = vec![0i64; v.len()];

    for (i,j) in map.into_iter().enumerate() {
        v2[*j] = v[i];
    }
    v2
}

fn mix(list : &Vec<i64>, nloop: i64) -> (Vec<i64>,Vec<usize>) {
    let mut map : Vec<usize> = vec![0; list.len()];
    map.iter_mut().enumerate().for_each(|(i,n)| *n = i);

    for _ in 0..nloop {
        println!("### {:?}", *list);
        println!("map {:?}", map);
        print(list, &map);
        println!("-----------------");

        // let map2 : Vec<usize> = vec![0, 2, 1, 3, 4, 5, 6];
        // let map3 : Vec<usize> = vec![0, 1, 4, 2, 3, 5, 6];
        // print(&lines, &map3);
        // return ();

        let len = list.len();
        for (i,n) in list.into_iter().enumerate() {
            let j0 = map[i];
            let mut j1;

            if *n == 0 {
                continue;
            }

            if *n > 0 {
                j1 = j0 + (*n % (len - 1) as i64) as usize;
                if j1 >= len - 1 {
                    j1 = (j1 + 1) % len;
                }
            } else {

                if n.abs() < j0 as i64 {
                    j1 = ((j0 + len) as i64 + *n) as usize % len;
                }
                else {
                    let m : i64 = if *n % (len-1) as i64 == 0 { -1i64 * (len - 1) as i64 } else { *n % (len-1) as i64};
                    j1 = ((j0 + len) as i64 + m - 1).abs() as usize % len;
                }
            }
            println!("i:{} move {} from {} to {}", i, *n, j0, j1);

            if j1 > j0 {
                for k in 0..len {
                    if map[k] > j0 && map[k] <= j1 { map[k] -= 1 }
                }
                map[i] = j1;
            }
            else {
                for k in 0..len {
                    if map[k] >= j1 && map[k] <= j0 { map[k] += 1 }
                }
                map[i] = j1;
            }

            //println!("map {:?}", map);
            //print(&lines, &map);
            //println!();
        }
    }
    (apply_map(list, &map), map)
}

fn answer(list: &Vec<i64>, crypt: &Vec<i64>, map: &Vec<usize>) -> i64 {
    let len = list.len();
    let (i0,_) = list.iter()
                        .find_position(|n| **n == 0 )
                            .unwrap();
    println!("{} {}", i0, map[i0]);
    let v1 = crypt[((map[i0] + 1000) % len) as usize];
    let v2 = crypt[((map[i0] + 2000) % len) as usize];
    let v3 = crypt[((map[i0] + 3000) % len) as usize];

    println!("{} {} {}", v1, v2, v3);
    v1 + v2 + v3
}

fn check_map(map: &Vec<usize>) {
    let f1 = (0_usize..map.len()).fold(0, |s,n| s+n);
    let f2 = map.iter().fold(0 as usize, |s, n| s + *n);
    assert_eq!(f1, f2);
}

fn main() {
    // load file
    let lines : Vec<i64> = read_lines("./src/day20/input-test.txt")
        .unwrap()
        .map(|l| l.unwrap().parse::<i64>().unwrap() * 811589153)
        .collect();

    // let (crypt, map) = mix(&lines, 10);
    // println!("+++ {:?}", crypt);
    // check_map(&map);

    let ref_1 : Vec<i64> = vec![ 0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153 ];
    let (crypt, _) = mix(&lines, 1);
    assert_eq!(ref_1,crypt);

    let ref_2 : Vec<i64> = vec![ 0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153 ];
    let (crypt, _) = mix(&lines, 2);
    assert_eq!(ref_2,crypt);

    let ref_3 : Vec<i64> = vec![ 0, 811589153, 2434767459, 3246356612, 1623178306, -1623178306, -2434767459 ];
    let (crypt, _) = mix(&lines, 3);
    assert_eq!(ref_3,crypt);

    let ref_4 : Vec<i64> = vec![ 0, 1623178306, -2434767459, 811589153, 2434767459, 3246356612, -1623178306 ];
    let (crypt, _) = mix(&lines, 4);
    assert_eq!(ref_4,crypt);

    let ref_5 : Vec<i64> = vec![ 0, 811589153, -1623178306, 1623178306, -2434767459, 3246356612, 2434767459 ];
    let (crypt, _) = mix(&lines, 5);
    assert_eq!(ref_5,crypt);

    let ref_6 : Vec<i64> = vec![ 0, 811589153, -1623178306, 3246356612, -2434767459, 1623178306, 2434767459 ];
    let (crypt, _) = mix(&lines, 6);
    assert_eq!(ref_6,crypt);

    let ref_7 : Vec<i64> = vec![ 0, -2434767459, 2434767459, 1623178306, -1623178306, 811589153, 3246356612 ];
    let ref_8 : Vec<i64> = vec![ 0, 1623178306, 3246356612, 811589153, -2434767459, 2434767459, -1623178306 ];
    let ref_9 : Vec<i64> = vec![ 0, 811589153, 1623178306, -2434767459, 3246356612, 2434767459, -1623178306 ];
    let ref10 : Vec<i64> = vec![ 0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153 ];

    //println!("answer: {}", answer(&lines, &crypt, &map));

    // println!("map {:?}", map);
    // print(&lines, &map);
    // println!("==> {:?}", crypt);


    //print(&lines, &map);
    //println!("{:?}", map);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
