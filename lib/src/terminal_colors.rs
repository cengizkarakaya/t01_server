// ANSI / Xterm terminal style constants.
// Renkler foreground yani yazı rengi içindir.
// Örnek: println!("{C208_DARK_ORANGE}Merhaba{RESET}");

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const ITALIC: &str = "\x1b[3m";
pub const DIM: &str = "\x1b[2m";
pub const BRIGHT: &str = "\x1b[0;90m";
pub const BLINK: &str = "\x1b[5m";
pub const CLEAR: &str = "\x1b[2J\x1b[1;1H";

pub const CLEAR_SCREEN: &str = "\x1b[2J";
pub const MOVE_CURSOR_HOME: &str = "\x1b[H";
pub const HIDE_CURSOR: &str = "\x1b[?25l";
pub const SHOW_CURSOR: &str = "\x1b[?25h";

/// Verilen Xterm sıra numarasından foreground escape sequence üretir.
/// Sabit `&str` gerektiğinde aşağıdaki `C000_...` sabitlerini kullan.
pub fn fg256(n: u8) -> String {
    format!("\x1b[38;5;{n}m")
}

/// Verilen Xterm sıra numarasından background escape sequence üretir.
pub fn bg256(n: u8) -> String {
    format!("\x1b[48;5;{n}m")
}

// 256 Xterm renkleri.
// 0-15 arası system renkleridir ve isimlerinde `_SYSTEM` vardır.

/// Xterm 0 - Black (SYSTEM) - #000000 - rgb(0,0,0)
pub const C000_BLACK_SYSTEM: &str = "\x1b[38;5;0m";

/// Xterm 1 - Maroon (SYSTEM) - #800000 - rgb(128,0,0)
pub const C001_MAROON_SYSTEM: &str = "\x1b[38;5;1m";

/// Xterm 2 - Green (SYSTEM) - #008000 - rgb(0,128,0)
pub const C002_GREEN_SYSTEM: &str = "\x1b[38;5;2m";

/// Xterm 3 - Olive (SYSTEM) - #808000 - rgb(128,128,0)
pub const C003_OLIVE_SYSTEM: &str = "\x1b[38;5;3m";

/// Xterm 4 - Navy (SYSTEM) - #000080 - rgb(0,0,128)
pub const C004_NAVY_SYSTEM: &str = "\x1b[38;5;4m";

/// Xterm 5 - Purple (SYSTEM) - #800080 - rgb(128,0,128)
pub const C005_PURPLE_SYSTEM: &str = "\x1b[38;5;5m";

/// Xterm 6 - Teal (SYSTEM) - #008080 - rgb(0,128,128)
pub const C006_TEAL_SYSTEM: &str = "\x1b[38;5;6m";

/// Xterm 7 - Silver (SYSTEM) - #c0c0c0 - rgb(192,192,192)
pub const C007_SILVER_SYSTEM: &str = "\x1b[38;5;7m";

/// Xterm 8 - Grey (SYSTEM) - #808080 - rgb(128,128,128)
pub const C008_GREY_SYSTEM: &str = "\x1b[38;5;8m";

/// Xterm 9 - Red (SYSTEM) - #ff0000 - rgb(255,0,0)
pub const C009_RED_SYSTEM: &str = "\x1b[38;5;9m";

/// Xterm 10 - Lime (SYSTEM) - #00ff00 - rgb(0,255,0)
pub const C010_LIME_SYSTEM: &str = "\x1b[38;5;10m";

/// Xterm 11 - Yellow (SYSTEM) - #ffff00 - rgb(255,255,0)
pub const C011_YELLOW_SYSTEM: &str = "\x1b[38;5;11m";

/// Xterm 12 - Blue (SYSTEM) - #0000ff - rgb(0,0,255)
pub const C012_BLUE_SYSTEM: &str = "\x1b[38;5;12m";

/// Xterm 13 - Fuchsia (SYSTEM) - #ff00ff - rgb(255,0,255)
pub const C013_FUCHSIA_SYSTEM: &str = "\x1b[38;5;13m";

/// Xterm 14 - Aqua (SYSTEM) - #00ffff - rgb(0,255,255)
pub const C014_AQUA_SYSTEM: &str = "\x1b[38;5;14m";

/// Xterm 15 - White (SYSTEM) - #ffffff - rgb(255,255,255)
pub const C015_WHITE_SYSTEM: &str = "\x1b[38;5;15m";

/// Xterm 16 - Grey0 - #000000 - rgb(0,0,0)
pub const C016_GREY0: &str = "\x1b[38;5;16m";

/// Xterm 17 - NavyBlue - #00005f - rgb(0,0,95)
pub const C017_NAVY_BLUE: &str = "\x1b[38;5;17m";

/// Xterm 18 - DarkBlue - #000087 - rgb(0,0,135)
pub const C018_DARK_BLUE: &str = "\x1b[38;5;18m";

/// Xterm 19 - Blue3 - #0000af - rgb(0,0,175)
pub const C019_BLUE3: &str = "\x1b[38;5;19m";

/// Xterm 20 - Blue3 - #0000d7 - rgb(0,0,215)
pub const C020_BLUE3: &str = "\x1b[38;5;20m";

/// Xterm 21 - Blue1 - #0000ff - rgb(0,0,255)
pub const C021_BLUE1: &str = "\x1b[38;5;21m";

/// Xterm 22 - DarkGreen - #005f00 - rgb(0,95,0)
pub const C022_DARK_GREEN: &str = "\x1b[38;5;22m";

/// Xterm 23 - DeepSkyBlue4 - #005f5f - rgb(0,95,95)
pub const C023_DEEP_SKY_BLUE4: &str = "\x1b[38;5;23m";

