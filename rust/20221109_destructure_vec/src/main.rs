fn extract_args<const N: usize, const M: usize>(args: Vec<String>) -> [String; M] {
    args.into_iter()
        .chain(std::iter::repeat_with(|| "".to_string()))
        .take(M)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn main() {
    println!("Hello, world!");

    let args = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let res = extract_args::<3, 3>(args);
    println!("{:?}", res); // ["a", "b", "c"]

    let args = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let res = extract_args::<2, 3>(args);
    println!("{:?}", res); // ["a", "b", "c"]

    let args = vec!["a".to_string(), "b".to_string()];
    let res = extract_args::<2, 3>(args);
    println!("{:?}", res); // ["a", "b", ""]

    let args = vec!["a".to_string(), "b".to_string()];
    let [a, b, c] = extract_args::<2, 3>(args);
    println!("{}, {}, {}", a, b, c); // a, b, ""
}
