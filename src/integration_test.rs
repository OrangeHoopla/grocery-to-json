

#[cfg(test)]
mod tests {


    #[test]
    fn test_add() {
        assert_eq!(4,4);
    }

    #[test]
    fn test_bad_add() {
        let mut x = 0.0;
        for i in 0..1000 {
            x += (i as f64) * 0.12;
        }
        println!("{}",x);
    }
}
