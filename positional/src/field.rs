use pad::{Alignment, PadStr};

/// a trait to represent a type that could be converted to a positional field
///
/// There is a generic implementation for types that implements Display
/// If you want to customize the output on the file you should implement this trait
pub trait ToPositionalField {
    fn to_positional_field(&self) -> String;
}

impl<T: ToString + ?Sized> ToPositionalField for T {
    fn to_positional_field(&self) -> String {
        self.to_string()
    }
}

/// a single field ready to be serialized in a positional row
#[derive(Debug)]
pub struct PositionalField {
    value: String,
    size: usize,
    filler: char,
    align_left: bool,
}

impl PositionalField {
    pub fn new<T: ToPositionalField + ?Sized>(
        value: Option<&T>,
        size: usize,
        filler: char,
        align_left: bool,
    ) -> Self {
        Self {
            value: value.map(|v| v.to_positional_field()).unwrap_or_default(),
            size,
            filler,
            align_left,
        }
    }
}

impl ToString for PositionalField {
    fn to_string(&self) -> String {
        let alignment: Alignment = if self.align_left {
            Alignment::Left
        } else {
            Alignment::Right
        };

        self.value.pad(self.size, self.filler, alignment, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = "scottex";

    #[test]
    fn empty_value_size_zero_to_string() {
        let field = PositionalField::new(Some(""), 0, ' ', true);
        assert!(field.to_string().is_empty());
    }

    #[test]
    fn non_empty_value_size_zero_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 0, ' ', true);
        assert!(field.to_string().is_empty());
    }

    #[test]
    fn empty_value_size_one_to_string() {
        let field = PositionalField::new(Some(""), 1, ' ', true);
        assert_eq!(field.to_string(), " ");
    }

    #[test]
    fn non_empty_value_size_one_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 1, ' ', true);
        assert_eq!(field.to_string(), "s");
    }

    #[test]
    fn non_empty_value_size_one_more_than_value_left_align_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 8, ' ', true);
        assert_eq!(field.to_string(), format!("{} ", TEST_STRING));
    }

    #[test]
    fn non_empty_value_size_one_more_than_value_right_align_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 8, ' ', false);
        assert_eq!(field.to_string(), format!(" {}", TEST_STRING));
    }
}
