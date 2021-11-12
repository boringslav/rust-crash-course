pub fn get_largest<T: PartialOrd + Copy> (list: Vec<T>) -> T {
    let mut largest = list [0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}