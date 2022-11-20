// given a list of ints use a vector and return the mean and mode

// convert a string to pig latin 
//  the first consonant goes to the end of the word and the "ay" is added to back

// hashmap and vector that allow a user to store names and departments

pub mod stats;

fn main() {

    let v:Vec<u32> = vec![1,1,1,2,5,17,4];
    
    let avg = crate::stats::stats::mean(&v);
    
    println!("avg {avg}");
}

