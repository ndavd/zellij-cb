use crate::ConfigurationColors;
use crate::LinePart;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

pub fn render_tab(text: String, tab: &TabInfo, colors: ConfigurationColors) -> LinePart {
    let background_color = colors.bg;
    let foreground_color = if tab.active {
        colors.active_tab
    } else {
        colors.tab
    };

    let tab_index = tab.position + 1;
    let text = if text == format!("Tab #{tab_index}") {
        "tab".to_string()
    } else {
        text
    };
    let tab_text = format!("{} {}", tab_index, text);
    let tab_right_padding = " ";
    let tab_left_padding = if tab.position == 0 {
        ""
    } else {
        tab_right_padding
    };
    let tab_text = format!("{tab_left_padding}{tab_text}{tab_right_padding}");
    let tab_text_len = tab_text.len();
    let tab_styled_text = style!(foreground_color, background_color)
        .bold()
        .paint(tab_text);

    LinePart {
        part: tab_styled_text.to_string(),
        len: tab_text_len,
        tab_index: Some(tab.position),
    }
}

pub fn tab_style(mut tabname: String, tab: &TabInfo, colors: ConfigurationColors) -> LinePart {
    if tab.is_sync_panes_active {
        tabname.push_str(" (Sync)");
    }

    render_tab(tabname, tab, colors)
}

pub(crate) fn get_tab_to_focus(
    tab_line: &[LinePart],
    active_tab_idx: usize,
    mouse_click_col: usize,
) -> Option<usize> {
    let clicked_line_part = get_clicked_line_part(tab_line, mouse_click_col)?;
    let clicked_tab_idx = clicked_line_part.tab_index?;
    // tabs are indexed starting from 1 so we need to add 1
    let clicked_tab_idx = clicked_tab_idx + 1;
    if clicked_tab_idx != active_tab_idx {
        return Some(clicked_tab_idx);
    }
    None
}

pub(crate) fn get_clicked_line_part(
    tab_line: &[LinePart],
    mouse_click_col: usize,
) -> Option<&LinePart> {
    let mut len = 0;
    for tab_line_part in tab_line {
        if mouse_click_col >= len && mouse_click_col < len + tab_line_part.len {
            return Some(tab_line_part);
        }
        len += tab_line_part.len;
    }
    None
}
