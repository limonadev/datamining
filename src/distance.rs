use crate::table::{Row};

pub enum Distance {
    Minkowski {
        grade: i32
    },
    Manhattan,
    Euclidean
}

impl Distance {
    pub fn calculate(&self, first: &Row, second: &Row) -> f32 {
        match self {
            Self::Manhattan => Distance::Minkowski{grade: 1}.calculate(first, second),
            Self::Euclidean => Distance::Minkowski{grade: 2}.calculate(first, second),
            Self::Minkowski{grade} => {
                Self::calculate_minkowski(*grade, &first, &second)
            }
        }
    }

    fn calculate_minkowski(grade:i32, first: &Row, second: &Row) -> f32 {
        let first_fields = first.get_fields();
        let second_fields = second.get_fields();

        let mut total = 0.0;
        for (field, first_value) in first_fields {
            if let Some(second_value) = second_fields.get(field) {
                let diff = (first_value-second_value).abs().powi(grade);
                total += diff;
            }
        }
        total.powf(1.0/(grade as f32))
    }
}