use crate::doto::Doto;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    View,
    ActionMenu,
    CreateModal,
}

pub struct App {
    pub dotos: Vec<Doto>,
    pub selected_idx: usize,
    pub mode: AppMode,
    pub input_buffer: String,
    pub modal_selection: usize,  // For Create modal: 0=Create, 1=Cancel
    pub undo_stack: Vec<Doto>,
    pub next_id: usize,
    pub menu_selection: usize,
    pub data_path: PathBuf,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let data_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".doto");

        fs::create_dir_all(&data_dir)?;

        let data_path = data_dir.join("tasks.json");

        let mut app = Self {
            dotos: Vec::new(),
            selected_idx: 0,
            mode: AppMode::View,
            input_buffer: String::new(),
            modal_selection: 0,
            undo_stack: Vec::new(),
            next_id: 1,
            menu_selection: 0,
            data_path,
            should_quit: false,
        };

        app.load()?;

        // Ensure we start in View mode
        app.mode = AppMode::View;

        Ok(app)
    }

    pub fn visible_dotos(&self) -> Vec<usize> {
        (0..self.dotos.len()).collect()
    }

    pub fn add_doto(&mut self, text: String) {
        if !text.trim().is_empty() {
            let doto = Doto::new(self.next_id, text);
            self.next_id += 1;
            self.dotos.insert(0, doto);
            self.input_buffer.clear();
            self.save().ok();
        }
    }

    pub fn delete_selected(&mut self) {
        if let Some(visible_idx) = self.visible_dotos().get(self.selected_idx).copied() {
            let doto = self.dotos.remove(visible_idx);
            self.undo_stack.push(doto);
            self.selected_idx = self.selected_idx.saturating_sub(1);
            self.mode = AppMode::View;
            self.save().ok();
        }
    }

    pub fn toggle_selected(&mut self) {
        if let Some(visible_idx) = self.visible_dotos().get(self.selected_idx).copied() {
            // Toggle the doto
            self.dotos[visible_idx].completed = !self.dotos[visible_idx].completed;

            if self.dotos[visible_idx].completed {
                // Move completed doto to bottom
                let completed_doto = self.dotos.remove(visible_idx);
                self.dotos.push(completed_doto);

                // Set selection to next incomplete doto or end of list
                self.selected_idx = self.dotos.iter()
                    .position(|t| !t.completed)
                    .unwrap_or_else(|| {
                    // All dotos are completed or list is empty
                    if !self.dotos.is_empty() {
                        self.dotos.len() - 1
                    } else {
                        0
                    }
                });
            } else {
                // Move uncompleted doto to just after last completed
                let insert_idx = if let Some(last_comp_idx) = self.dotos.iter().rposition(|t| t.completed) {
                    last_comp_idx + 1
                } else {
                    0
                };

                let incomplete_doto = self.dotos.remove(visible_idx);
                self.dotos.insert(insert_idx, incomplete_doto);
                self.selected_idx = insert_idx;
            }

            self.mode = AppMode::View;
            self.save().ok();
        }
    }

    pub fn move_selection(&mut self, delta: isize) {
        let visible_count = self.visible_dotos().len();
        if visible_count == 0 {
            return;
        }

        let new_idx = if delta >= 0 {
            self.selected_idx + delta as usize
        } else {
            self.selected_idx.checked_sub(delta.unsigned_abs() as usize)
                .unwrap_or(0)
        };

        self.selected_idx = new_idx.min(visible_count.saturating_sub(1));
    }

    pub fn move_menu_selection(&mut self, delta: isize) {
        let menu_count = 3; // Complete, Delete, Cancel
        self.menu_selection = (self.menu_selection as isize + delta).rem_euclid(menu_count) as usize;
    }

    pub fn show_action_menu(&mut self) {
        if !self.dotos.is_empty() {
            self.mode = AppMode::ActionMenu;
            self.menu_selection = 0;
        }
    }

    pub fn open_create_modal(&mut self) {
        self.mode = AppMode::CreateModal;
        self.modal_selection = 0;  // Default to Create
        self.input_buffer.clear();
    }

    pub fn close_create_modal(&mut self) {
        self.mode = AppMode::View;
        self.input_buffer.clear();
    }

    pub fn move_modal_selection(&mut self, delta: isize) {
        let options = 2; // Create, Cancel
        self.modal_selection = (self.modal_selection as isize + delta).rem_euclid(options) as usize;
    }

    pub fn execute_create_modal_action(&mut self) {
        match self.modal_selection {
            0 => {
                // Create - add the doto
                self.add_doto(self.input_buffer.clone());
                self.mode = AppMode::View;
            }
            1 => {
                // Cancel
                self.close_create_modal();
            }
            _ => {}
        }
    }

    pub fn execute_menu_action(&mut self) {
        match self.menu_selection {
            0 => self.toggle_selected(),     // Complete
            1 => self.delete_selected(),     // Delete
            2 => self.mode = AppMode::View,  // Cancel
            _ => {}
        }
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.dotos)?;
        fs::write(&self.data_path, json)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        if self.data_path.exists() {
            let json = fs::read_to_string(&self.data_path)?;
            self.dotos = serde_json::from_str(&json)?;
            self.next_id = self.dotos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            dotos: Vec::new(),
            selected_idx: 0,
            mode: AppMode::View,
            input_buffer: String::new(),
            modal_selection: 0,
            undo_stack: Vec::new(),
            next_id: 1,
            menu_selection: 0,
            data_path: PathBuf::from("tasks.json"),
            should_quit: false,
        })
    }
}
