pub fn conditionals() {
    // if_conditional();

    // loop_lol();
    // loop_returns_a_value_lol();
    // loop_label_lol();
    // for_loop();
    countdown();
}

/*
fn if_conditional() {
    let my_name: &str = "mukunda";
    if my_name == "abhinav" {
        println!("My name is Abhinav!");
    } else {
        println!("Oh no! My name is not Abhinav. My name is {my_name}")
    }

    // alternative
    let my_res = if my_name == "abhinav" {
        "heck yeah"
    } else {
        "boo hoo"
    };
    println!("{my_res}");

    // error ... not similar to C
    // let num = 69;
    // if num {
    //     dbg!(num);
    // }
}
*/

/*
fn loop_lol() {
    let mut count = 69;
    loop {
        println!("I'm looping!!! lol");
        count -= 1;
        if count == 0 {
            break;
        }
    }
}
*/

/*
fn loop_returns_a_value_lol() {
    let mut counter = 100;
    let result = loop {
        counter -= 1;
        if counter == 0 {
            break "done";
        }
    };
    dbg!(result);
}
*/

/*
fn loop_label_lol() {
    let mut counter_top = 69;
    'top_lol: loop {
        let mut counter_bottom = 23;
        loop {
            println!("I'm a bottom feeder lol");
            counter_bottom -= 1;
            if counter_bottom == 0 {
                break;
            }
            if counter_top == 66 {
                break 'top_lol;
            }
        }
        println!("*****************************************");
        println!("");
        println!("I'm a top feeder lol");
        println!("");
        println!("*****************************************");
        counter_top -= 1;
    }
} 
*/

/*

fn for_loop() {
    let a = [1,2,3,4,5];
    let mut counter = 1;
    for element in a {
        println!("Element {} is: {}",counter, element);
        counter += 1;
    }
}
 */

fn countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!! lol");
}