/// Xterm 24 - DeepSkyBlue4 - #005f87 - rgb(0,95,135)
pub const C024_DEEP_SKY_BLUE4: &str = "\x1b[38;5;24m";

/// Xterm 25 - DeepSkyBlue4 - #005faf - rgb(0,95,175)
pub const C025_DEEP_SKY_BLUE4: &str = "\x1b[38;5;25m";

/// Xterm 26 - DodgerBlue3 - #005fd7 - rgb(0,95,215)
pub const C026_DODGER_BLUE3: &str = "\x1b[38;5;26m";

/// Xterm 27 - DodgerBlue2 - #005fff - rgb(0,95,255)
pub const C027_DODGER_BLUE2: &str = "\x1b[38;5;27m";

/// Xterm 28 - Green4 - #008700 - rgb(0,135,0)
pub const C028_GREEN4: &str = "\x1b[38;5;28m";

/// Xterm 29 - SpringGreen4 - #00875f - rgb(0,135,95)
pub const C029_SPRING_GREEN4: &str = "\x1b[38;5;29m";

/// Xterm 30 - Turquoise4 - #008787 - rgb(0,135,135)
pub const C030_TURQUOISE4: &str = "\x1b[38;5;30m";

/// Xterm 31 - DeepSkyBlue3 - #0087af - rgb(0,135,175)
pub const C031_DEEP_SKY_BLUE3: &str = "\x1b[38;5;31m";

/// Xterm 32 - DeepSkyBlue3 - #0087d7 - rgb(0,135,215)
pub const C032_DEEP_SKY_BLUE3: &str = "\x1b[38;5;32m";

/// Xterm 33 - DodgerBlue1 - #0087ff - rgb(0,135,255)
pub const C033_DODGER_BLUE1: &str = "\x1b[38;5;33m";

/// Xterm 34 - Green3 - #00af00 - rgb(0,175,0)
pub const C034_GREEN3: &str = "\x1b[38;5;34m";

/// Xterm 35 - SpringGreen3 - #00af5f - rgb(0,175,95)
pub const C035_SPRING_GREEN3: &str = "\x1b[38;5;35m";

/// Xterm 36 - DarkCyan - #00af87 - rgb(0,175,135)
pub const C036_DARK_CYAN: &str = "\x1b[38;5;36m";

/// Xterm 37 - LightSeaGreen - #00afaf - rgb(0,175,175)
pub const C037_LIGHT_SEA_GREEN: &str = "\x1b[38;5;37m";

/// Xterm 38 - DeepSkyBlue2 - #00afd7 - rgb(0,175,215)
pub const C038_DEEP_SKY_BLUE2: &str = "\x1b[38;5;38m";

/// Xterm 39 - DeepSkyBlue1 - #00afff - rgb(0,175,255)
pub const C039_DEEP_SKY_BLUE1: &str = "\x1b[38;5;39m";

/// Xterm 40 - Green3 - #00d700 - rgb(0,215,0)
pub const C040_GREEN3: &str = "\x1b[38;5;40m";

/// Xterm 41 - SpringGreen3 - #00d75f - rgb(0,215,95)
pub const C041_SPRING_GREEN3: &str = "\x1b[38;5;41m";

/// Xterm 42 - SpringGreen2 - #00d787 - rgb(0,215,135)
pub const C042_SPRING_GREEN2: &str = "\x1b[38;5;42m";

/// Xterm 43 - Cyan3 - #00d7af - rgb(0,215,175)
pub const C043_CYAN3: &str = "\x1b[38;5;43m";

/// Xterm 44 - DarkTurquoise - #00d7d7 - rgb(0,215,215)
pub const C044_DARK_TURQUOISE: &str = "\x1b[38;5;44m";

/// Xterm 45 - Turquoise2 - #00d7ff - rgb(0,215,255)
pub const C045_TURQUOISE2: &str = "\x1b[38;5;45m";

/// Xterm 46 - Green1 - #00ff00 - rgb(0,255,0)
pub const C046_GREEN1: &str = "\x1b[38;5;46m";

/// Xterm 47 - SpringGreen2 - #00ff5f - rgb(0,255,95)
pub const C047_SPRING_GREEN2: &str = "\x1b[38;5;47m";

/// Xterm 48 - SpringGreen1 - #00ff87 - rgb(0,255,135)
pub const C048_SPRING_GREEN1: &str = "\x1b[38;5;48m";

/// Xterm 49 - MediumSpringGreen - #00ffaf - rgb(0,255,175)
pub const C049_MEDIUM_SPRING_GREEN: &str = "\x1b[38;5;49m";

/// Xterm 50 - Cyan2 - #00ffd7 - rgb(0,255,215)
pub const C050_CYAN2: &str = "\x1b[38;5;50m";

/// Xterm 51 - Cyan1 - #00ffff - rgb(0,255,255)
pub const C051_CYAN1: &str = "\x1b[38;5;51m";

/// Xterm 52 - DarkRed - #5f0000 - rgb(95,0,0)
pub const C052_DARK_RED: &str = "\x1b[38;5;52m";

/// Xterm 53 - DeepPink4 - #5f005f - rgb(95,0,95)
pub const C053_DEEP_PINK4: &str = "\x1b[38;5;53m";

/// Xterm 54 - Purple4 - #5f0087 - rgb(95,0,135)
pub const C054_PURPLE4: &str = "\x1b[38;5;54m";

/// Xterm 55 - Purple4 - #5f00af - rgb(95,0,175)
pub const C055_PURPLE4: &str = "\x1b[38;5;55m";

/// Xterm 56 - Purple3 - #5f00d7 - rgb(95,0,215)
pub const C056_PURPLE3: &str = "\x1b[38;5;56m";

