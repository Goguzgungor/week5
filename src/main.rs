struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        // Customize the matching logic based on your requirements
        *item == self.value
    }
}

fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
    T: Clone,
{
    let mut filtered = Vec::new();

    for item in collection {
        if condition.is_match(item) {
            filtered.push(item.clone());
        }
    }

    filtered
}

fn main() {
    // Create a collection
    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Initialize a FilterCondition object
    let condition = FilterCondition { value: 5 };

    // Call custom_filter function
    let filtered_result = custom_filter(&collection, &condition);

    // Print the filtered result
    for item in filtered_result {
        print!("{} ", item);
    }
    println!();
}
