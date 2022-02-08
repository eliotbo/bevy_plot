fn make_color_palette(

    let gray = vec![
        "d4d2dd",
        "b4b3b9",
        "aaa9b1",
        "9f9ea4",
        "66656a",
        "59585e",
    ]

    let black = vec![
        "38373c",
        "323337",
        "49484d",
        "323136",
        "1c1c1c",
        "111111",
    ];

    let light_pink = vec![
        "f1b8bf",
        "d08693",
        "ecbbbf",
        "f2b9bf",
        "febdc5",
        "df9ea6",
    ];


    let pink = vec![
        "f05285",
        "f9558a",
        "e74479",
        "f85187",
        "e9467d",
        "ca1950",
    ];

    let violet = vec![
        "9e6ea2",
        "94639a",
        "64356c",
        "9d71a2",
        "714576",
        "4b2451",
    ];

    let blue = vec![
        "5197ca",
        "4a8dc1",
        "4285ba",
        "226599",
        "3b6d90",
        "1c567e",

    ];

    let green = vec![
        "afce92",
        "a2c986",
        "b6dd9a",
        "8eb274",
        "8eb274",
        "366821",
    ];

    let salmon = vec![
        "f96960",
        "e6564d",
        "fc655e",
        "df4442",
        "dc4846",
        "bb2727",
    ];

    let orange = vec![
        "f8ae6d",
        "ffaf6a",
        "e78347",
        "f28e50",
        "e16f3b",
        "cb6229",
    ];

    let latte = vec![
        "dbb993",
        "e5c49b",
        "dbbb92",
        "d1ae86",
        "be9b71",
        "b38e62",
    ];

    let cream = vec![
        "f7efe4",
        "f6edde",
        "f5e9d9",
        "f2e6d8",
        "e9dbce",
        "e8dccc",
    ];

    let yellow = vec![
        "fcd402",
        "fcd305",
        "fad008",
        "efc000",
        "f9c907",
        "d8a600",
    ]
    .iter().map( 
        // to hex
        |h| Color::hex(h).unwrap()
    ).collect::<Vec<Color>>;



    let mut colors = HashMap::new();
    
    colors.insert(PlotColor::Gray, gray);
    colors.insert(PlotColor::Black, black);
    colors.insert(PlotColor::LightPink, light_pink);
    colors.insert(PlotColor::Pink, pink);
    colors.insert(PlotColor::Violet, violet);
    colors.insert(PlotColor::Blue, blue);
    colors.insert(PlotColor::Green, green);
    colors.insert(PlotColor::Salmon, salmon);
    colors.insert(PlotColor::Orange, orange);
    colors.insert(PlotColor::Latte, latte);
    colors.insert(PlotColor::Cream, cream);
    colors.insert(PlotColor::Yellow, yellow);


}

pub enum PlotColor {
    Gray,
    Black,
    LightPink,
    Pink,
    Violet,
    Blue,
    Green,
    Salmon,
    Orange,
    Latte,
    Cream,
    Yellow,
}