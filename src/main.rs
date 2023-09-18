fn main() {
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let filter_condition = FilterCondition { value: 5 };

    let filtered_result = custom_filter(collection, &filter_condition);

    println!("Filtered Result: {:?}", filtered_result);
}

struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.value == item
    }
}

fn custom_filter<T>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
{
    collection
        .into_iter()
        .filter(|item| condition.is_match(item))
        .collect()
}


