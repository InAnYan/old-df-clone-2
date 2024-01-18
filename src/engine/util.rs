pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input > max {
        max
    } else if input < min {
        min
    } else {
        input
    }
}
