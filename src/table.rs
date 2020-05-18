use std::fmt;

use std::collections::HashMap;

pub struct Row {
    fields: HashMap<String, f32>
}

impl Row {
    pub fn new() -> Row {
        Self {
            fields: HashMap::new()
        }
    }

    pub fn add_field(&mut self, key: String, value: f32){
        self.fields.insert(key, value);
    }

    pub fn get_fields(&self) -> &HashMap<String, f32> {
        &self.fields
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.get_fields())
         .finish()
    }
}

pub struct Table {
    rows: HashMap<String, Row>,
    default_row: Row
}

impl Table {
    pub fn new() -> Table {
        Table {
            rows: HashMap::new(),
            default_row: Row::new()
        }
    }

    pub fn insert_row(&mut self, id: &String, row: Row) {
        self.rows.insert(String::from(id), row);
    }

    pub fn get_rows(&self) -> &HashMap<String, Row> {
        &self.rows
    }

    pub fn add_to_row(&mut self, id: &String, key: String, value: f32) {
        if let None = self.rows.get_mut(id) {
            self.insert_row(id, Row::new())
        }
        if let Some(row) = self.rows.get_mut(id) {
            row.add_field(key, value);
        }
    }

    pub fn get_row_by_id(&self, id: &String) -> &Row {
        match self.rows.get(id) {
            None => &self.default_row,
            Some(row) => row
        }
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.get_rows())
         .finish()
    }
}