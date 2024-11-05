use charming::{
    component::{Axis, Grid, Legend, Title},
    df, dz,
    element::{
        Color, ColorStop, Emphasis, EmphasisFocus, Formatter, ItemStyle, Label, LabelPosition,
        LineStyle, LineStyleType, SplitLine,
    },
    series::Scatter,
    Chart,
    HtmlRenderer,
};

fn main() {
    let chartx = chart();
    let mut renderer = HtmlRenderer::new("My Chart", 1000, 800);
    renderer.save(&chartx, "./chart.html").unwrap();

}

pub fn chart() -> Chart {
    /*
    let data0 = df![
        [28604, 77, 17096869, "Australia", 1990],
        [31163, 77.4, 27662440, "Canada", 1990],
        [1516, 68, 1154605773, "China", 1990],
        [13670, 74.7, 10582082, "Cuba", 1990],
        [28599, 75, 4986705, "Finland", 1990],
        [29476, 77.1, 56943299, "France", 1990],
        [31476, 75.4, 78958237, "Germany", 1990],
        [28666, 78.1, 254830, "Iceland", 1990],
        [1777, 57.7, 870601776, "India", 1990],
        [29550, 79.1, 122249285, "Japan", 1990],
        [2076, 67.9, 20194354, "North Korea", 1990],
        [12087, 72, 42972254, "South Korea", 1990],
        [24021, 75.4, 3397534, "New Zealand", 1990],
        [43296, 76.8, 4240375, "Norway", 1990],
        [10088, 70.8, 38195258, "Poland", 1990],
        [19349, 69.6, 147568552, "Russia", 1990],
        [10670, 67.3, 53994605, "Turkey", 1990],
        [26424, 75.7, 57110117, "United Kingdom", 1990],
        [37062, 75.4, 252847810, "United States", 1990]
    ];
    let data1 = df![
        [44056, 81.8, 23968973, "Australia", 2015],
        [43294, 81.7, 35939927, "Canada", 2015],
        [13334, 76.9, 1376048943, "China", 2015],
        [21291, 78.5, 11389562, "Cuba", 2015],
        [38923, 80.8, 5503457, "Finland", 2015],
        [37599, 81.9, 64395345, "France", 2015],
        [44053, 81.1, 80688545, "Germany", 2015],
        [42182, 82.8, 329425, "Iceland", 2015],
        [5903, 66.8, 1311050527, "India", 2015],
        [36162, 83.5, 126573481, "Japan", 2015],
        [1390, 71.4, 25155317, "North Korea", 2015],
        [34644, 80.7, 50293439, "South Korea", 2015],
        [34186, 80.6, 4528526, "New Zealand", 2015],
        [64304, 81.6, 5210967, "Norway", 2015],
        [24787, 77.3, 38611794, "Poland", 2015],
        [23038, 73.13, 143456918, "Russia", 2015],
        [19360, 76.5, 78665830, "Turkey", 2015],
        [38225, 81.4, 64715810, "United Kingdom", 2015],
        [53354, 79.1, 321773631, "United States", 2015]
    ];
    */
    let data0_gdp = vec![28604, 31163, 1516, 13670, 28599, 29476, 31476, 28666, 1777, 29550, 2076, 12087, 24021, 43296, 10088, 19349, 10670, 26424, 37062];
    let data0_expectancy = vec![77.0, 77.4, 68.0, 74.7, 75.0, 77.1, 75.4, 78.1, 57.7, 79.1, 67.9, 72.0, 75.4, 76.8, 70.8, 69.6, 67.3, 75.7, 75.4];
    let data0_population = vec![17096869, 27662440, 1154605773, 10582082, 4986705, 56943299, 78958237, 254830, 870601776, 122249285, 20194354, 42972254, 3397534, 4240375, 38195258, 147568552, 53994605, 57110117, 252847810];
    let data0_country = vec!["Australia", "Canada", "China", "Cuba", "Finland", "France", "Germany", "Iceland", "India", "Japan", "North Korea", "South Korea", "New Zealand", "Norway", "Poland", "Russia", "Turkey", "United Kingdom", "United States"];
    let data0_year = vec![1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990, 1990];

    let data1_gdp = vec![44056, 43294, 13334, 21291, 38923, 37599, 44053, 42182, 5903, 36162, 1390, 34644, 34186, 64304, 24787, 23038, 19360, 38225, 53354];
    let data1_expectancy = vec![81.8, 81.7, 76.9, 78.5, 80.8, 81.9, 81.1, 82.8, 66.8, 83.5, 71.4, 80.7, 80.6, 81.6, 77.3, 73.13, 76.5, 81.4, 79.1];
    let data1_population = vec![23968973, 35939927, 1376048943, 11389562, 5503457, 64395345, 80688545, 329425, 1311050527, 126573481, 25155317, 50293439, 4528526, 5210967, 38611794, 143456918, 78665830, 64715810, 321773631];
    let data1_country = vec!["Australia", "Canada", "China", "Cuba", "Finland", "France", "Germany", "Iceland", "India", "Japan", "North Korea", "South Korea", "New Zealand", "Norway", "Poland", "Russia", "Turkey", "United Kingdom", "United States"];
    let data1_year = vec![2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015, 2015];

    let data0 = dz!(data0_gdp, data0_expectancy, data0_population, data0_country, data0_year);
    let data1 = dz!(data1_gdp, data1_expectancy, data1_population, data1_country, data1_year);
    
    Chart::new()
        .title(
            Title::new()
                .text("Life Expectancy and GDP by Country")
                .left("5%")
                .top("3%"),
        )
        .legend(
            Legend::new()
                .right("10%")
                .top("3%")
                .data(vec!["1990", "2015"]),
        )
        .grid(Grid::new().left("8%").top("10%"))
        .background_color(Color::RadialGradient {
            x: 0.3,
            y: 0.3,
            r: 0.8,
            color_stops: vec![ColorStop::new(0, "#f7f8fa"), ColorStop::new(1, "#cdd0d5")],
        })
        .x_axis(
            Axis::new().split_line(
                SplitLine::new().line_style(LineStyle::new().type_(LineStyleType::Dashed)),
            ),
        )
        .y_axis(
            Axis::new()
                .split_line(
                    SplitLine::new().line_style(LineStyle::new().type_(LineStyleType::Dashed)),
                )
                .scale(true),
        )
        .series(
            Scatter::new()
                .name("1990")
                .data(data0)
                .symbol_size("function (data) { return Math.sqrt(data[2]) / 5e2; }")
                .emphasis(
                    Emphasis::new().focus(EmphasisFocus::Series).label(
                        Label::new()
                            .show(true)
                            .formatter(Formatter::Function(
                                "function (param) { return param.data[3]; }".into(),
                            ))
                            .position(LabelPosition::Top),
                    ),
                )
                .item_style(
                    ItemStyle::new()
                        .shadow_blur(10)
                        .shadow_color("rgba(120, 36, 50, 0.5)")
                        .shadow_offset_y(5)
                        .color(Color::RadialGradient {
                            x: 0.4,
                            y: 0.3,
                            r: 1.,
                            color_stops: vec![
                                ColorStop::new(0, "rgb(251, 118, 123)"),
                                ColorStop::new(1, "rgb(204, 46, 72)"),
                            ],
                        }),
                ),
        )
        .series(
            Scatter::new()
                .name("2015")
                .data(data1)
                .symbol_size("function (data) { return Math.sqrt(data[2]) / 5e2; }")
                .emphasis(
                    Emphasis::new().focus(EmphasisFocus::Series).label(
                        Label::new()
                            .show(true)
                            .formatter(Formatter::Function(
                                "function (param) { return param.data[3]; }".into(),
                            ))
                            .position(LabelPosition::Top),
                    ),
                )
                .item_style(
                    ItemStyle::new()
                        .shadow_blur(10)
                        .shadow_color("rgba(25, 100, 150, 0.5)")
                        .shadow_offset_y(5)
                        .color(Color::RadialGradient {
                            x: 0.4,
                            y: 0.3,
                            r: 1.,
                            color_stops: vec![
                                ColorStop::new(0, "rgb(129, 227, 238)"),
                                ColorStop::new(1, "rgb(25, 183, 207)"),
                            ],
                        }),
                ),
        )
}
