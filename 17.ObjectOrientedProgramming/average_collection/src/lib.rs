/*****************************************************************************
 * Filename      : lib.rs
 * Created       : Thu Jul 28 2022
 * Author        : Zolo
 * Github        : https://github.com/zolodev
 * Description   : Working through the Rust book chapter 17
*****************************************************************************/
#![warn(clippy::all, clippy::pedantic)]

pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    #[must_use]
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = f64::from(total) / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ac2 = AverageCollection {
            list: vec![1, 2],
            average: 0.0,
        };

        ac2.add(3);

        eprintln!("ac2 average: {}", ac2.average());
        assert_eq!(2.0, ac2.average());
    }
}
