pub fn ownership() {
    let mut s = String::from("hello");
    // s.push_str(" monke");
    // dbg!(s.clone());
    // let a = s.split(" ");
    // for bleh in a{
    //     println!("{bleh}");
    // }
    // bruh(s);
    // println!("{s}");    // error
    take_a_reference_pill_bruv(&s);
    s = String::from("Damn! ") + &s;
    take_a_reference_pill_bruv(&s);

    let mut a = 10;
    let mut b = 20;
    swap(&mut a, &mut b);
    dbg!(a,b);

    two_mutable_references();
}


/*
fn bruh(boom: String) {
    println!("{boom}");
}
*/

fn take_a_reference_pill_bruv(cup_of_tea: &String) {
    dbg!(cup_of_tea);
}

fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
}

/* 
 *  WE CANNOT HAVE A MUTABLE REFERENCE IN THE SAME SCOPE AS ANOTHER MUTABLE REFERENCE
 *  OR AN IMMUTABLE REFERENCE.
 */
fn two_mutable_references() {
    // not allowed
    let mut s = String::from("deadmau5");
    let r1 = &mut s;
    // let r2: &mut s; // error
    r1.push_str(", the GOAT.");
    dbg!(r1);
    let r2 = &mut s;
    dbg!(r2);
}