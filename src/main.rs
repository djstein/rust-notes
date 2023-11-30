fn main() {
    // create a reverse iterator over a range
    for i in (0..=99).rev() {
        println!("{}", i)
    }

    // new vector
    let mut _v1: Vec<i32> = <Vec<i32>>::new();
    let _v2: Vec<i32> = vec![];
    let v3: Vec<i32> = vec![1, 2, 3];
    // loop over a vector in reverse USING index
    for p in (0..=v3.len() - 1).rev() {
        println!("{}", v3[p]);
    }
}
