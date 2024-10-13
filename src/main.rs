use core::arch;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::mpsc::{self},
    thread::{self, spawn},
};

use chrono::{Local, Utc};
fn main() {
    let ans = is_even(10);

    let fibo = fiboncci(10);

    let str_length = get_str_length("Hello World".to_string());

    let person = User {
        name: "John".to_string(),
        age: 20,
    };

    let rect = Rect {
        width: 10,
        height: 10,
    };
    println!("{} {}", rect.area(), rect.perimeter());

    let now = Local::now();
    println!("current time is {}", now);

    println!(
        "{} {} {} {} {}",
        ans, fibo, str_length, person.name, person.age
    );

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let result = filter_vector_even(&vec);

    println!("{:?}", result);

    let tuples = vec![
        (String::from("Pramod"), 1),
        (String::from("Pramod"), 2),
        (String::from("Other"), 4),
    ];

    let val = group_values_by_name(tuples);
    println!("{:?}", val);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    println!("{}", total);

    assert_eq!(total, 6);
    //consumed cannot be used later
    //let sum2:i32=v1_iter.sum();

    odd_double_vector();

    let word = String::from("Pramod Daware");

    let first_word = find_first_word(&word);
    // word=String:: from("sdfs");
    // first_word = &word[0..5];

    // println!("{}", first_word);

    println!("{}", first_word);

    let person: Person = Person {
        name: String::from("Pramod"),
        age: 23,
    };
    let summary = person.summarise();
    println!("{}", summary);

    let user1: User = User {
        name: String::from("Prmaod"),
        age: 343,
    };
    let summary2 = user1.summarise();

    println!("{}", summary2);

    // let ans;
    let str1 = String::from("Small");
    // {
    let str2 = String::from("Longer");
    // ans = longest(&str1, &str2);
    // }
    // println!("{}", ans);
    longest_with_announcement(&str1, &str2, person);

    // let handle = thread::spawn(|| {
    //     for i in 0..100 {
    //         println!("{}", i);
    //     }
    // });

    // for i in 0..200 {
    //     println!("{}", i);
    // }

    //Channels
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i + 1 * 10000000) - 1 {
                sum += j;
            }
            producer.send(sum).unwrap();
        });
    }

    drop(tx);
    let mut final_sum = 0;
    for val in rx {
        final_sum += val;
    }

    println!("Final sum : {}", final_sum);
}

//lifetimes, traits, genercs
fn longest_with_announcement<'a, T: Display>(
    str1: &'a str,
    str2: &'a str,
    announcement: T,
) -> &'a str {
    println!("Annopucement ! {announcement}");
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
//understainfthe life times
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
//function  tro filter out all the odd values and then fouble each value and create a new Vectore
fn odd_double_vector() {
    let v1 = vec![1, 2, 3, 4, 5, 6];

    let odd_iter = v1.iter().filter(|x| *x % 2 == 1);
    // for i in odd_iter {
    //     println!("{}", i);
    // }
    let doubled_iter = odd_iter.map(|x| x * 2);
    for x in doubled_iter {
        println!("{}", x);
    }
}

//string and slices
fn find_first_word(words: &String) -> &str {
    let mut index = 0;
    for (_, i) in words.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index += 1;
    }
    return &words[0..index];
}

fn group_values_by_name(data: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    for (key, value) in data {
        let present_value = map.get_mut(&key);
        match present_value {
            Some(val) => val.push(value),
            None => {
                map.insert(key, vec![value]);
            }
        }
    }
    return map;
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn fiboncci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fiboncci(n - 1) + fiboncci(n - 2);
    }
}

fn get_str_length(str: String) -> u32 {
    let mut count = 0;
    for _ in str.chars() {
        count += 1;
    }
    return count;
}

struct User {
    name: String,
    age: u32,
}

struct Rect {
    height: u32,
    width: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn perimeter(&self) -> u32 {
        return (self.height + self.width) * 2;
    }
}

enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn print_area(shape: Shape) -> f64 {
    return match shape {
        Shape::Circle(r) => r * r * 3.14,
        Shape::Rectangle(r, b) => r * b,
    };
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

fn filter_vector_even(vec: &Vec<i32>) -> Vec<&i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}

trait Summary {
    fn summarise(&self) -> String {
        return String::from("No value");
    }
}

struct Person {
    name: String,
    age: i32,
}

impl Summary for Person {
    fn summarise(&self) -> String {
        return String::from(format!("{} is {} years old", self.name, self.age));
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

impl Summary for User {}
