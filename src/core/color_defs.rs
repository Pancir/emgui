//! The same colors as defined in the Flutter's material.

use super::Rgba8;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip] pub const fn invisible() -> Rgba8 { Rgba8::init(0x00000000) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// BLACK

#[rustfmt::skip] pub const fn black()    -> Rgba8 { Rgba8::init_argb(0xFF000000) }
#[rustfmt::skip] pub const fn black_87() -> Rgba8 { Rgba8::init_argb(0xDD000000) }
#[rustfmt::skip] pub const fn black_54() -> Rgba8 { Rgba8::init_argb(0x8A000000) }
#[rustfmt::skip] pub const fn black_45() -> Rgba8 { Rgba8::init_argb(0x73000000) }
#[rustfmt::skip] pub const fn black_38() -> Rgba8 { Rgba8::init_argb(0x61000000) }
#[rustfmt::skip] pub const fn black_26() -> Rgba8 { Rgba8::init_argb(0x42000000) }
#[rustfmt::skip] pub const fn black_12() -> Rgba8 { Rgba8::init_argb(0x1F000000) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// WHITE

#[rustfmt::skip] pub const fn white()    -> Rgba8 { Rgba8::init_argb(0xFFFFFFFF) }
#[rustfmt::skip] pub const fn white_70() -> Rgba8 { Rgba8::init_argb(0xB3FFFFFF) }
#[rustfmt::skip] pub const fn white_60() -> Rgba8 { Rgba8::init_argb(0x99FFFFFF) }
#[rustfmt::skip] pub const fn white_54() -> Rgba8 { Rgba8::init_argb(0x8AFFFFFF) }
#[rustfmt::skip] pub const fn white_38() -> Rgba8 { Rgba8::init_argb(0x62FFFFFF) }
#[rustfmt::skip] pub const fn white_30() -> Rgba8 { Rgba8::init_argb(0x4DFFFFFF) }
#[rustfmt::skip] pub const fn white_24() -> Rgba8 { Rgba8::init_argb(0x3DFFFFFF) }
#[rustfmt::skip] pub const fn white_12() -> Rgba8 { Rgba8::init_argb(0x1FFFFFFF) }
#[rustfmt::skip] pub const fn white_10() -> Rgba8 { Rgba8::init_argb(0x1AFFFFFF) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// RED

const RED_PRIMARY_VALUE: u32 = 0xFFF44336;
#[rustfmt::skip] pub const fn red_50()  -> Rgba8 { Rgba8::init_argb(0xFFFFEBEE) }
#[rustfmt::skip] pub const fn red_100() -> Rgba8 { Rgba8::init_argb(0xFFFFCDD2) }
#[rustfmt::skip] pub const fn red_200() -> Rgba8 { Rgba8::init_argb(0xFFEF9A9A) }
#[rustfmt::skip] pub const fn red_300() -> Rgba8 { Rgba8::init_argb(0xFFE57373) }
#[rustfmt::skip] pub const fn red_400() -> Rgba8 { Rgba8::init_argb(0xFFEF5350) }
#[rustfmt::skip] pub const fn red_500() -> Rgba8 { Rgba8::init_argb(RED_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn red_600() -> Rgba8 { Rgba8::init_argb(0xFFE53935) }
#[rustfmt::skip] pub const fn red_700() -> Rgba8 { Rgba8::init_argb(0xFFD32F2F) }
#[rustfmt::skip] pub const fn red_800() -> Rgba8 { Rgba8::init_argb(0xFFC62828) }
#[rustfmt::skip] pub const fn red_900() -> Rgba8 { Rgba8::init_argb(0xFFB71C1C) }

//----------------------------------------
// RED ACCENT

const RED_ACCENT_VALUE: u32 = 0xFFFF5252;
#[rustfmt::skip] pub const fn red_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFFF8A80) }
#[rustfmt::skip] pub const fn red_accent_200() -> Rgba8 { Rgba8::init_argb(RED_ACCENT_VALUE) }
#[rustfmt::skip] pub const fn red_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFFF1744) }
#[rustfmt::skip] pub const fn red_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFD50000) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// PINK

const PINK_PRIMARY_VALUE: u32 = 0xFFE91E63;
#[rustfmt::skip] pub const fn pink_50()  -> Rgba8 { Rgba8::init_argb(0xFFFCE4EC) }
#[rustfmt::skip] pub const fn pink_100() -> Rgba8 { Rgba8::init_argb(0xFFF8BBD0) }
#[rustfmt::skip] pub const fn pink_200() -> Rgba8 { Rgba8::init_argb(0xFFF48FB1) }
#[rustfmt::skip] pub const fn pink_300() -> Rgba8 { Rgba8::init_argb(0xFFF06292) }
#[rustfmt::skip] pub const fn pink_400() -> Rgba8 { Rgba8::init_argb(0xFFEC407A) }
#[rustfmt::skip] pub const fn pink_500() -> Rgba8 { Rgba8::init_argb(PINK_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn pink_600() -> Rgba8 { Rgba8::init_argb(0xFFD81B60) }
#[rustfmt::skip] pub const fn pink_700() -> Rgba8 { Rgba8::init_argb(0xFFC2185B) }
#[rustfmt::skip] pub const fn pink_800() -> Rgba8 { Rgba8::init_argb(0xFFAD1457) }
#[rustfmt::skip] pub const fn pink_900() -> Rgba8 { Rgba8::init_argb(0xFF880E4F) }

//----------------------------------------
// PINK ACCENT

const PINK_ACCENT_PRIMARY_VALUE: u32 = 0xFFFF4081;
#[rustfmt::skip] pub const fn pink_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFFF80AB) }
#[rustfmt::skip] pub const fn pink_accent_200() -> Rgba8 { Rgba8::init_argb(PINK_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn pink_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFF50057) }
#[rustfmt::skip] pub const fn pink_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFC51162) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// PURPLE

const PURPLE_PRIMARY_VALUE: u32 = 0xFF9C27B0;
#[rustfmt::skip] pub const fn purple_50()  -> Rgba8 { Rgba8::init_argb(0xFFF3E5F5) }
#[rustfmt::skip] pub const fn purple_100() -> Rgba8 { Rgba8::init_argb(0xFFE1BEE7) }
#[rustfmt::skip] pub const fn purple_200() -> Rgba8 { Rgba8::init_argb(0xFFCE93D8) }
#[rustfmt::skip] pub const fn purple_300() -> Rgba8 { Rgba8::init_argb(0xFFBA68C8) }
#[rustfmt::skip] pub const fn purple_400() -> Rgba8 { Rgba8::init_argb(0xFFAB47BC) }
#[rustfmt::skip] pub const fn purple_500() -> Rgba8 { Rgba8::init_argb(PURPLE_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn purple_600() -> Rgba8 { Rgba8::init_argb(0xFF8E24AA) }
#[rustfmt::skip] pub const fn purple_700() -> Rgba8 { Rgba8::init_argb(0xFF7B1FA2) }
#[rustfmt::skip] pub const fn purple_800() -> Rgba8 { Rgba8::init_argb(0xFF6A1B9A) }
#[rustfmt::skip] pub const fn purple_900() -> Rgba8 { Rgba8::init_argb(0xFF4A148C) }

//----------------------------------------
// PURPLE ACCENT

const PURPLE_ACCENT_PRIMARY_VALUE: u32 = 0xFFE040FB;
#[rustfmt::skip] pub const fn purple_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFEA80FC) }
#[rustfmt::skip] pub const fn purple_accent_200() -> Rgba8 { Rgba8::init_argb(PURPLE_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn purple_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFD500F9) }
#[rustfmt::skip] pub const fn purple_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFAA00FF) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// PURPLE

const DEEP_PURPLE_PRIMARY_VALUE: u32 = 0xFF673AB7;
#[rustfmt::skip] pub const fn deep_purple_50()  -> Rgba8 { Rgba8::init_argb(0xFFEDE7F6) }
#[rustfmt::skip] pub const fn deep_purple_100() -> Rgba8 { Rgba8::init_argb(0xFFD1C4E9) }
#[rustfmt::skip] pub const fn deep_purple_200() -> Rgba8 { Rgba8::init_argb(0xFFB39DDB) }
#[rustfmt::skip] pub const fn deep_purple_300() -> Rgba8 { Rgba8::init_argb(0xFF9575CD) }
#[rustfmt::skip] pub const fn deep_purple_400() -> Rgba8 { Rgba8::init_argb(0xFF7E57C2) }
#[rustfmt::skip] pub const fn deep_purple_500() -> Rgba8 { Rgba8::init_argb(DEEP_PURPLE_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn deep_purple_600() -> Rgba8 { Rgba8::init_argb(0xFF5E35B1) }
#[rustfmt::skip] pub const fn deep_purple_700() -> Rgba8 { Rgba8::init_argb(0xFF512DA8) }
#[rustfmt::skip] pub const fn deep_purple_800() -> Rgba8 { Rgba8::init_argb(0xFF4527A0) }
#[rustfmt::skip] pub const fn deep_purple_900() -> Rgba8 { Rgba8::init_argb(0xFF311B92) }

//----------------------------------------
// PURPLE ACCENT

const DEEP_PURPLE_ACCENT_PRIMARY_VALUE: u32 = 0xFF7C4DFF;
#[rustfmt::skip] pub const fn deep_purple_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFB388FF) }
#[rustfmt::skip] pub const fn deep_purple_accent_200() -> Rgba8 { Rgba8::init_argb(DEEP_PURPLE_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn deep_purple_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF651FFF) }
#[rustfmt::skip] pub const fn deep_purple_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF6200EA) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// INDIGO

const INDIGO_PRIMARY_VALUE: u32 = 0xFF3F51B5;
#[rustfmt::skip] pub const fn indigo_50()  -> Rgba8 { Rgba8::init_argb(0xFFE8EAF6) }
#[rustfmt::skip] pub const fn indigo_100() -> Rgba8 { Rgba8::init_argb(0xFFC5CAE9) }
#[rustfmt::skip] pub const fn indigo_200() -> Rgba8 { Rgba8::init_argb(0xFF9FA8DA) }
#[rustfmt::skip] pub const fn indigo_300() -> Rgba8 { Rgba8::init_argb(0xFF7986CB) }
#[rustfmt::skip] pub const fn indigo_400() -> Rgba8 { Rgba8::init_argb(0xFF5C6BC0) }
#[rustfmt::skip] pub const fn indigo_500() -> Rgba8 { Rgba8::init_argb(INDIGO_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn indigo_600() -> Rgba8 { Rgba8::init_argb(0xFF3949AB) }
#[rustfmt::skip] pub const fn indigo_700() -> Rgba8 { Rgba8::init_argb(0xFF303F9F) }
#[rustfmt::skip] pub const fn indigo_800() -> Rgba8 { Rgba8::init_argb(0xFF283593) }
#[rustfmt::skip] pub const fn indigo_900() -> Rgba8 { Rgba8::init_argb(0xFF1A237E) }

//----------------------------------------
// INDIGO ACCENT

const INDIGO_ACCENT_PRIMARY_VALUE: u32 = 0xFF536DFE;
#[rustfmt::skip] pub const fn indigo_accent_100() -> Rgba8 { Rgba8::init_argb(0xFF8C9EFF) }
#[rustfmt::skip] pub const fn indigo_accent_200() -> Rgba8 { Rgba8::init_argb(INDIGO_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn indigo_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF3D5AFE) }
#[rustfmt::skip] pub const fn indigo_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF304FFE) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// BLUE

const BLUE_PRIMARY_VALUE: u32 = 0xFF2196F3;
#[rustfmt::skip] pub const fn blue_50()  -> Rgba8 { Rgba8::init_argb(0xFFE3F2FD) }
#[rustfmt::skip] pub const fn blue_100() -> Rgba8 { Rgba8::init_argb(0xFFBBDEFB) }
#[rustfmt::skip] pub const fn blue_200() -> Rgba8 { Rgba8::init_argb(0xFF90CAF9) }
#[rustfmt::skip] pub const fn blue_300() -> Rgba8 { Rgba8::init_argb(0xFF64B5F6) }
#[rustfmt::skip] pub const fn blue_400() -> Rgba8 { Rgba8::init_argb(0xFF42A5F5) }
#[rustfmt::skip] pub const fn blue_500() -> Rgba8 { Rgba8::init_argb(BLUE_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn blue_600() -> Rgba8 { Rgba8::init_argb(0xFF1E88E5) }
#[rustfmt::skip] pub const fn blue_700() -> Rgba8 { Rgba8::init_argb(0xFF1976D2) }
#[rustfmt::skip] pub const fn blue_800() -> Rgba8 { Rgba8::init_argb(0xFF1565C0) }
#[rustfmt::skip] pub const fn blue_900() -> Rgba8 { Rgba8::init_argb(0xFF0D47A1) }

//----------------------------------------
// BLUE ACCENT

const BLUE_ACCENT_PRIMARY_VALUE: u32 = 0xFF448AFF;
#[rustfmt::skip] pub const fn blue_accent_100() -> Rgba8 { Rgba8::init_argb(0xFF82B1FF) }
#[rustfmt::skip] pub const fn blue_accent_200() -> Rgba8 { Rgba8::init_argb(BLUE_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn blue_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF2979FF) }
#[rustfmt::skip] pub const fn blue_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF2962FF) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// LIGHT BLUE

const LIGHT_BLUE_PRIMARY_VALUE: u32 = 0xFF03A9F4;
#[rustfmt::skip] pub const fn light_blue_50()  -> Rgba8 { Rgba8::init_argb(0xFFE1F5FE) }
#[rustfmt::skip] pub const fn light_blue_100() -> Rgba8 { Rgba8::init_argb(0xFFB3E5FC) }
#[rustfmt::skip] pub const fn light_blue_200() -> Rgba8 { Rgba8::init_argb(0xFF81D4FA) }
#[rustfmt::skip] pub const fn light_blue_300() -> Rgba8 { Rgba8::init_argb(0xFF4FC3F7) }
#[rustfmt::skip] pub const fn light_blue_400() -> Rgba8 { Rgba8::init_argb(0xFF29B6F6) }
#[rustfmt::skip] pub const fn light_blue_500() -> Rgba8 { Rgba8::init_argb(LIGHT_BLUE_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn light_blue_600() -> Rgba8 { Rgba8::init_argb(0xFF039BE5) }
#[rustfmt::skip] pub const fn light_blue_700() -> Rgba8 { Rgba8::init_argb(0xFF0288D1) }
#[rustfmt::skip] pub const fn light_blue_800() -> Rgba8 { Rgba8::init_argb(0xFF0277BD) }
#[rustfmt::skip] pub const fn light_blue_900() -> Rgba8 { Rgba8::init_argb(0xFF01579B) }

//----------------------------------------
// LIGHT BLUE ACCENT

const LIGHT_BLUE_ACCENT_PRIMARY_VALUE: u32 = 0xFF40C4FF;
#[rustfmt::skip] pub const fn light_blue_accent_100() -> Rgba8 { Rgba8::init_argb(0xFF80D8FF) }
#[rustfmt::skip] pub const fn light_blue_accent_200() -> Rgba8 { Rgba8::init_argb(LIGHT_BLUE_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn light_blue_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF00B0FF) }
#[rustfmt::skip] pub const fn light_blue_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF0091EA) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// CYAN

const CYAN_PRIMARY_VALUE: u32 = 0xFF00BCD4;
#[rustfmt::skip] pub const fn cyan_50()  -> Rgba8 { Rgba8::init_argb(0xFFE0F7FA) }
#[rustfmt::skip] pub const fn cyan_100() -> Rgba8 { Rgba8::init_argb(0xFFB2EBF2) }
#[rustfmt::skip] pub const fn cyan_200() -> Rgba8 { Rgba8::init_argb(0xFF80DEEA) }
#[rustfmt::skip] pub const fn cyan_300() -> Rgba8 { Rgba8::init_argb(0xFF4DD0E1) }
#[rustfmt::skip] pub const fn cyan_400() -> Rgba8 { Rgba8::init_argb(0xFF26C6DA) }
#[rustfmt::skip] pub const fn cyan_500() -> Rgba8 { Rgba8::init_argb(CYAN_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn cyan_600() -> Rgba8 { Rgba8::init_argb(0xFF00ACC1) }
#[rustfmt::skip] pub const fn cyan_700() -> Rgba8 { Rgba8::init_argb(0xFF0097A7) }
#[rustfmt::skip] pub const fn cyan_800() -> Rgba8 { Rgba8::init_argb(0xFF00838F) }
#[rustfmt::skip] pub const fn cyan_900() -> Rgba8 { Rgba8::init_argb(0xFF006064) }

//----------------------------------------
// CYAN ACCENT

const CYAN_ACCENT_PRIMARY_VALUE: u32 = 0xFF18FFFF;
#[rustfmt::skip] pub const fn cyan_accent_100() -> Rgba8 { Rgba8::init_argb(0xFF84FFFF) }
#[rustfmt::skip] pub const fn cyan_accent_200() -> Rgba8 { Rgba8::init_argb(CYAN_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn cyan_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF00E5FF) }
#[rustfmt::skip] pub const fn cyan_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF00B8D4) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// TEAL

const TEAL_PRIMARY_VALUE: u32 = 0xFF009688;
#[rustfmt::skip] pub const fn teal_50()  -> Rgba8 { Rgba8::init_argb(0xFFE0F2F1) }
#[rustfmt::skip] pub const fn teal_100() -> Rgba8 { Rgba8::init_argb(0xFFB2DFDB) }
#[rustfmt::skip] pub const fn teal_200() -> Rgba8 { Rgba8::init_argb(0xFF80CBC4) }
#[rustfmt::skip] pub const fn teal_300() -> Rgba8 { Rgba8::init_argb(0xFF4DB6AC) }
#[rustfmt::skip] pub const fn teal_400() -> Rgba8 { Rgba8::init_argb(0xFF26A69A) }
#[rustfmt::skip] pub const fn teal_500() -> Rgba8 { Rgba8::init_argb(TEAL_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn teal_600() -> Rgba8 { Rgba8::init_argb(0xFF00897B) }
#[rustfmt::skip] pub const fn teal_700() -> Rgba8 { Rgba8::init_argb(0xFF00796B) }
#[rustfmt::skip] pub const fn teal_800() -> Rgba8 { Rgba8::init_argb(0xFF00695C) }
#[rustfmt::skip] pub const fn teal_900() -> Rgba8 { Rgba8::init_argb(0xFF004D40) }

//----------------------------------------
// TEAL ACCENT

const TEAL_ACCENT_PRIMARY_VALUE: u32 = 0xFF18FFFF;
#[rustfmt::skip] pub const fn teal_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFA7FFEB) }
#[rustfmt::skip] pub const fn teal_accent_200() -> Rgba8 { Rgba8::init_argb(TEAL_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn teal_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF1DE9B6) }
#[rustfmt::skip] pub const fn teal_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF00BFA5) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// GREEN

const GREEN_PRIMARY_VALUE: u32 = 0xFF4CAF50;
#[rustfmt::skip] pub const fn green_50()  -> Rgba8 { Rgba8::init_argb(0xFFE8F5E9) }
#[rustfmt::skip] pub const fn green_100() -> Rgba8 { Rgba8::init_argb(0xFFC8E6C9) }
#[rustfmt::skip] pub const fn green_200() -> Rgba8 { Rgba8::init_argb(0xFFA5D6A7) }
#[rustfmt::skip] pub const fn green_300() -> Rgba8 { Rgba8::init_argb(0xFF81C784) }
#[rustfmt::skip] pub const fn green_400() -> Rgba8 { Rgba8::init_argb(0xFF66BB6A) }
#[rustfmt::skip] pub const fn green_500() -> Rgba8 { Rgba8::init_argb(GREEN_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn green_600() -> Rgba8 { Rgba8::init_argb(0xFF43A047) }
#[rustfmt::skip] pub const fn green_700() -> Rgba8 { Rgba8::init_argb(0xFF388E3C) }
#[rustfmt::skip] pub const fn green_800() -> Rgba8 { Rgba8::init_argb(0xFF2E7D32) }
#[rustfmt::skip] pub const fn green_900() -> Rgba8 { Rgba8::init_argb(0xFF1B5E20) }

//----------------------------------------
// GREEN ACCENT

const GREEN_ACCENT_PRIMARY_VALUE: u32 = 0xFF69F0AE;
#[rustfmt::skip] pub const fn green_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFB9F6CA) }
#[rustfmt::skip] pub const fn green_accent_200() -> Rgba8 { Rgba8::init_argb(GREEN_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn green_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF00E676) }
#[rustfmt::skip] pub const fn green_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF00C853) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// LIGHT GREEN

const LIGHT_GREEN_PRIMARY_VALUE: u32 = 0xFF8BC34A;
#[rustfmt::skip] pub const fn light_green_50()  -> Rgba8 { Rgba8::init_argb(0xFFF1F8E9) }
#[rustfmt::skip] pub const fn light_green_100() -> Rgba8 { Rgba8::init_argb(0xFFDCEDC8) }
#[rustfmt::skip] pub const fn light_green_200() -> Rgba8 { Rgba8::init_argb(0xFFC5E1A5) }
#[rustfmt::skip] pub const fn light_green_300() -> Rgba8 { Rgba8::init_argb(0xFFAED581) }
#[rustfmt::skip] pub const fn light_green_400() -> Rgba8 { Rgba8::init_argb(0xFF9CCC65) }
#[rustfmt::skip] pub const fn light_green_500() -> Rgba8 { Rgba8::init_argb(LIGHT_GREEN_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn light_green_600() -> Rgba8 { Rgba8::init_argb(0xFF7CB342) }
#[rustfmt::skip] pub const fn light_green_700() -> Rgba8 { Rgba8::init_argb(0xFF689F38) }
#[rustfmt::skip] pub const fn light_green_800() -> Rgba8 { Rgba8::init_argb(0xFF558B2F) }
#[rustfmt::skip] pub const fn light_green_900() -> Rgba8 { Rgba8::init_argb(0xFF33691E) }

//----------------------------------------
// LIGHT GREEN ACCENT

const LIGHT_GREEN_ACCENT_PRIMARY_VALUE: u32 = 0xFFB2FF59;
#[rustfmt::skip] pub const fn light_green_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFCCFF90) }
#[rustfmt::skip] pub const fn light_green_accent_200() -> Rgba8 { Rgba8::init_argb(LIGHT_GREEN_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn light_green_accent_400() -> Rgba8 { Rgba8::init_argb(0xFF76FF03) }
#[rustfmt::skip] pub const fn light_green_accent_700() -> Rgba8 { Rgba8::init_argb(0xFF64DD17) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// LIME

const LIME_PRIMARY_VALUE: u32 = 0xFFCDDC39;
#[rustfmt::skip] pub const fn lime_50()  -> Rgba8 { Rgba8::init_argb(0xFFF9FBE7) }
#[rustfmt::skip] pub const fn lime_100() -> Rgba8 { Rgba8::init_argb(0xFFF0F4C3) }
#[rustfmt::skip] pub const fn lime_200() -> Rgba8 { Rgba8::init_argb(0xFFE6EE9C) }
#[rustfmt::skip] pub const fn lime_300() -> Rgba8 { Rgba8::init_argb(0xFFDCE775) }
#[rustfmt::skip] pub const fn lime_400() -> Rgba8 { Rgba8::init_argb(0xFFD4E157) }
#[rustfmt::skip] pub const fn lime_500() -> Rgba8 { Rgba8::init_argb(LIME_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn lime_600() -> Rgba8 { Rgba8::init_argb(0xFFC0CA33) }
#[rustfmt::skip] pub const fn lime_700() -> Rgba8 { Rgba8::init_argb(0xFFAFB42B) }
#[rustfmt::skip] pub const fn lime_800() -> Rgba8 { Rgba8::init_argb(0xFF9E9D24) }
#[rustfmt::skip] pub const fn lime_900() -> Rgba8 { Rgba8::init_argb(0xFF827717) }

//----------------------------------------
// LIME ACCENT

const LIME_ACCENT_PRIMARY_VALUE: u32 = 0xFFEEFF41;
#[rustfmt::skip] pub const fn lime_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFF4FF81) }
#[rustfmt::skip] pub const fn lime_accent_200() -> Rgba8 { Rgba8::init_argb(LIME_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn lime_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFC6FF00) }
#[rustfmt::skip] pub const fn lime_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFAEEA00) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// YELLOW

const YELLOW_PRIMARY_VALUE: u32 = 0xFFFFEB3B;
#[rustfmt::skip] pub const fn yellow_50()  -> Rgba8 { Rgba8::init_argb(0xFFFFFDE7) }
#[rustfmt::skip] pub const fn yellow_100() -> Rgba8 { Rgba8::init_argb(0xFFFFF9C4) }
#[rustfmt::skip] pub const fn yellow_200() -> Rgba8 { Rgba8::init_argb(0xFFFFF59D) }
#[rustfmt::skip] pub const fn yellow_300() -> Rgba8 { Rgba8::init_argb(0xFFFFF176) }
#[rustfmt::skip] pub const fn yellow_400() -> Rgba8 { Rgba8::init_argb(0xFFFFEE58) }
#[rustfmt::skip] pub const fn yellow_500() -> Rgba8 { Rgba8::init_argb(YELLOW_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn yellow_600() -> Rgba8 { Rgba8::init_argb(0xFFFDD835) }
#[rustfmt::skip] pub const fn yellow_700() -> Rgba8 { Rgba8::init_argb(0xFFFBC02D) }
#[rustfmt::skip] pub const fn yellow_800() -> Rgba8 { Rgba8::init_argb(0xFFF9A825) }
#[rustfmt::skip] pub const fn yellow_900() -> Rgba8 { Rgba8::init_argb(0xFFF57F17) }

//----------------------------------------
// YELLOW ACCENT

const YELLOW_ACCENT_PRIMARY_VALUE: u32 = 0xFFFFFF00;
#[rustfmt::skip] pub const fn yellow_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFFFFF8D) }
#[rustfmt::skip] pub const fn yellow_accent_200() -> Rgba8 { Rgba8::init_argb(YELLOW_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn yellow_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFFFEA00) }
#[rustfmt::skip] pub const fn yellow_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFFFD600) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// AMBER

const AMBER_PRIMARY_VALUE: u32 = 0xFFFFC107;
#[rustfmt::skip] pub const fn amber_50()  -> Rgba8 { Rgba8::init_argb(0xFFFFF8E1) }
#[rustfmt::skip] pub const fn amber_100() -> Rgba8 { Rgba8::init_argb(0xFFFFECB3) }
#[rustfmt::skip] pub const fn amber_200() -> Rgba8 { Rgba8::init_argb(0xFFFFE082) }
#[rustfmt::skip] pub const fn amber_300() -> Rgba8 { Rgba8::init_argb(0xFFFFD54F) }
#[rustfmt::skip] pub const fn amber_400() -> Rgba8 { Rgba8::init_argb(0xFFFFCA28) }
#[rustfmt::skip] pub const fn amber_500() -> Rgba8 { Rgba8::init_argb(AMBER_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn amber_600() -> Rgba8 { Rgba8::init_argb(0xFFFFB300) }
#[rustfmt::skip] pub const fn amber_700() -> Rgba8 { Rgba8::init_argb(0xFFFFA000) }
#[rustfmt::skip] pub const fn amber_800() -> Rgba8 { Rgba8::init_argb(0xFFFF8F00) }
#[rustfmt::skip] pub const fn amber_900() -> Rgba8 { Rgba8::init_argb(0xFFFF6F00) }

//----------------------------------------
// AMBER ACCENT

const AMBER_ACCENT_PRIMARY_VALUE: u32 = 0xFFFFD740;
#[rustfmt::skip] pub const fn amber_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFFFE57F) }
#[rustfmt::skip] pub const fn amber_accent_200() -> Rgba8 { Rgba8::init_argb(AMBER_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn amber_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFFFC400) }
#[rustfmt::skip] pub const fn amber_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFFFAB00) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// ORANGE

const ORANGE_PRIMARY_VALUE: u32 = 0xFFFF9800;
#[rustfmt::skip] pub const fn orange_50()  -> Rgba8 { Rgba8::init_argb(0xFFFFF3E0) }
#[rustfmt::skip] pub const fn orange_100() -> Rgba8 { Rgba8::init_argb(0xFFFFE0B2) }
#[rustfmt::skip] pub const fn orange_200() -> Rgba8 { Rgba8::init_argb(0xFFFFCC80) }
#[rustfmt::skip] pub const fn orange_300() -> Rgba8 { Rgba8::init_argb(0xFFFFB74D) }
#[rustfmt::skip] pub const fn orange_400() -> Rgba8 { Rgba8::init_argb(0xFFFFA726) }
#[rustfmt::skip] pub const fn orange_500() -> Rgba8 { Rgba8::init_argb(ORANGE_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn orange_600() -> Rgba8 { Rgba8::init_argb(0xFFFB8C00) }
#[rustfmt::skip] pub const fn orange_700() -> Rgba8 { Rgba8::init_argb(0xFFF57C00) }
#[rustfmt::skip] pub const fn orange_800() -> Rgba8 { Rgba8::init_argb(0xFFEF6C00) }
#[rustfmt::skip] pub const fn orange_900() -> Rgba8 { Rgba8::init_argb(0xFFE65100) }

//----------------------------------------
// ORANGE ACCENT

const ORANGE_ACCENT_PRIMARY_VALUE: u32 = 0xFFFFAB40;
#[rustfmt::skip] pub const fn orange_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFFFD180) }
#[rustfmt::skip] pub const fn orange_accent_200() -> Rgba8 { Rgba8::init_argb(ORANGE_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn orange_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFFF9100) }
#[rustfmt::skip] pub const fn orange_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFFF6D00) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// DEEP ORANGE

const DEEP_ORANGE_PRIMARY_VALUE: u32 = 0xFFFF5722;
#[rustfmt::skip] pub const fn deep_orange_50()  -> Rgba8 { Rgba8::init_argb(0xFFFBE9E7) }
#[rustfmt::skip] pub const fn deep_orange_100() -> Rgba8 { Rgba8::init_argb(0xFFFFCCBC) }
#[rustfmt::skip] pub const fn deep_orange_200() -> Rgba8 { Rgba8::init_argb(0xFFFFAB91) }
#[rustfmt::skip] pub const fn deep_orange_300() -> Rgba8 { Rgba8::init_argb(0xFFFF8A65) }
#[rustfmt::skip] pub const fn deep_orange_400() -> Rgba8 { Rgba8::init_argb(0xFFFF7043) }
#[rustfmt::skip] pub const fn deep_orange_500() -> Rgba8 { Rgba8::init_argb(DEEP_ORANGE_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn deep_orange_600() -> Rgba8 { Rgba8::init_argb(0xFFF4511E) }
#[rustfmt::skip] pub const fn deep_orange_700() -> Rgba8 { Rgba8::init_argb(0xFFE64A19) }
#[rustfmt::skip] pub const fn deep_orange_800() -> Rgba8 { Rgba8::init_argb(0xFFD84315) }
#[rustfmt::skip] pub const fn deep_orange_900() -> Rgba8 { Rgba8::init_argb(0xFFBF360C) }

//----------------------------------------
// DEEP ORANGE ACCENT

const DEEP_ORANGE_ACCENT_PRIMARY_VALUE: u32 = 0xFFFF6E40;
#[rustfmt::skip] pub const fn deep_orange_accent_100() -> Rgba8 { Rgba8::init_argb(0xFFFF9E80) }
#[rustfmt::skip] pub const fn deep_orange_accent_200() -> Rgba8 { Rgba8::init_argb(DEEP_ORANGE_ACCENT_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn deep_orange_accent_400() -> Rgba8 { Rgba8::init_argb(0xFFFF3D00) }
#[rustfmt::skip] pub const fn deep_orange_accent_700() -> Rgba8 { Rgba8::init_argb(0xFFDD2C00) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// BROWN

const BROWN_PRIMARY_VALUE: u32 = 0xFF795548;
#[rustfmt::skip] pub const fn brown_50()  -> Rgba8 { Rgba8::init_argb(0xFFEFEBE9) }
#[rustfmt::skip] pub const fn brown_100() -> Rgba8 { Rgba8::init_argb(0xFFD7CCC8) }
#[rustfmt::skip] pub const fn brown_200() -> Rgba8 { Rgba8::init_argb(0xFFBCAAA4) }
#[rustfmt::skip] pub const fn brown_300() -> Rgba8 { Rgba8::init_argb(0xFFA1887F) }
#[rustfmt::skip] pub const fn brown_400() -> Rgba8 { Rgba8::init_argb(0xFF8D6E63) }
#[rustfmt::skip] pub const fn brown_500() -> Rgba8 { Rgba8::init_argb(BROWN_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn brown_600() -> Rgba8 { Rgba8::init_argb(0xFF6D4C41) }
#[rustfmt::skip] pub const fn brown_700() -> Rgba8 { Rgba8::init_argb(0xFF5D4037) }
#[rustfmt::skip] pub const fn brown_800() -> Rgba8 { Rgba8::init_argb(0xFF4E342E) }
#[rustfmt::skip] pub const fn brown_900() -> Rgba8 { Rgba8::init_argb(0xFF3E2723) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// GREY

const GREY_PRIMARY_VALUE: u32 = 0xFF9E9E9E;
#[rustfmt::skip] pub const fn grey_50()  -> Rgba8 { Rgba8::init_argb(0xFFFAFAFA) }
#[rustfmt::skip] pub const fn grey_100() -> Rgba8 { Rgba8::init_argb(0xFFF5F5F5) }
#[rustfmt::skip] pub const fn grey_200() -> Rgba8 { Rgba8::init_argb(0xFFEEEEEE) }
#[rustfmt::skip] pub const fn grey_300() -> Rgba8 { Rgba8::init_argb(0xFFE0E0E0) }
#[rustfmt::skip] pub const fn grey_350() -> Rgba8 { Rgba8::init_argb(0xFFD6D6D6) }
#[rustfmt::skip] pub const fn grey_400() -> Rgba8 { Rgba8::init_argb(0xFFBDBDBD) }
#[rustfmt::skip] pub const fn grey_500() -> Rgba8 { Rgba8::init_argb(GREY_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn grey_600() -> Rgba8 { Rgba8::init_argb(0xFF757575) }
#[rustfmt::skip] pub const fn grey_700() -> Rgba8 { Rgba8::init_argb(0xFF616161) }
#[rustfmt::skip] pub const fn grey_800() -> Rgba8 { Rgba8::init_argb(0xFF424242) }
#[rustfmt::skip] pub const fn grey_850() -> Rgba8 { Rgba8::init_argb(0xFF303030) }
#[rustfmt::skip] pub const fn grey_900() -> Rgba8 { Rgba8::init_argb(0xFF212121) }

////////////////////////////////////////////////////////////////////////////////////////////////////
// BLUE GREY

const BLUE_GREY_PRIMARY_VALUE: u32 = 0xFF607D8B;
#[rustfmt::skip] pub const fn blue_grey_50()  -> Rgba8 { Rgba8::init_argb(0xFFECEFF1) }
#[rustfmt::skip] pub const fn blue_grey_100() -> Rgba8 { Rgba8::init_argb(0xFFCFD8DC) }
#[rustfmt::skip] pub const fn blue_grey_200() -> Rgba8 { Rgba8::init_argb(0xFFB0BEC5) }
#[rustfmt::skip] pub const fn blue_grey_300() -> Rgba8 { Rgba8::init_argb(0xFF90A4AE) }
#[rustfmt::skip] pub const fn blue_grey_400() -> Rgba8 { Rgba8::init_argb(0xFF78909C) }
#[rustfmt::skip] pub const fn blue_grey_500() -> Rgba8 { Rgba8::init_argb(BLUE_GREY_PRIMARY_VALUE) }
#[rustfmt::skip] pub const fn blue_grey_600() -> Rgba8 { Rgba8::init_argb(0xFF546E7A) }
#[rustfmt::skip] pub const fn blue_grey_700() -> Rgba8 { Rgba8::init_argb(0xFF455A64) }
#[rustfmt::skip] pub const fn blue_grey_800() -> Rgba8 { Rgba8::init_argb(0xFF37474F) }
#[rustfmt::skip] pub const fn blue_grey_900() -> Rgba8 { Rgba8::init_argb(0xFF263238) }

////////////////////////////////////////////////////////////////////////////////////////////////////
