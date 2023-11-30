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

    // REMOVE an element from a vector
    let mut v4 = vec![1, 2, 3, 4, 5];
    let needle = 3;
    // Option 1: Remove first instance equal to needle O(n)
    v4.remove(
        v4.iter()
            .position(|x| *x == needle)
            .expect("needle not found"),
    );
    // Option 2: Remove last instance equal to needle O(n)
    v4.remove(
        v4.iter()
            .rposition(|x| *x == needle)
            .expect("needle not found"),
    );
    // Option 2: Remove all instances equal to needle
    v4.retain(|x| *x != needle);
    // Option 3: Remove and swap by shifting indices to last element O(1)
    v4.swap_remove(
        v4.iter()
            .position(|x| *x == needle)
            .expect("needle not found"),
    );
}