/// Xterm 57 - BlueViolet - #5f00ff - rgb(95,0,255)
pub const C057_BLUE_VIOLET: &str = "\x1b[38;5;57m";

/// Xterm 58 - Orange4 - #5f5f00 - rgb(95,95,0)
pub const C058_ORANGE4: &str = "\x1b[38;5;58m";

/// Xterm 59 - Grey37 - #5f5f5f - rgb(95,95,95)
pub const C059_GREY37: &str = "\x1b[38;5;59m";

/// Xterm 60 - MediumPurple4 - #5f5f87 - rgb(95,95,135)
pub const C060_MEDIUM_PURPLE4: &str = "\x1b[38;5;60m";

/// Xterm 61 - SlateBlue3 - #5f5faf - rgb(95,95,175)
pub const C061_SLATE_BLUE3: &str = "\x1b[38;5;61m";

/// Xterm 62 - SlateBlue3 - #5f5fd7 - rgb(95,95,215)
pub const C062_SLATE_BLUE3: &str = "\x1b[38;5;62m";

/// Xterm 63 - RoyalBlue1 - #5f5fff - rgb(95,95,255)
pub const C063_ROYAL_BLUE1: &str = "\x1b[38;5;63m";

/// Xterm 64 - Chartreuse4 - #5f8700 - rgb(95,135,0)
pub const C064_CHARTREUSE4: &str = "\x1b[38;5;64m";

/// Xterm 65 - DarkSeaGreen4 - #5f875f - rgb(95,135,95)
pub const C065_DARK_SEA_GREEN4: &str = "\x1b[38;5;65m";

/// Xterm 66 - PaleTurquoise4 - #5f8787 - rgb(95,135,135)
pub const C066_PALE_TURQUOISE4: &str = "\x1b[38;5;66m";

/// Xterm 67 - SteelBlue - #5f87af - rgb(95,135,175)
pub const C067_STEEL_BLUE: &str = "\x1b[38;5;67m";

/// Xterm 68 - SteelBlue3 - #5f87d7 - rgb(95,135,215)
pub const C068_STEEL_BLUE3: &str = "\x1b[38;5;68m";

/// Xterm 69 - CornflowerBlue - #5f87ff - rgb(95,135,255)
pub const C069_CORNFLOWER_BLUE: &str = "\x1b[38;5;69m";

/// Xterm 70 - Chartreuse3 - #5faf00 - rgb(95,175,0)
pub const C070_CHARTREUSE3: &str = "\x1b[38;5;70m";

/// Xterm 71 - DarkSeaGreen4 - #5faf5f - rgb(95,175,95)
pub const C071_DARK_SEA_GREEN4: &str = "\x1b[38;5;71m";

/// Xterm 72 - CadetBlue - #5faf87 - rgb(95,175,135)
pub const C072_CADET_BLUE: &str = "\x1b[38;5;72m";

/// Xterm 73 - CadetBlue - #5fafaf - rgb(95,175,175)
pub const C073_CADET_BLUE: &str = "\x1b[38;5;73m";

/// Xterm 74 - SkyBlue3 - #5fafd7 - rgb(95,175,215)
pub const C074_SKY_BLUE3: &str = "\x1b[38;5;74m";

/// Xterm 75 - SteelBlue1 - #5fafff - rgb(95,175,255)
pub const C075_STEEL_BLUE1: &str = "\x1b[38;5;75m";

/// Xterm 76 - Chartreuse3 - #5fd700 - rgb(95,215,0)
pub const C076_CHARTREUSE3: &str = "\x1b[38;5;76m";

/// Xterm 77 - PaleGreen3 - #5fd75f - rgb(95,215,95)
pub const C077_PALE_GREEN3: &str = "\x1b[38;5;77m";

/// Xterm 78 - SeaGreen3 - #5fd787 - rgb(95,215,135)
pub const C078_SEA_GREEN3: &str = "\x1b[38;5;78m";

/// Xterm 79 - Aquamarine3 - #5fd7af - rgb(95,215,175)
pub const C079_AQUAMARINE3: &str = "\x1b[38;5;79m";

/// Xterm 80 - MediumTurquoise - #5fd7d7 - rgb(95,215,215)
pub const C080_MEDIUM_TURQUOISE: &str = "\x1b[38;5;80m";

/// Xterm 81 - SteelBlue1 - #5fd7ff - rgb(95,215,255)
pub const C081_STEEL_BLUE1: &str = "\x1b[38;5;81m";

/// Xterm 82 - Chartreuse2 - #5fff00 - rgb(95,255,0)
pub const C082_CHARTREUSE2: &str = "\x1b[38;5;82m";

/// Xterm 83 - SeaGreen2 - #5fff5f - rgb(95,255,95)
pub const C083_SEA_GREEN2: &str = "\x1b[38;5;83m";

/// Xterm 84 - SeaGreen1 - #5fff87 - rgb(95,255,135)
pub const C084_SEA_GREEN1: &str = "\x1b[38;5;84m";

/// Xterm 85 - SeaGreen1 - #5fffaf - rgb(95,255,175)
pub const C085_SEA_GREEN1: &str = "\x1b[38;5;85m";

/// Xterm 86 - Aquamarine1 - #5fffd7 - rgb(95,255,215)
pub const C086_AQUAMARINE1: &str = "\x1b[38;5;86m";

/// Xterm 87 - DarkSlateGray2 - #5fffff - rgb(95,255,255)
pub const C087_DARK_SLATE_GRAY2: &str = "\x1b[38;5;87m";

/// Xterm 88 - DarkRed - #870000 - rgb(135,0,0)
pub const C088_DARK_RED: &str = "\x1b[38;5;88m";

