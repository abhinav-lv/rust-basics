pub fn variables() {
    // ------------------------------
    // Shadowing
    let name = "Abhinav";
    loop {
        let name = "Mukunda";
        println!("{name}");
        break;
    }
    println!("{name}");
    // ------------------------------
}