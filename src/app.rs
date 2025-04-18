use std::error::Error;

use crate::event_log_level::EventLogLevel;
use cleverlib::{clever_parser_options::CleverParserOptions, event_collection::EventCollection};
use ratatui::widgets::{ListState, Row, TableState};

#[derive(Debug, Default)]
pub enum AppState {
    FILTERING,
    #[default]
    ITERATING,
}

#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub counter: u8,
    pub rows: Vec<Row<'a>>,
    pub event_table_state: TableState,
    pub filter_list_state: ListState,
    pub file_path: String,
    pub event_types: Vec<EventLogLevel>,
    pub app_state: AppState,
    pub event_collection: EventCollection,
    pub ignore_parsing_errors: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn load_lines(&mut self, lines: &Vec<String>) -> Result<(), Box<dyn Error>> {
        let parsing_options = CleverParserOptions {
            ignore_errors: Some(self.ignore_parsing_errors),
            debug: Some(false),
        };
        self.event_collection = match EventCollection::create(lines, Some(&parsing_options)) {
            Ok(collection) => collection,
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }

    pub fn get_event_types(&mut self) {
        self.event_types = self
            .event_collection
            .log_levels
            .iter()
            .map(|f| EventLogLevel {
                selected: true,
                value: f.to_string(),
            })
            .collect()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn move_row_up(&mut self, range: usize) {
        if let Some(selected) = self.event_table_state.selected() {
            if selected > range {
                self.event_table_state.select(Some(selected - range));
            } else {
                self.event_table_state
                    .select(self.event_collection.events.len().checked_sub(0));
            }
        }
    }

    pub fn move_row_down(&mut self, range: usize) {
        if let Some(selected) = self.event_table_state.selected() {
            if selected < self.event_collection.events.len() {
                self.event_table_state.select(Some(selected + range));
            } else {
                self.event_table_state.select(Some(0));
            }
        }
    }

    pub fn move_list_up(&mut self) {
        if let Some(selected) = self.filter_list_state.selected() {
            if selected >= 1 {
                self.filter_list_state.select(selected.checked_sub(1));
            } else {
                self.filter_list_state.select(Some(0));
            }
        }
    }

    pub fn move_list_down(&mut self) {
        if let Some(selected) = self.filter_list_state.selected() {
            if selected < self.event_types.len() {
                self.filter_list_state.select(Some(selected + 1));
            } else {
                self.filter_list_state.select(Some(0));
            }
        }
    }
}