/// Xterm 89 - DeepPink4 - #87005f - rgb(135,0,95)
pub const C089_DEEP_PINK4: &str = "\x1b[38;5;89m";

/// Xterm 90 - DarkMagenta - #870087 - rgb(135,0,135)
pub const C090_DARK_MAGENTA: &str = "\x1b[38;5;90m";

/// Xterm 91 - DarkMagenta - #8700af - rgb(135,0,175)
pub const C091_DARK_MAGENTA: &str = "\x1b[38;5;91m";

/// Xterm 92 - DarkViolet - #8700d7 - rgb(135,0,215)
pub const C092_DARK_VIOLET: &str = "\x1b[38;5;92m";

/// Xterm 93 - Purple - #8700ff - rgb(135,0,255)
pub const C093_PURPLE: &str = "\x1b[38;5;93m";

/// Xterm 94 - Orange4 - #875f00 - rgb(135,95,0)
pub const C094_ORANGE4: &str = "\x1b[38;5;94m";

/// Xterm 95 - LightPink4 - #875f5f - rgb(135,95,95)
pub const C095_LIGHT_PINK4: &str = "\x1b[38;5;95m";

/// Xterm 96 - Plum4 - #875f87 - rgb(135,95,135)
pub const C096_PLUM4: &str = "\x1b[38;5;96m";

/// Xterm 97 - MediumPurple3 - #875faf - rgb(135,95,175)
pub const C097_MEDIUM_PURPLE3: &str = "\x1b[38;5;97m";

/// Xterm 98 - MediumPurple3 - #875fd7 - rgb(135,95,215)
pub const C098_MEDIUM_PURPLE3: &str = "\x1b[38;5;98m";

/// Xterm 99 - SlateBlue1 - #875fff - rgb(135,95,255)
pub const C099_SLATE_BLUE1: &str = "\x1b[38;5;99m";

/// Xterm 100 - Yellow4 - #878700 - rgb(135,135,0)
pub const C100_YELLOW4: &str = "\x1b[38;5;100m";

/// Xterm 101 - Wheat4 - #87875f - rgb(135,135,95)
pub const C101_WHEAT4: &str = "\x1b[38;5;101m";

/// Xterm 102 - Grey53 - #878787 - rgb(135,135,135)
pub const C102_GREY53: &str = "\x1b[38;5;102m";

/// Xterm 103 - LightSlateGrey - #8787af - rgb(135,135,175)
pub const C103_LIGHT_SLATE_GREY: &str = "\x1b[38;5;103m";

/// Xterm 104 - MediumPurple - #8787d7 - rgb(135,135,215)
pub const C104_MEDIUM_PURPLE: &str = "\x1b[38;5;104m";

/// Xterm 105 - LightSlateBlue - #8787ff - rgb(135,135,255)
pub const C105_LIGHT_SLATE_BLUE: &str = "\x1b[38;5;105m";

/// Xterm 106 - Yellow4 - #87af00 - rgb(135,175,0)
pub const C106_YELLOW4: &str = "\x1b[38;5;106m";

/// Xterm 107 - DarkOliveGreen3 - #87af5f - rgb(135,175,95)
pub const C107_DARK_OLIVE_GREEN3: &str = "\x1b[38;5;107m";

/// Xterm 108 - DarkSeaGreen - #87af87 - rgb(135,175,135)
pub const C108_DARK_SEA_GREEN: &str = "\x1b[38;5;108m";

/// Xterm 109 - LightSkyBlue3 - #87afaf - rgb(135,175,175)
pub const C109_LIGHT_SKY_BLUE3: &str = "\x1b[38;5;109m";

/// Xterm 110 - LightSkyBlue3 - #87afd7 - rgb(135,175,215)
pub const C110_LIGHT_SKY_BLUE3: &str = "\x1b[38;5;110m";

/// Xterm 111 - SkyBlue2 - #87afff - rgb(135,175,255)
pub const C111_SKY_BLUE2: &str = "\x1b[38;5;111m";

/// Xterm 112 - Chartreuse2 - #87d700 - rgb(135,215,0)
pub const C112_CHARTREUSE2: &str = "\x1b[38;5;112m";

/// Xterm 113 - DarkOliveGreen3 - #87d75f - rgb(135,215,95)
pub const C113_DARK_OLIVE_GREEN3: &str = "\x1b[38;5;113m";

/// Xterm 114 - PaleGreen3 - #87d787 - rgb(135,215,135)
pub const C114_PALE_GREEN3: &str = "\x1b[38;5;114m";

/// Xterm 115 - DarkSeaGreen3 - #87d7af - rgb(135,215,175)
pub const C115_DARK_SEA_GREEN3: &str = "\x1b[38;5;115m";

/// Xterm 116 - DarkSlateGray3 - #87d7d7 - rgb(135,215,215)
pub const C116_DARK_SLATE_GRAY3: &str = "\x1b[38;5;116m";

/// Xterm 117 - SkyBlue1 - #87d7ff - rgb(135,215,255)
pub const C117_SKY_BLUE1: &str = "\x1b[38;5;117m";

/// Xterm 118 - Chartreuse1 - #87ff00 - rgb(135,255,0)
pub const C118_CHARTREUSE1: &str = "\x1b[38;5;118m";

/// Xterm 119 - LightGreen - #87ff5f - rgb(135,255,95)
pub const C119_LIGHT_GREEN: &str = "\x1b[38;5;119m";

/// Xterm 120 - LightGreen - #87ff87 - rgb(135,255,135)
pub const C120_LIGHT_GREEN: &str = "\x1b[38;5;120m";

/// Xterm 121 - PaleGreen1 - #87ffaf - rgb(135,255,175)
pub const C121_PALE_GREEN1: &str = "\x1b[38;5;121m";

