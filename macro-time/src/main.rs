use std::collections::HashMap;
use proccers::{XmlSerialize};
use stuff::XSerialize;

macro_rules! x_and_y {
    (x => $e:expr) => {
        println!("X: {}", $e)
    };
    (y => $e:expr) => {
        println!("Y: {}", $e)
    };
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    }
}

macro_rules! print_ex {
    ($e:expr) => {
        println!("{:?} = {:?}",
            stringify!($e),
            $e);
    };
}

macro_rules! exame {
    ($l: expr; and $r:expr) => {
        println!("{:?} and {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r)
    };
    ($l: expr; or $r:expr) => {
        println!("{:?} or {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r)
    };
}

macro_rules! compr {
    ($id1: ident | $id2: ident <- [$start: expr ; $end: expr] , $cond: expr) => {
        {
            let mut vec = Vec::new();

            for num in $start..$end + 1 {
                if $cond(num) {
                    vec.push(num);
                }
            }

            vec
        }
    };
}

fn even(x: i32) -> bool {
    x&1==0
}

fn odd(x: i32) -> bool {
    x&1==1
}

macro_rules! new_map {
    ($($key: expr => $val: expr)*) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

#[derive(XmlSerialize)]
pub struct TestemUpFierce {
    something: i32,
}

build_fn!(lawl);

fn main() {
    x_and_y!(x => 10);
    x_and_y!(y => 20 + 30);

    lawl();

    let x = 20;
    print_ex!({
        let y = 30;
        x + y + 10 * 3 * 100
    });

    exame!(true; and true);
    exame!(true; or false);

    let evens = compr![x | x <- [1;10], even];
    println!("{:?}", evens);

    let odds = compr![y | y <- [1;10], odd];
    println!("{:?}", odds);

    let m = new_map!{
        1 => "one"
        2 => "two"
        3 => "three"
    };

    println!("{:?}", m);

    let yoip = TestemUpFierce {
        something: 3,
    };

    println!("{}", yoip.xml_serialize());
}
