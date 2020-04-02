#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::josephus_survivor::josephus_survivor;

fn main() {
    assert_eq!(josephus_survivor(7, 3), 4);
    assert_eq!(josephus_survivor(11, 19), 10);
    assert_eq!(josephus_survivor(40, 3), 28);
    assert_eq!(josephus_survivor(14, 2), 13);
    assert_eq!(josephus_survivor(100, 1), 100);
    assert_eq!(josephus_survivor(1, 300), 1);
    assert_eq!(josephus_survivor(2, 300), 1);
    assert_eq!(josephus_survivor(5, 300), 1);
    assert_eq!(josephus_survivor(7, 300), 7);
    assert_eq!(josephus_survivor(300, 300), 265);
}