/// Xterm 122 - Aquamarine1 - #87ffd7 - rgb(135,255,215)
pub const C122_AQUAMARINE1: &str = "\x1b[38;5;122m";

/// Xterm 123 - DarkSlateGray1 - #87ffff - rgb(135,255,255)
pub const C123_DARK_SLATE_GRAY1: &str = "\x1b[38;5;123m";

/// Xterm 124 - Red3 - #af0000 - rgb(175,0,0)
pub const C124_RED3: &str = "\x1b[38;5;124m";

/// Xterm 125 - DeepPink4 - #af005f - rgb(175,0,95)
pub const C125_DEEP_PINK4: &str = "\x1b[38;5;125m";

/// Xterm 126 - MediumVioletRed - #af0087 - rgb(175,0,135)
pub const C126_MEDIUM_VIOLET_RED: &str = "\x1b[38;5;126m";

/// Xterm 127 - Magenta3 - #af00af - rgb(175,0,175)
pub const C127_MAGENTA3: &str = "\x1b[38;5;127m";

/// Xterm 128 - DarkViolet - #af00d7 - rgb(175,0,215)
pub const C128_DARK_VIOLET: &str = "\x1b[38;5;128m";

/// Xterm 129 - Purple - #af00ff - rgb(175,0,255)
pub const C129_PURPLE: &str = "\x1b[38;5;129m";

/// Xterm 130 - DarkOrange3 - #af5f00 - rgb(175,95,0)
pub const C130_DARK_ORANGE3: &str = "\x1b[38;5;130m";

/// Xterm 131 - IndianRed - #af5f5f - rgb(175,95,95)
pub const C131_INDIAN_RED: &str = "\x1b[38;5;131m";

/// Xterm 132 - HotPink3 - #af5f87 - rgb(175,95,135)
pub const C132_HOT_PINK3: &str = "\x1b[38;5;132m";

/// Xterm 133 - MediumOrchid3 - #af5faf - rgb(175,95,175)
pub const C133_MEDIUM_ORCHID3: &str = "\x1b[38;5;133m";

/// Xterm 134 - MediumOrchid - #af5fd7 - rgb(175,95,215)
pub const C134_MEDIUM_ORCHID: &str = "\x1b[38;5;134m";

/// Xterm 135 - MediumPurple2 - #af5fff - rgb(175,95,255)
pub const C135_MEDIUM_PURPLE2: &str = "\x1b[38;5;135m";

/// Xterm 136 - DarkGoldenrod - #af8700 - rgb(175,135,0)
pub const C136_DARK_GOLDENROD: &str = "\x1b[38;5;136m";

/// Xterm 137 - LightSalmon3 - #af875f - rgb(175,135,95)
pub const C137_LIGHT_SALMON3: &str = "\x1b[38;5;137m";

/// Xterm 138 - RosyBrown - #af8787 - rgb(175,135,135)
pub const C138_ROSY_BROWN: &str = "\x1b[38;5;138m";

/// Xterm 139 - Grey63 - #af87af - rgb(175,135,175)
pub const C139_GREY63: &str = "\x1b[38;5;139m";

/// Xterm 140 - MediumPurple2 - #af87d7 - rgb(175,135,215)
pub const C140_MEDIUM_PURPLE2: &str = "\x1b[38;5;140m";

/// Xterm 141 - MediumPurple1 - #af87ff - rgb(175,135,255)
pub const C141_MEDIUM_PURPLE1: &str = "\x1b[38;5;141m";

/// Xterm 142 - Gold3 - #afaf00 - rgb(175,175,0)
pub const C142_GOLD3: &str = "\x1b[38;5;142m";

/// Xterm 143 - DarkKhaki - #afaf5f - rgb(175,175,95)
pub const C143_DARK_KHAKI: &str = "\x1b[38;5;143m";

/// Xterm 144 - NavajoWhite3 - #afaf87 - rgb(175,175,135)
pub const C144_NAVAJO_WHITE3: &str = "\x1b[38;5;144m";

/// Xterm 145 - Grey69 - #afafaf - rgb(175,175,175)
pub const C145_GREY69: &str = "\x1b[38;5;145m";

/// Xterm 146 - LightSteelBlue3 - #afafd7 - rgb(175,175,215)
pub const C146_LIGHT_STEEL_BLUE3: &str = "\x1b[38;5;146m";

/// Xterm 147 - LightSteelBlue - #afafff - rgb(175,175,255)
pub const C147_LIGHT_STEEL_BLUE: &str = "\x1b[38;5;147m";

/// Xterm 148 - Yellow3 - #afd700 - rgb(175,215,0)
pub const C148_YELLOW3: &str = "\x1b[38;5;148m";

/// Xterm 149 - DarkOliveGreen3 - #afd75f - rgb(175,215,95)
pub const C149_DARK_OLIVE_GREEN3: &str = "\x1b[38;5;149m";

/// Xterm 150 - DarkSeaGreen3 - #afd787 - rgb(175,215,135)
pub const C150_DARK_SEA_GREEN3: &str = "\x1b[38;5;150m";

/// Xterm 151 - DarkSeaGreen2 - #afd7af - rgb(175,215,175)
pub const C151_DARK_SEA_GREEN2: &str = "\x1b[38;5;151m";

/// Xterm 152 - LightCyan3 - #afd7d7 - rgb(175,215,215)
pub const C152_LIGHT_CYAN3: &str = "\x1b[38;5;152m";

/// Xterm 153 - LightSkyBlue1 - #afd7ff - rgb(175,215,255)
pub const C153_LIGHT_SKY_BLUE1: &str = "\x1b[38;5;153m";

