use std::vec::Vec;

use csv;

mod distance;
mod table;

use crate::table::{Table, Row};
use crate::distance::{Distance};

fn main() {

    //let file = File::open("./Movie_Ratings.csv").expect("Didn't work on read file");
    
    //let mut content = csv::Reader::from_reader(file);

    

    let mut content = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./Movie_Ratings.csv")
        .expect("");

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

        database.insert_row(id.clone(), row);
    }

    let distance_calculator = Distance::Manhattan;
    println!("{}", distance_calculator.calculate(database.get_row_by_id(&String::from("Heather")), database.get_row_by_id(&String::from("Bryan"))));

    //println!("{:#?}", database);

    let mut table = Table::new();

    
    let mut r = Row::new();
    r.add_field(String::from("Hola"), 10.0);

    table.insert_row(String::from("quebin31"), r);

    for i in (0..5) {
        let mut kappa = Row::new();
        for j in (0..3) {
            kappa.add_field(String::from("Field"), 0.9);
        }
        table.insert_row(String::from("Nani"), kappa);
    }
}