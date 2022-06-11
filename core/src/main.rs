use get_str::get_string;

fn main() {
    let s = get_string();
    // assert_eq!("not a test", &s); this assertion fails
    println!("This is ... {}", s);
}


#[cfg(test)]
mod test {
    use get_str::get_string;

    #[test]
    fn test_get_str() {
        assert_eq!("a test", get_string());
    }
}