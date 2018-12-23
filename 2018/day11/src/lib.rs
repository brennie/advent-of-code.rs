pub fn generate_grid(serial: i32) -> Vec<Vec<i32>> {
    (1..=300)
        .map(|y| (1..=300).map(|x| power(x, y, serial)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;

    ((((rack_id * y + serial) * rack_id) / 100) % 10) - 5
}

#[cfg(test)]
mod test {
    use super::power;

    #[test]
    fn test_sample() {
        assert_eq!(power(122, 79, 57), -5);
        assert_eq!(power(217, 196, 39), 0);
        assert_eq!(power(101, 153, 71), 4);
    }
}
