use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn synthwave_84() -> UserThemeFamily {
    UserThemeFamily {
        name: "Synthwave 84".into(),
        author: "Robb Owen (robb0wen)".into(),
        themes: vec![UserTheme {
            name: "Synthwave 84".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    background: Some(rgba(0x252334ff).into()),
                    element_background: Some(rgba(0x614d85ff).into()),
                    text: Some(rgba(0xffffffff).into()),
                    tab_inactive_background: Some(rgba(0x252334ff).into()),
                    terminal_ansi_bright_red: Some(rgba(0xfe444fff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x71f1b7ff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0xfede5cff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x02edf9ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0xff7ddaff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x02edf9ff).into()),
                    terminal_ansi_red: Some(rgba(0xfe444fff).into()),
                    terminal_ansi_green: Some(rgba(0x71f1b7ff).into()),
                    terminal_ansi_yellow: Some(rgba(0xf3e70fff).into()),
                    terminal_ansi_blue: Some(rgba(0x02edf9ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xff7ddaff).into()),
                    terminal_ansi_cyan: Some(rgba(0x02edf9ff).into()),
                    ..Default::default()
                },
            },
        }],
    }
}
