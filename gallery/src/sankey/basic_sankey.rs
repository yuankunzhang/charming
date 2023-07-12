use charming::{
    element::{Emphasis, EmphasisFocus},
    series::Sankey,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new().series(
        Sankey::new()
            .emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
            .data(vec!["a", "b", "a1", "a2", "b1", "c"])
            .links(vec![
                ("a", "a1", 5),
                ("a", "a2", 3),
                ("b", "b1", 8),
                ("a", "b1", 3),
                ("b1", "a1", 1),
                ("b1", "c", 2),
            ]),
    )
}
