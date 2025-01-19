use ratatui::prelude::*;
use ratatui::widgets::{Block, HighlightSpacing, List, ListItem, Paragraph};

use crate::app::App;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(3), Constraint::Length(30)])
            .split(area);

        let block = Block::bordered();

        let items = self.wifi_info.interfaces().into_iter().map(|interface| {
            if interface.name.is_empty() {
                return ListItem::new("<empty>".to_string());
            }
            ListItem::new(interface.name.clone())
        });

        let list = List::new(items)
            .block(block)
            .highlight_symbol("> ")
            .highlight_style(Style::new().add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(list, chunks[1], buf, &mut self.list_state);

        let title = Line::from(" iwtui ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());

        let selected = self.list_state.selected().unwrap_or(0);
        let interfaces = self.wifi_info.interfaces();
        let selected_interface = interfaces.get(selected);
        if let Some(interface) = selected_interface {
            let mut text = vec![
                format!("Generation: {}", interface.generation),
                format!("Name: {}", interface.name),
                format!("Index: {}", interface.interface_index),
                format!("Type: {:?}", interface.interface_type),
                format!("Mac: {}", interface.mac),
                format!("Channel width: {}", interface.channel_width),
            ];
            if let Some(v) = interface.frequency {
                text.push(format!("Frequency: {v}"));
            }
            if let Some(v) = interface.frequency_offset {
                text.push(format!("Frequency Offset: {v}"));
            }
            if let Some(v) = interface.center_frequency1 {
                text.push(format!("Center Frequency 1: {v}"));
            }
            if let Some(v) = interface.center_frequency2 {
                text.push(format!("Center Frequency 2: {v}"));
            }
            if let Some(v) = interface.tx_power {
                text.push(format!("TX Power: {} dBm", v / 100));
            }
            if let Some(v) = interface.ssid.as_ref() {
                text.push(format!("SSID: {v}"));
            }
            let stations = self.wifi_info.stations();
            for station in stations
                .iter()
                .filter(|s| s.interface_index == interface.interface_index)
            {
                text.push(format!("Station Mac: {}", station.mac));
                if let Some(v) = station.signal {
                    text.push(format!("Station Signal: {v} dBm"));
                }
                if let (Some(b), Some(p)) = (station.tx_bytes64, station.tx_packets) {
                    text.push(format!("TX bytes {b} (packets {p})"));
                }
                if let (Some(b), Some(p)) = (station.rx_bytes64, station.rx_packets) {
                    text.push(format!("RX bytes {b} (packets {p})"));
                }
                if let Some(v) = station.connected_time {
                    text.push(format!("Station Connected Time: {}", v.as_secs()));
                }
                if let Some(v) = station.tx_failed {
                    text.push(format!("TX failed: {v}"));
                }
                if let Some(v) = station.tx_retries {
                    text.push(format!("TX retries: {v}"));
                }
                if let Some(rate_info) = &station.rx_bitrate {
                    text.push(format!("RX bitrate: {} Mb/s", rate_info.bitrate / 10));
                    text.push(format!("RX MCS: {}", rate_info.mcs));
                    text.push(format!("RX type: {}", rate_info.connection_type));
                }
                if let Some(rate_info) = &station.tx_bitrate {
                    text.push(format!("TX bitrate: {} Mb/s", rate_info.bitrate / 10));
                    text.push(format!("TX MCS: {}", rate_info.mcs));
                    text.push(format!("TX type: {}", rate_info.connection_type));
                }
            }

            let interface_text = text.join("\n");

            Paragraph::new(interface_text)
                .left_aligned()
                .block(block)
                .render(chunks[0], buf);
        }
    }
}
