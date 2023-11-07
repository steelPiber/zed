use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn solarized() -> UserThemeFamily {
    UserThemeFamily {
        name: "Solarized".into(),
        author: "Ethan Schoonover (altercation)".into(),
        themes: vec![
            UserTheme {
                name: "Solarized Dark".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x003847ff).into()),
                        border_variant: Some(rgba(0x003847ff).into()),
                        border_focused: Some(rgba(0x003847ff).into()),
                        border_selected: Some(rgba(0x003847ff).into()),
                        border_transparent: Some(rgba(0x003847ff).into()),
                        border_disabled: Some(rgba(0x003847ff).into()),
                        background: Some(rgba(0x002a35ff).into()),
                        element_background: Some(rgba(0x29a19899).into()),
                        tab_inactive_background: Some(rgba(0x003f51ff).into()),
                        tab_active_background: Some(rgba(0x002a36ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x586e75ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xcb4b15ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x839496ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0x6c71c4ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x93a1a1ff).into()),
                        terminal_ansi_bright_white: Some(rgba(0x839496ff).into()),
                        terminal_ansi_black: Some(rgba(0x063642ff).into()),
                        terminal_ansi_red: Some(rgba(0xdc312eff).into()),
                        terminal_ansi_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xb58800ff).into()),
                        terminal_ansi_blue: Some(rgba(0x258ad2ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd33582ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x29a198ff).into()),
                        terminal_ansi_white: Some(rgba(0x839496ff).into()),
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Solarized Light".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0xddd6c1ff).into()),
                        border_variant: Some(rgba(0xddd6c1ff).into()),
                        border_focused: Some(rgba(0xddd6c1ff).into()),
                        border_selected: Some(rgba(0xddd6c1ff).into()),
                        border_transparent: Some(rgba(0xddd6c1ff).into()),
                        border_disabled: Some(rgba(0xddd6c1ff).into()),
                        background: Some(rgba(0xfdf6e3ff).into()),
                        element_background: Some(rgba(0xab9d56ff).into()),
                        tab_inactive_background: Some(rgba(0xd3cbb7ff).into()),
                        tab_active_background: Some(rgba(0xfdf6e3ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xcb4b15ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x839496ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0x6c71c4ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x93a1a1ff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xeee8d5ff).into()),
                        terminal_ansi_black: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_red: Some(rgba(0xdc312eff).into()),
                        terminal_ansi_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xb58800ff).into()),
                        terminal_ansi_blue: Some(rgba(0x258ad2ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd33582ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x29a198ff).into()),
                        terminal_ansi_white: Some(rgba(0xeee8d5ff).into()),
                        ..Default::default()
                    },
                },
            },
        ],
    }
}
