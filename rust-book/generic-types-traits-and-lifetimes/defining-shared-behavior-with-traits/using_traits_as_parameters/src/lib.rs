pub trait Summary {
    fn summarize(
        &self
    ) -> String;
}

pub fn notify(item: &impl Summary) {
    println!(
        "Breaking news! {}",
        item.summarize()
    );
}

pub fn aggregator;