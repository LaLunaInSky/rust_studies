pub trait Summary {
    fn summarize(
        &self
    ) -> String;
}

pub mod aggregator;