/// Xterm 154 - GreenYellow - #afff00 - rgb(175,255,0)
pub const C154_GREEN_YELLOW: &str = "\x1b[38;5;154m";

/// Xterm 155 - DarkOliveGreen2 - #afff5f - rgb(175,255,95)
pub const C155_DARK_OLIVE_GREEN2: &str = "\x1b[38;5;155m";

/// Xterm 156 - PaleGreen1 - #afff87 - rgb(175,255,135)
pub const C156_PALE_GREEN1: &str = "\x1b[38;5;156m";

/// Xterm 157 - DarkSeaGreen2 - #afffaf - rgb(175,255,175)
pub const C157_DARK_SEA_GREEN2: &str = "\x1b[38;5;157m";

/// Xterm 158 - DarkSeaGreen1 - #afffd7 - rgb(175,255,215)
pub const C158_DARK_SEA_GREEN1: &str = "\x1b[38;5;158m";

/// Xterm 159 - PaleTurquoise1 - #afffff - rgb(175,255,255)
pub const C159_PALE_TURQUOISE1: &str = "\x1b[38;5;159m";

/// Xterm 160 - Red3 - #d70000 - rgb(215,0,0)
pub const C160_RED3: &str = "\x1b[38;5;160m";

/// Xterm 161 - DeepPink3 - #d7005f - rgb(215,0,95)
pub const C161_DEEP_PINK3: &str = "\x1b[38;5;161m";

/// Xterm 162 - DeepPink3 - #d70087 - rgb(215,0,135)
pub const C162_DEEP_PINK3: &str = "\x1b[38;5;162m";

/// Xterm 163 - Magenta3 - #d700af - rgb(215,0,175)
pub const C163_MAGENTA3: &str = "\x1b[38;5;163m";

/// Xterm 164 - Magenta3 - #d700d7 - rgb(215,0,215)
pub const C164_MAGENTA3: &str = "\x1b[38;5;164m";

/// Xterm 165 - Magenta2 - #d700ff - rgb(215,0,255)
pub const C165_MAGENTA2: &str = "\x1b[38;5;165m";

/// Xterm 166 - DarkOrange3 - #d75f00 - rgb(215,95,0)
pub const C166_DARK_ORANGE3: &str = "\x1b[38;5;166m";

/// Xterm 167 - IndianRed - #d75f5f - rgb(215,95,95)
pub const C167_INDIAN_RED: &str = "\x1b[38;5;167m";

/// Xterm 168 - HotPink3 - #d75f87 - rgb(215,95,135)
pub const C168_HOT_PINK3: &str = "\x1b[38;5;168m";

/// Xterm 169 - HotPink2 - #d75faf - rgb(215,95,175)
pub const C169_HOT_PINK2: &str = "\x1b[38;5;169m";

/// Xterm 170 - Orchid - #d75fd7 - rgb(215,95,215)
pub const C170_ORCHID: &str = "\x1b[38;5;170m";

/// Xterm 171 - MediumOrchid1 - #d75fff - rgb(215,95,255)
pub const C171_MEDIUM_ORCHID1: &str = "\x1b[38;5;171m";

/// Xterm 172 - Orange3 - #d78700 - rgb(215,135,0)
pub const C172_ORANGE3: &str = "\x1b[38;5;172m";

/// Xterm 173 - LightSalmon3 - #d7875f - rgb(215,135,95)
pub const C173_LIGHT_SALMON3: &str = "\x1b[38;5;173m";

/// Xterm 174 - LightPink3 - #d78787 - rgb(215,135,135)
pub const C174_LIGHT_PINK3: &str = "\x1b[38;5;174m";

/// Xterm 175 - Pink3 - #d787af - rgb(215,135,175)
pub const C175_PINK3: &str = "\x1b[38;5;175m";

/// Xterm 176 - Plum3 - #d787d7 - rgb(215,135,215)
pub const C176_PLUM3: &str = "\x1b[38;5;176m";

/// Xterm 177 - Violet - #d787ff - rgb(215,135,255)
pub const C177_VIOLET: &str = "\x1b[38;5;177m";

/// Xterm 178 - Gold3 - #d7af00 - rgb(215,175,0)
pub const C178_GOLD3: &str = "\x1b[38;5;178m";

/// Xterm 179 - LightGoldenrod3 - #d7af5f - rgb(215,175,95)
pub const C179_LIGHT_GOLDENROD3: &str = "\x1b[38;5;179m";

/// Xterm 180 - Tan - #d7af87 - rgb(215,175,135)
pub const C180_TAN: &str = "\x1b[38;5;180m";

/// Xterm 181 - MistyRose3 - #d7afaf - rgb(215,175,175)
pub const C181_MISTY_ROSE3: &str = "\x1b[38;5;181m";

/// Xterm 182 - Thistle3 - #d7afd7 - rgb(215,175,215)
pub const C182_THISTLE3: &str = "\x1b[38;5;182m";

/// Xterm 183 - Plum2 - #d7afff - rgb(215,175,255)
pub const C183_PLUM2: &str = "\x1b[38;5;183m";

/// Xterm 184 - Yellow3 - #d7d700 - rgb(215,215,0)
pub const C184_YELLOW3: &str = "\x1b[38;5;184m";

/// Xterm 185 - Khaki3 - #d7d75f - rgb(215,215,95)
pub const C185_KHAKI3: &str = "\x1b[38;5;185m";

/// Xterm 186 - LightGoldenrod2 - #d7d787 - rgb(215,215,135)
pub const C186_LIGHT_GOLDENROD2: &str = "\x1b[38;5;186m";

/// Xterm 187 - LightYellow3 - #d7d7af - rgb(215,215,175)
pub const C187_LIGHT_YELLOW3: &str = "\x1b[38;5;187m";

