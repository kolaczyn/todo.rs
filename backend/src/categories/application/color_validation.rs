const NICE_COLOR: [&'static str; 8] = [
    "#584B53", "#D6E3F8", "#FEF5EF", "#709775", "#2E4052", "#FFC857", "#412234", "#9F9FED",
];

pub fn is_color_nice(color_in_hex: &String) -> bool {
    NICE_COLOR.contains(&color_in_hex.as_str())
}
