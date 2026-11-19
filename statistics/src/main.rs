
// some des observations
fn sum(obs: &[i32]) -> i32 {
    let mut sm = 0;
    for i in obs {
        sm += *i;
    }
    sm
}

// moyenne des observations
fn average(obs: &[i32]) -> i32 {
   sum(obs) /  obs.len() as i32
}


fn main() {
    println!("sum : {}", average(&[5, 4, 6]));
}
