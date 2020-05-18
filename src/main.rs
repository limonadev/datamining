use std::vec::Vec;

use csv;

mod distance;
mod table;

use crate::table::{Table, Row};
use crate::distance::{Distance};

enum Loader {
    Movies,
    Books,
    Songs
}

impl Loader {
    fn load_data(&self) -> Table {
        match self {
            Self::Books => Self::load_books(),
            Self::Movies => Self::load_movies(),
            Self::Songs => Self::load_songs()
        }
    }

    fn load_books() -> Table {
        let t = Table::new();
        t
    }

    fn load_movies() -> Table {
        let mut content = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("./Movie_Ratings.csv")
            .unwrap();

        let mut matrix = Vec::new();

        for r in content.records() {
            if let Ok(record) = r{
                let mut row = Vec::new();
                for r in record.iter() {
                    row.push(String::from(r));
                }
                matrix.push(row);
            }
        }

        let mut database = Table::new();

        for c in 1..matrix[0].len() {
            let id = &matrix[0][c];

            let mut row = Row::new();
            for r in 1..matrix.len() {
                if matrix[r][c] != "" {
                    row.add_field(matrix[r][0].clone(), matrix[r][c].parse().expect("Failed to parse the value"));   
                }
            }

            database.insert_row(id, row);
        }

        database
    }

    fn load_songs() -> Table {
        let mut content = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("./songs.csv")
            .unwrap();

        let mut matrix = Vec::new();

        for r in content.records() {
            if let Ok(record) = r{
                let mut row = Vec::new();
                for r in record.iter() {
                    row.push(String::from(r));
                }
                matrix.push(row);
            }
        }

        let mut database = Table::new();

        for c in 1..matrix[0].len() {
            let id = &matrix[0][c];

            let mut row = Row::new();
            for r in 1..matrix.len() {
                if matrix[r][c] != "" {
                    row.add_field(matrix[r][0].clone(), matrix[r][c].parse().expect("Failed to parse the value"));   
                }
            }

            database.insert_row(id, row);
        }

        database
    }
}

fn main() {
    
    let database = Loader::Songs.load_data();

    let distance_calculator = Distance::Euclidean;
    //println!("{}", distance_calculator.calculate(database.get_row_by_id(&String::from("Heather")), database.get_row_by_id(&String::from("Bryan"))));
    println!("{}", distance_calculator.calculate(database.get_row_by_id(&String::from("Hailey")), database.get_row_by_id(&String::from("Jordyn"))));
}