/// Xterm 188 - Grey84 - #d7d7d7 - rgb(215,215,215)
pub const C188_GREY84: &str = "\x1b[38;5;188m";

/// Xterm 189 - LightSteelBlue1 - #d7d7ff - rgb(215,215,255)
pub const C189_LIGHT_STEEL_BLUE1: &str = "\x1b[38;5;189m";

/// Xterm 190 - Yellow2 - #d7ff00 - rgb(215,255,0)
pub const C190_YELLOW2: &str = "\x1b[38;5;190m";

/// Xterm 191 - DarkOliveGreen1 - #d7ff5f - rgb(215,255,95)
pub const C191_DARK_OLIVE_GREEN1: &str = "\x1b[38;5;191m";

/// Xterm 192 - DarkOliveGreen1 - #d7ff87 - rgb(215,255,135)
pub const C192_DARK_OLIVE_GREEN1: &str = "\x1b[38;5;192m";

/// Xterm 193 - DarkSeaGreen1 - #d7ffaf - rgb(215,255,175)
pub const C193_DARK_SEA_GREEN1: &str = "\x1b[38;5;193m";

/// Xterm 194 - Honeydew2 - #d7ffd7 - rgb(215,255,215)
pub const C194_HONEYDEW2: &str = "\x1b[38;5;194m";

/// Xterm 195 - LightCyan1 - #d7ffff - rgb(215,255,255)
pub const C195_LIGHT_CYAN1: &str = "\x1b[38;5;195m";

/// Xterm 196 - Red1 - #ff0000 - rgb(255,0,0)
pub const C196_RED1: &str = "\x1b[38;5;196m";

/// Xterm 197 - DeepPink2 - #ff005f - rgb(255,0,95)
pub const C197_DEEP_PINK2: &str = "\x1b[38;5;197m";

/// Xterm 198 - DeepPink1 - #ff0087 - rgb(255,0,135)
pub const C198_DEEP_PINK1: &str = "\x1b[38;5;198m";

/// Xterm 199 - DeepPink1 - #ff00af - rgb(255,0,175)
pub const C199_DEEP_PINK1: &str = "\x1b[38;5;199m";

/// Xterm 200 - Magenta2 - #ff00d7 - rgb(255,0,215)
pub const C200_MAGENTA2: &str = "\x1b[38;5;200m";

/// Xterm 201 - Magenta1 - #ff00ff - rgb(255,0,255)
pub const C201_MAGENTA1: &str = "\x1b[38;5;201m";

/// Xterm 202 - OrangeRed1 - #ff5f00 - rgb(255,95,0)
pub const C202_ORANGE_RED1: &str = "\x1b[38;5;202m";

/// Xterm 203 - IndianRed1 - #ff5f5f - rgb(255,95,95)
pub const C203_INDIAN_RED1: &str = "\x1b[38;5;203m";

/// Xterm 204 - IndianRed1 - #ff5f87 - rgb(255,95,135)
pub const C204_INDIAN_RED1: &str = "\x1b[38;5;204m";

/// Xterm 205 - HotPink - #ff5faf - rgb(255,95,175)
pub const C205_HOT_PINK: &str = "\x1b[38;5;205m";

/// Xterm 206 - HotPink - #ff5fd7 - rgb(255,95,215)
pub const C206_HOT_PINK: &str = "\x1b[38;5;206m";

/// Xterm 207 - MediumOrchid1 - #ff5fff - rgb(255,95,255)
pub const C207_MEDIUM_ORCHID1: &str = "\x1b[38;5;207m";

/// Xterm 208 - DarkOrange - #ff8700 - rgb(255,135,0)
pub const C208_DARK_ORANGE: &str = "\x1b[38;5;208m";

/// Xterm 209 - Salmon1 - #ff875f - rgb(255,135,95)
pub const C209_SALMON1: &str = "\x1b[38;5;209m";

/// Xterm 210 - LightCoral - #ff8787 - rgb(255,135,135)
pub const C210_LIGHT_CORAL: &str = "\x1b[38;5;210m";

/// Xterm 211 - PaleVioletRed1 - #ff87af - rgb(255,135,175)
pub const C211_PALE_VIOLET_RED1: &str = "\x1b[38;5;211m";

/// Xterm 212 - Orchid2 - #ff87d7 - rgb(255,135,215)
pub const C212_ORCHID2: &str = "\x1b[38;5;212m";

/// Xterm 213 - Orchid1 - #ff87ff - rgb(255,135,255)
pub const C213_ORCHID1: &str = "\x1b[38;5;213m";

/// Xterm 214 - Orange1 - #ffaf00 - rgb(255,175,0)
pub const C214_ORANGE1: &str = "\x1b[38;5;214m";

/// Xterm 215 - SandyBrown - #ffaf5f - rgb(255,175,95)
pub const C215_SANDY_BROWN: &str = "\x1b[38;5;215m";

/// Xterm 216 - LightSalmon1 - #ffaf87 - rgb(255,175,135)
pub const C216_LIGHT_SALMON1: &str = "\x1b[38;5;216m";

/// Xterm 217 - LightPink1 - #ffafaf - rgb(255,175,175)
pub const C217_LIGHT_PINK1: &str = "\x1b[38;5;217m";

/// Xterm 218 - Pink1 - #ffafd7 - rgb(255,175,215)
pub const C218_PINK1: &str = "\x1b[38;5;218m";

/// Xterm 219 - Plum1 - #ffafff - rgb(255,175,255)
pub const C219_PLUM1: &str = "\x1b[38;5;219m";

/// Xterm 220 - Gold1 - #ffd700 - rgb(255,215,0)
pub const C220_GOLD1: &str = "\x1b[38;5;220m";

