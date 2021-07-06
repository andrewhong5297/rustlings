  
// move_semantics2.rs
// Make me compile without changing line 10! Scroll down for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut ownedVec = vec.to_vec();

    ownedVec.push(22);
    ownedVec.push(44);
    ownedVec.push(66);

    ownedVec
}