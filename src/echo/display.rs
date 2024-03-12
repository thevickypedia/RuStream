#[allow(dead_code)]
pub struct Format {
    pub bold: &'static str,
    pub underline: &'static str,
    pub italic: &'static str,
}

pub struct Colors {
    pub green: &'static str,
    pub yellow: &'static str,
    pub red: &'static str,
    pub end: &'static str,
    pub light_green: &'static str,
}

#[allow(dead_code)]
pub fn format() -> Format {
    let bold = "\x1B[1m";
    let underline = "\x1B[4m";
    let italic = "\x1B[3m";
    Format { bold, underline, italic }
}

pub fn colors() -> Colors {
    let green = "\x1B[92m";
    let yellow = "\x1B[93m";
    let red = "\x1B[91m";
    let end = "\x1B[0m";
    let light_green = "\x1B[32m";
    Colors { green, yellow, red, end, light_green }
}
