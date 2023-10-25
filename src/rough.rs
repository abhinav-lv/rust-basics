#![allow(dead_code)]

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn square(side: i32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }

    fn display(&self) {
        println!("A rectangle with width {} and height {}.", self.width, self.height);
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }
}

pub fn rough() {
    // mutable_references();
    // index_out_of_bounds_at_runtime();

    // let mut v: Vec<char> = vec!['a', 'b', 'c'];
    // println!("{:?}", v);
    // capitalize_first_character(&mut v);
    // println!("{:?}", v);

    // let mut nums: Vec<i32> = vec![1, 2, 3, 4];
    // println!("{:?}", nums);
    // func(&mut nums);
    // println!("{:?}", nums);

    // let mut c = Rectangle::square(32);
    // c.display();
    // c.set_width(43);
    // c.display();
    second_largest_element_in_array(&vec![3,7,2,1,96,55,49]);
}

fn second_largest_element_in_array(arr: &Vec<i32>) {
    let mut first = i32::MIN;
    let mut second = i32::MIN;
    if arr.len() < 2 {
        // handle insufficient case
        return;
    }
    
    for num in arr {
        if *num > first {
            second = first;
            first = *num;
        }
        else if *num > second {
            second = *num;
        }
    }

    println!("Second largest element in array: {}", second);
}

fn mutable_references() {
    let mut v: Vec<i32> = vec![34, 45, 56, 67];
    let num: &mut i32 = &mut v[2];
    *num = 3;
    println!("{}", *num);
    println!("Vector is now: {:?}", v);
}

fn index_out_of_bounds_at_runtime() {
    let v: Vec<i32> = vec![69, 420, 54];
    println!("{}", v[3]);
}

fn capitalize_first_character(v: &mut Vec<char>) {
    v[0] = v[0].to_ascii_uppercase();
}

fn func(nums: &mut [i32]) {
    nums.reverse();
}