/// Xterm 221 - LightGoldenrod2 - #ffd75f - rgb(255,215,95)
pub const C221_LIGHT_GOLDENROD2: &str = "\x1b[38;5;221m";

/// Xterm 222 - LightGoldenrod2 - #ffd787 - rgb(255,215,135)
pub const C222_LIGHT_GOLDENROD2: &str = "\x1b[38;5;222m";

/// Xterm 223 - NavajoWhite1 - #ffd7af - rgb(255,215,175)
pub const C223_NAVAJO_WHITE1: &str = "\x1b[38;5;223m";

/// Xterm 224 - MistyRose1 - #ffd7d7 - rgb(255,215,215)
pub const C224_MISTY_ROSE1: &str = "\x1b[38;5;224m";

/// Xterm 225 - Thistle1 - #ffd7ff - rgb(255,215,255)
pub const C225_THISTLE1: &str = "\x1b[38;5;225m";

/// Xterm 226 - Yellow1 - #ffff00 - rgb(255,255,0)
pub const C226_YELLOW1: &str = "\x1b[38;5;226m";

/// Xterm 227 - LightGoldenrod1 - #ffff5f - rgb(255,255,95)
pub const C227_LIGHT_GOLDENROD1: &str = "\x1b[38;5;227m";

/// Xterm 228 - Khaki1 - #ffff87 - rgb(255,255,135)
pub const C228_KHAKI1: &str = "\x1b[38;5;228m";

/// Xterm 229 - Wheat1 - #ffffaf - rgb(255,255,175)
pub const C229_WHEAT1: &str = "\x1b[38;5;229m";

/// Xterm 230 - Cornsilk1 - #ffffd7 - rgb(255,255,215)
pub const C230_CORNSILK1: &str = "\x1b[38;5;230m";

/// Xterm 231 - Grey100 - #ffffff - rgb(255,255,255)
pub const C231_GREY100: &str = "\x1b[38;5;231m";

/// Xterm 232 - Grey3 - #080808 - rgb(8,8,8)
pub const C232_GREY3: &str = "\x1b[38;5;232m";

/// Xterm 233 - Grey7 - #121212 - rgb(18,18,18)
pub const C233_GREY7: &str = "\x1b[38;5;233m";

/// Xterm 234 - Grey11 - #1c1c1c - rgb(28,28,28)
pub const C234_GREY11: &str = "\x1b[38;5;234m";

/// Xterm 235 - Grey15 - #262626 - rgb(38,38,38)
pub const C235_GREY15: &str = "\x1b[38;5;235m";

/// Xterm 236 - Grey19 - #303030 - rgb(48,48,48)
pub const C236_GREY19: &str = "\x1b[38;5;236m";

/// Xterm 237 - Grey23 - #3a3a3a - rgb(58,58,58)
pub const C237_GREY23: &str = "\x1b[38;5;237m";

/// Xterm 238 - Grey27 - #444444 - rgb(68,68,68)
pub const C238_GREY27: &str = "\x1b[38;5;238m";

/// Xterm 239 - Grey30 - #4e4e4e - rgb(78,78,78)
pub const C239_GREY30: &str = "\x1b[38;5;239m";

/// Xterm 240 - Grey35 - #585858 - rgb(88,88,88)
pub const C240_GREY35: &str = "\x1b[38;5;240m";

/// Xterm 241 - Grey39 - #626262 - rgb(98,98,98)
pub const C241_GREY39: &str = "\x1b[38;5;241m";

/// Xterm 242 - Grey42 - #6c6c6c - rgb(108,108,108)
pub const C242_GREY42: &str = "\x1b[38;5;242m";

/// Xterm 243 - Grey46 - #767676 - rgb(118,118,118)
pub const C243_GREY46: &str = "\x1b[38;5;243m";

/// Xterm 244 - Grey50 - #808080 - rgb(128,128,128)
pub const C244_GREY50: &str = "\x1b[38;5;244m";

/// Xterm 245 - Grey54 - #8a8a8a - rgb(138,138,138)
pub const C245_GREY54: &str = "\x1b[38;5;245m";

/// Xterm 246 - Grey58 - #949494 - rgb(148,148,148)
pub const C246_GREY58: &str = "\x1b[38;5;246m";

/// Xterm 247 - Grey62 - #9e9e9e - rgb(158,158,158)
pub const C247_GREY62: &str = "\x1b[38;5;247m";

/// Xterm 248 - Grey66 - #a8a8a8 - rgb(168,168,168)
pub const C248_GREY66: &str = "\x1b[38;5;248m";

/// Xterm 249 - Grey70 - #b2b2b2 - rgb(178,178,178)
pub const C249_GREY70: &str = "\x1b[38;5;249m";

/// Xterm 250 - Grey74 - #bcbcbc - rgb(188,188,188)
pub const C250_GREY74: &str = "\x1b[38;5;250m";

/// Xterm 251 - Grey78 - #c6c6c6 - rgb(198,198,198)
pub const C251_GREY78: &str = "\x1b[38;5;251m";

/// Xterm 252 - Grey82 - #d0d0d0 - rgb(208,208,208)
pub const C252_GREY82: &str = "\x1b[38;5;252m";

/// Xterm 253 - Grey85 - #dadada - rgb(218,218,218)
pub const C253_GREY85: &str = "\x1b[38;5;253m";

/// Xterm 254 - Grey89 - #e4e4e4 - rgb(228,228,228)
pub const C254_GREY89: &str = "\x1b[38;5;254m";

/// Xterm 255 - Grey93 - #eeeeee - rgb(238,238,238)
pub const C255_GREY93: &str = "\x1b[38;5;255m";
