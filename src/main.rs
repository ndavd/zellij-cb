mod line;
mod tab;

use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::convert::TryInto;

use tab::get_tab_to_focus;
use zellij_tile::prelude::*;

use crate::line::tab_line;
use crate::tab::tab_style;

#[derive(Debug, Default)]
pub struct LinePart {
    part: String,
    len: usize,
    tab_index: Option<usize>,
}

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    configuration: BTreeMap<String, String>,
    configuration_colors: ConfigurationColors,
    mode_info: ModeInfo,
    tab_line: Vec<LinePart>,
    session_directory: String,
}

register_plugin!(State);

#[derive(Default, Clone, Debug)]
pub struct ConfigurationColors {
    fg: PaletteColor,
    bg: PaletteColor,
    session_directory: PaletteColor,
    session_name: PaletteColor,
    tab: PaletteColor,
    active_tab: PaletteColor,
    normal_mode: PaletteColor,
    other_modes: PaletteColor,
    others: PaletteColor,
}

impl ConfigurationColors {
    fn str_to_color(str: &String, colors: &Palette) -> Option<PaletteColor> {
        match str.as_str() {
            "fg" => Some(colors.fg),
            "bg" => Some(colors.bg),
            "black" => Some(colors.black),
            "red" => Some(colors.red),
            "green" => Some(colors.green),
            "yellow" => Some(colors.yellow),
            "blue" => Some(colors.blue),
            "magenta" => Some(colors.magenta),
            "cyan" => Some(colors.cyan),
            "white" => Some(colors.white),
            "orange" => Some(colors.orange),
            "gray" => Some(colors.gray),
            "purple" => Some(colors.purple),
            "gold" => Some(colors.gold),
            "silver" => Some(colors.silver),
            "pink" => Some(colors.pink),
            "brown" => Some(colors.brown),
            _ => {
                eprintln!("Failed reading color configuration: Invalid color");
                None
            }
        }
    }
    fn get_color_from_configuration(
        configuration: &BTreeMap<String, String>,
        query: &str,
        fallback_color: PaletteColor,
        colors: &Palette,
    ) -> PaletteColor {
        if let Some(color_string) = configuration.get(query) {
            if let Some(color) = Self::str_to_color(color_string, colors) {
                return color;
            }
        }
        fallback_color
    }
    pub fn populate_from_configuration(
        configuration: &BTreeMap<String, String>,
        colors: &Palette,
    ) -> Self {
        Self {
            fg: Self::get_color_from_configuration(&configuration, "FgColor", colors.white, colors),
            bg: Self::get_color_from_configuration(&configuration, "BgColor", colors.black, colors),
            session_directory: Self::get_color_from_configuration(
                &configuration,
                "SessionDirectoryColor",
                colors.white,
                colors,
            ),
            session_name: Self::get_color_from_configuration(
                &configuration,
                "SessionNameColor",
                colors.gray,
                colors,
            ),
            tab: Self::get_color_from_configuration(
                &configuration,
                "TabColor",
                colors.gray,
                colors,
            ),
            active_tab: Self::get_color_from_configuration(
                &configuration,
                "ActiveTabColor",
                colors.green,
                colors,
            ),
            normal_mode: Self::get_color_from_configuration(
                &configuration,
                "NormalModeColor",
                colors.gold,
                colors,
            ),
            other_modes: Self::get_color_from_configuration(
                &configuration,
                "OtherModesColor",
                colors.orange,
                colors,
            ),
            others: Self::get_color_from_configuration(
                &configuration,
                "OthersColor",
                colors.orange,
                colors,
            ),
        }
    }
}

fn pwd() {
    let mut context = BTreeMap::new();
    context.insert("type".to_string(), "pwd".to_string());
    run_command(&["pwd"], context);
}

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
        ]);
        subscribe(&[
            EventType::TabUpdate,
            EventType::ModeUpdate,
            EventType::Mouse,
            EventType::PermissionRequestResult,
            EventType::RunCommandResult,
        ]);
        self.configuration = _configuration;
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::RunCommandResult(_exit_code, _stdout, _stderr, _context) => {
                if let Some(value) = _context.get("type") {
                    match value.as_ref() {
                        "pwd" => {
                            self.session_directory = std::str::from_utf8(_stdout.as_slice())
                                .unwrap()
                                .trim()
                                .split('/')
                                .last()
                                .unwrap()
                                .to_string();
                        }
                        _ => {}
                    }
                }
                should_render = true;
            }
            Event::ModeUpdate(mode_info) => {
                self.configuration_colors = ConfigurationColors::populate_from_configuration(
                    &self.configuration,
                    &mode_info.style.colors,
                );
                self.mode_info = mode_info;
                should_render = true;
            }
            Event::TabUpdate(tabs) => {
                self.active_tab_idx = tabs.iter().position(|t| t.active).unwrap() + 1;
                self.tabs = tabs;
                should_render = true;
            }
            Event::Mouse(me) => match me {
                Mouse::LeftClick(_, col) => {
                    let tab_to_focus = get_tab_to_focus(&self.tab_line, self.active_tab_idx, col);
                    if let Some(idx) = tab_to_focus {
                        switch_tab_to(idx.try_into().unwrap());
                    }
                }
                Mouse::ScrollUp(_) => {
                    switch_tab_to(min(self.active_tab_idx + 1, self.tabs.len()) as u32);
                }
                Mouse::ScrollDown(_) => {
                    switch_tab_to(max(self.active_tab_idx.saturating_sub(1), 1) as u32);
                }
                _ => {}
            },
            Event::PermissionRequestResult(_) => {
                set_selectable(false);
                pwd();
                switch_to_input_mode(&InputMode::Locked);
            }
            _ => {
                eprintln!("Got unrecognized event: {:?}", event);
            }
        };
        should_render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        if self.tabs.is_empty() {
            return;
        }
        let mut all_tabs: Vec<LinePart> = vec![];
        let mut active_tab_index = 0;
        let mut is_alternate_tab = false; // NOTE: In case I need it in the future
        for t in &mut self.tabs {
            let mut tabname = t.name.clone();
            if t.active && self.mode_info.mode == InputMode::RenameTab {
                if tabname.is_empty() {
                    tabname = String::from("Enter name...");
                }
                active_tab_index = t.position;
            } else if t.active {
                active_tab_index = t.position;
            }
            let tab = tab_style(tabname, t, self.configuration_colors.clone());
            is_alternate_tab = !is_alternate_tab;
            all_tabs.push(tab);
        }
        self.tab_line = tab_line(
            self.mode_info.session_name.as_deref(),
            all_tabs,
            active_tab_index,
            cols.saturating_sub(1),
            self.configuration_colors.clone(),
            self.mode_info.style.hide_session_name,
            self.mode_info.mode,
            self.session_directory.clone(),
        );
        let output = self
            .tab_line
            .iter()
            .fold(String::new(), |output, part| output + &part.part);
        let background = self.configuration_colors.bg;
        match background {
            PaletteColor::Rgb((r, g, b)) => {
                print!("{}\u{1b}[48;2;{};{};{}m\u{1b}[0K", output, r, g, b);
            }
            PaletteColor::EightBit(color) => {
                print!("{}\u{1b}[48;5;{}m\u{1b}[0K", output, color);
            }
        }
    }
}
