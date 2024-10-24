#[cfg(test)]
mod tests {

    use charming::{df, dz};
    
    #[test]
    fn dz_test1() {
        // Set up test data
        let d1 = vec![44056, 13334];
        let d2 = vec![81.8, 76.9];
        let d3 = vec![23968973, 1376048943];
        let d4 = vec!["Australia", "China"];
        let d5 = vec![2015, 2015];

        // expected output
        let df_out = df![
            [44056, 81.8, 23968973, "Australia", 2015],
            [13334, 76.9, 1376048943, "China", 2015]
        ];

        // Call the macro
        let dz_out = dz!(d1, d2, d3, d4, d5);

        // Assert expected output
        assert_eq!(dz_out, df_out);
    }
}
