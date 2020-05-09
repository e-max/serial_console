#![no_std]

enum Message {
    Text(&'static str),
    Num(u32),
    Coord { x: u32, y: u32 },
}

//#[cfg(test)]
//mod tests {
//use super::*;
//use test::Bencher;

//#[test]
//fn it_works() {
//assert_eq!(4, add_two(2));
//}

//#[bench]
//fn bench_add_two(b: &mut Bencher) {
//b.iter(|| add_two(2));
//}
//}
