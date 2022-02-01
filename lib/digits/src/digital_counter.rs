#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn binary_sequence() {
        let counter = DigitalCounter::new(2, 2);
        let sequence: Vec<Vec<usize>> = counter.collect();
        assert_eq!(sequence.len(), 4);
        assert_eq!(sequence[0], vec![0,0]);
        assert_eq!(sequence[1], vec![1,0]);
        assert_eq!(sequence[3], vec![1,1]);
    }

    #[test]
    fn decimal_sequence() {
        let counter = DigitalCounter::new(3, 10);
        let sequence: Vec<Vec<usize>> = counter.collect();
        assert_eq!(sequence.len(), 1000);
        assert_eq!(sequence[0], vec![0,0,0]);
        assert_eq!(sequence[10], vec![0,1,0]);
        assert_eq!(sequence[153], vec![3,5,1]);
    }

}

pub struct DigitalCounter {
    digits: Vec<usize>,
    base: usize,
    done: bool,
    first: bool, 
}

impl DigitalCounter {
    pub fn new(length: usize, base:usize) -> Self {
        DigitalCounter{digits: vec![0;length], base, done:false, first:true}
    }
}

impl Iterator for DigitalCounter{
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {       
        if self.done {
            return None;
        } 
        if self.first {
            self.first = false;
            return Some(self.digits.clone());
        }
        let mut i = 0;
        self.digits[0] += 1;

        while self.digits[i] >= self.base {
            self.digits[i] = 0;
            i += 1;

            if i >= self.digits.len() {
                self.done = true;
                return None;
            }

            self.digits[i] += 1;
        }

        Some(self.digits.clone())
    }
}