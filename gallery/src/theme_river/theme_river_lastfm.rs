use charming::{component::SingleAxis, element::Label, series::ThemeRiver, Chart};

pub fn chart() -> Chart {
    let raw_data = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![
            0, 49, 67, 16, 0, 19, 19, 0, 0, 1, 10, 5, 6, 1, 1, 0, 25, 0, 0, 0,
        ],
        vec![
            0, 6, 3, 34, 0, 16, 1, 0, 0, 1, 6, 0, 1, 56, 0, 2, 0, 2, 0, 0,
        ],
        vec![
            0, 8, 13, 15, 0, 12, 23, 0, 0, 1, 0, 1, 0, 0, 6, 0, 0, 1, 0, 1,
        ],
        vec![
            0, 9, 28, 0, 91, 6, 1, 0, 0, 0, 7, 18, 0, 9, 16, 0, 1, 0, 0, 0,
        ],
        vec![
            0, 3, 42, 36, 21, 0, 1, 0, 0, 0, 0, 16, 30, 1, 4, 62, 55, 1, 0, 0,
        ],
        vec![
            0, 7, 13, 12, 64, 5, 0, 0, 0, 8, 17, 3, 72, 1, 1, 53, 1, 0, 0, 0,
        ],
        vec![
            1, 14, 13, 7, 8, 8, 7, 0, 1, 1, 14, 6, 44, 8, 7, 17, 21, 1, 0, 0,
        ],
        vec![
            0, 6, 14, 2, 14, 1, 0, 0, 0, 0, 2, 2, 7, 15, 6, 3, 0, 0, 0, 0,
        ],
        vec![
            0, 9, 11, 3, 0, 8, 0, 0, 14, 2, 0, 1, 1, 1, 7, 13, 2, 1, 0, 0,
        ],
        vec![
            0, 7, 5, 10, 8, 21, 0, 0, 130, 1, 2, 18, 6, 1, 5, 1, 4, 1, 0, 7,
        ],
        vec![0, 2, 15, 1, 5, 5, 0, 0, 6, 0, 0, 0, 4, 1, 3, 1, 17, 0, 0, 9],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![
            6, 27, 26, 1, 0, 11, 1, 0, 0, 0, 1, 1, 2, 0, 0, 9, 1, 0, 0, 0,
        ],
        vec![
            31, 81, 11, 6, 11, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 3, 14, 0, 0, 12,
        ],
        vec![
            19, 53, 6, 20, 0, 4, 37, 0, 30, 86, 43, 7, 5, 7, 17, 19, 2, 0, 0, 5,
        ],
        vec![
            0, 22, 14, 6, 10, 24, 18, 0, 13, 21, 5, 2, 13, 35, 7, 1, 8, 0, 0, 1,
        ],
        vec![
            0, 56, 5, 0, 0, 0, 0, 0, 7, 24, 0, 17, 7, 0, 0, 3, 0, 0, 0, 8,
        ],
        vec![
            18, 29, 3, 6, 11, 0, 15, 0, 12, 42, 37, 0, 3, 3, 13, 8, 0, 0, 0, 1,
        ],
        vec![
            32, 39, 37, 3, 33, 21, 6, 0, 4, 17, 0, 11, 8, 2, 3, 0, 23, 0, 0, 17,
        ],
        vec![
            72, 15, 28, 0, 0, 0, 0, 0, 1, 3, 0, 35, 0, 9, 17, 1, 9, 1, 0, 8,
        ],
        vec![
            11, 15, 4, 2, 0, 18, 10, 0, 20, 3, 0, 0, 2, 0, 0, 2, 2, 30, 0, 0,
        ],
        vec![
            14, 29, 19, 3, 2, 17, 13, 0, 7, 12, 2, 0, 6, 0, 0, 1, 1, 34, 0, 1,
        ],
        vec![
            1, 1, 7, 6, 1, 1, 15, 1, 1, 2, 1, 3, 1, 1, 9, 1, 1, 25, 1, 72,
        ],
    ];

    let labels = vec![
        "The Sea and Cake",
        "Andrew Bird",
        "Laura Veirs",
        "Brian Eno",
        "Christopher Willits",
        "Wilco",
        "Edgar Meyer",
        "BÃ©la Fleck",
        "Fleet Foxes",
        "Kings of Convenience",
        "Brett Dennen",
        "Psapp",
        "The Bad Plus",
        "Feist",
        "Battles",
        "Avishai Cohen",
        "Rachael Yamagata",
        "Norah Jones",
        "BÃ©la Fleck and the Flecktones",
        "Joshua Redman",
    ];

    let iter = labels.iter().zip(raw_data.iter());
    let data = iter
        .map(|(name, data)| {
            data.iter()
                .enumerate()
                .map(|(idx, v)| (idx as f64, *v, *name))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    Chart::new()
        .single_axis(SingleAxis::new().max("dataMax"))
        .series(ThemeRiver::new().label(Label::new().show(false)).data(data))
}
