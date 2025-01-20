use bevy::prelude::*;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

/// To get a particular color, get the color from the hashmap with a key of the PlotColor enum.
/// Then get the shade of this color from the Vec of colors, the higher the index the darker the shade.
pub fn make_color_palette() -> HashMap<PlotColor, Vec<Color>> {
    let gray = vec!["d4d2dd", "b4b3b9", "aaa9b1", "9f9ea4", "66656a", "59585e"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let black = vec!["38373c", "323337", "49484d", "323136", "1c1c1c", "111111"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let light_pink = vec!["f1b8bf", "d08693", "ecbbbf", "f2b9bf", "febdc5", "df9ea6"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let pink = vec!["f05285", "f9558a", "e74479", "f85187", "e9467d", "ca1950"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let violet = vec!["9e6ea2", "94639a", "64356c", "9d71a2", "714576", "4b2451"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let blue = vec!["5197ca", "4a8dc1", "4285ba", "226599", "3b6d90", "1c567e"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let green = vec!["afce92", "a2c986", "b6dd9a", "8eb274", "8eb274", "366821"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let salmon = vec!["f96960", "e6564d", "fc655e", "df4442", "dc4846", "bb2727"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let orange = vec!["f8ae6d", "ffaf6a", "e78347", "f28e50", "e16f3b", "cb6229"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let latte = vec!["dbb993", "e5c49b", "dbbb92", "d1ae86", "be9b71", "b38e62"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let cream = vec!["f7efe4", "f6edde", "f5e9d9", "f2e6d8", "e9dbce", "e8dccc"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

    let yellow = vec!["fcd402", "fcd305", "fad008", "efc000", "f9c907", "d8a600"]
        .iter()
        .map(
            // to hex
            |h| Color::Srgba(Srgba::hex(h).unwrap()),
        )
        .collect::<Vec<Color>>();

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

    colors
}
