use std::collections::HashMap;

#[derive(Debug)]
pub struct Point<T, U> {
    x: T,
    y: U
}

impl<T: Clone, U: Clone> Point<T, U> {
    pub fn new(
        x: T,
        y: U
    ) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn x(
        &self
    ) -> HashMap<char, T> {
        let mut result: HashMap<char, T> = HashMap::new();

        result.insert(
            'x',
            self.x.clone()
        );

        result
    }

    pub fn y(
        &self
    ) -> HashMap<char, U> {
        let mut result: HashMap<char, U> = HashMap::new();

        result.insert(
            'y',
            self.y.clone()
        );

        result
    }
}