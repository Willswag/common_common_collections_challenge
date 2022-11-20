pub mod stats{
    pub fn mean(v1: & Vec<u32>) -> u32 {
        let mut acc = 0;
        let n = v1.len() as u32;

        for i in v1{
            acc += *i;
        }
        acc / n

    }
    
}