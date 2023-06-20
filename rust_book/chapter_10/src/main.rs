// use std::f64::sqrt;
use std::cmp::Ordering;

fn main() {
    let numbers = vec![2,1,3,8,6,7];

    let char_list = vec!['q', 'w', 'e', 'r', 't', 'a'];

    let point_list = vec![
        LineSegment {x1: 1.0, y1: 1.0, x2: 2.0, y2: 3.0},
        LineSegment {x1: -1.0, y1: -1.5, x2: -2.8, y2: -3.9},
        LineSegment {x1: 0.3, y1: 0.0001, x2: 1.0, y2: 3.03}
        // LineSegment {x1: 1.0, y1: 3.0, x2: -4.0, y2: -5.0}
    ];

    let l1 = LineSegment {x1: 0.0, y1: 0.0, x2: 1.0, y2: 0.0};
    let l2 = LineSegment {x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0};

    println!("Distance of line {:?} {}", &point_list[0], &point_list[0].distance());
    println!("L1 and L2 equal {}", l1 == l2);

    println!("Longest line found: {:?}", largest(&point_list).distance());
    // println!("Largest number found: {}", largest_int(&numbers));
    // println!("Largest char found: {}", largest_char(&char_list));
    // println!("Larges using generic: {}, {}", largest(&numbers), largest(&char_list));
}

#[derive(Debug)]
// #[derive(PartialOrd)]
// #[derive(PartialEq)]
struct LineSegment {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64
}

impl LineSegment {
    fn distance(&self) -> f64 {
        f64::sqrt(
            (&self.x2 - &self.x1) * (&self.x2 - &self.x1)
            +
            (&self.y2 - &self.y1) * (&self.y2 - &self.y1)
        )
    }
}

impl std::cmp::PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        &self.distance() == &other.distance()
    }
}

impl std::cmp::PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if &self.distance() == &other.distance() {
            Some(Ordering::Equal)
        } else if &self.distance() > &other.distance() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_seen = &list[0];

    for item in list {
        if item > largest_seen {
            largest_seen = item;
        }
    }
    largest_seen
}

fn largest_int(numbers: &[i64]) -> &i64 {

    let mut largest_number = &numbers[0];

    for number in numbers {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}

fn largest_char(char_list: &[char]) -> &char {
    let mut largest_char = &char_list[0];

    for c in char_list {
        if c > largest_char {
            largest_char = c;
        }
    }
    largest_char
}