pub fn data_types() {
    let tup: (i32, f64, char) = (1, 5.6, 'a');
    dbg!(tup.1);
    
    // array
    let arr: [i32; 10] = [1,2,3,4,5,6,7,8,9,'a' as i32];
    dbg!(arr);
}
