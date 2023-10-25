#![allow(dead_code)]
use std::{collections::HashMap, vec};

pub fn vectors() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

pub fn strings() {
    let s1 = String::from("hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");
    // println!("{s1}");        // This will throw error since s1 is moved in '+'

    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");
    let a = format!("{}-{}-{}", a1, a2, a3);
    println!("{a}");

    for c in "வணக்கம்".chars() {
        println!("{c}");
    }
}

/* MAPS ------------------------------------------------------------- */
fn disp_map(m: &HashMap<String, i32>) {
    println!("--------------");
    for (key, value) in m {
        println!("{}: {}", *key, *value);
    }
}

fn map_exercise(s: &String) {
    // take in a sentence and return frequency of words
    let words = s.split(' ');
    let mut freq: HashMap<String, u32> = HashMap::new();
    for word in words {
        let count = freq.entry(String::from(word)).or_insert(0);
        *count += 1;
    }
    for (key, value) in &freq {
        println!("{}: {}", *key, *value);
    }
}

pub fn maps() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Red"), 25);

    // let bruh = scores.get(&String::from("Red")).copied().unwrap_or(0);
    // println!("{bruh}");

    // disp_map(&scores);

    // scores.entry(String::from("Green")).or_insert(69);

    // disp_map(&scores);

    // map_exercise(&String::from(
    //     "I'm a blithering old idiot in an old blithering country that's old as shit.",
    // ));

    /* -------------------------------------------------- */

    /* Exercise 1 */
    // let nums = [3,5,2,5,7,7,9,9,8,7,5,5,10,11];
    // median_mode(&nums);

    /* -------------------------------------------------- */

    /* Exercise 2 */
    // let word = String::from("first");
    // to_pig_latin(&word);
    // println!("{word}");

    /* -------------------------------------------------- */

    /* Exercise 3 */
    let mut company = company::Company::new();

    company.add_employee("Add Abhinav L V to Rust Computing");
    company.add_employee("Add Kanish Gupta Singh to Sales");
    company.display_all();
}

/* EXERCISES */

// 1
fn median_mode(list: &[i32]) {
    let mut nums: Vec<i32> = Vec::new();
    let mut freq = HashMap::new();
    let mut res: Vec<i32> = vec![0, 0];

    for num in list {
        nums.push(*num);
        let t = freq.entry(*num).or_insert(0);
        *t += 1;
    }

    nums.sort();
    // println!("{:?}", nums);

    // Median
    res[0] = if nums.len() % 2 == 0 {
        (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) / 2
    } else {
        nums[nums.len() / 2]
    };

    // Mode
    let mut t = i32::MIN;
    for (key, value) in &freq {
        res[1] = if t < *value {
            t = *value;
            *key
        } else {
            res[1]
        };
    }

    // for (key, value) in &freq {
    //     println!("{}: {}", key, value);
    // }

    println!("Median: {}, Mode: {}", res[0], res[1]);
}

// 2
fn to_pig_latin(word: &String) {
    if word.len() == 0 {
        println!("No word given! You're a pig!");
        return;
    }

    let mut res = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let starts_with_vowel = vowels.contains(&word.chars().nth(0).unwrap());

    if starts_with_vowel {
        res.push_str(&format!("{}-hay", &word[..]));
    } else {
        res.push_str(&format!("{}-{}ay", &word[1..], &word[0..1]));
    }

    println!("{res}");
}

// 3
pub mod company {
    use std::collections::HashMap;

    pub struct Company {
        employees_by_department: HashMap<String, Vec<Employee>>,
    }

    impl Company {
        pub fn new() -> Self {
            Company {
                employees_by_department: HashMap::new(),
            }
        }

        // returns true if employee is added successfully
        pub fn add_employee(&mut self, command: &str) -> bool {
            let mut words = command.split_ascii_whitespace();
            let mut employee_name = String::new();
            let mut department_name = String::new();
            let mut found = [false, false, false, false];
            loop {
                // be careful here, the word gets consumed by calling nth()!
                match words.nth(0) {
                    Some(word) => {
                        if word == "Add" {
                            found[0] = true;
                        } else if word != "to" && found[0] && !found[2] {
                            found[1] = true;
                            employee_name.push_str(&format!("{} ", word));
                        } else if word == "to" && found[1] && !found[2] {
                            found[2] = true;
                        } else if found[2] {
                            found[3] = true;
                            department_name.push_str(&format!("{} ", word));
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            if found[0] && found[1] && found[2] && found[3] {
                let mut employee = Employee::new();
                employee.set(employee_name, department_name.clone());
                let t = self
                    .employees_by_department
                    .entry(department_name)
                    .or_insert(vec![]);
                t.push(employee);
                return true;
            }
            false
        }

        pub fn display_employees_by_dept(&self, dept: String) {
            let t = self.employees_by_department.get(&dept);
            match t {
                Some(emps) => {
                    for emp in emps {
                        emp.display();
                    }
                }
                None => println!("No employees in the department {}.", dept),
            }
        }

        pub fn display_all(&self) {
            for (key, value) in &self.employees_by_department {
                println!("--- {}---", *key);
                for emp in value {
                    emp.display();
                }
            }
        }
    }

    pub struct Employee {
        name: String,
        department: String,
    }

    impl Employee {
        pub fn new() -> Self {
            Employee {
                name: String::new(),
                department: String::new(),
            }
        }

        pub fn set(&mut self, name: String, department: String) {
            self.name = name;
            self.department = department;
        }

        pub fn display(&self) {
            println!("Name: {}", self.name);
            println!("Department: {}", self.department);
        }
    }
}
