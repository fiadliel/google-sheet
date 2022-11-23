#![warn(clippy::pedantic)]

#[doc(hidden)]
pub mod __private;

pub use smallmap;

pub use google_sheet_derive::GoogleSheet;
use google_sheets4::api::{CellData, ExtendedValue, GridData};

pub trait GoogleSheet<A> {
    /// Extract grid data into a vec of A.
    fn from_grid_data(sheet: &GridData) -> std::result::Result<Vec<A>, ()>;
}

pub trait FromExtendedValue {
    fn from_extended_value(value: &ExtendedValue) -> Self;
}

impl FromExtendedValue for String {
    fn from_extended_value(value: &ExtendedValue) -> Self {
        value.string_value.clone().unwrap()
    }
}

pub trait FromCellData {
    fn from_cell_data(value: &CellData) -> Self;
}

impl<A: FromExtendedValue> FromCellData for Option<A> {
    fn from_cell_data(value: &CellData) -> Self {
        value
            .effective_value
            .clone()
            .map(|v| FromExtendedValue::from_extended_value(&v))
    }
}

impl<A: FromExtendedValue> FromCellData for A {
    fn from_cell_data(value: &CellData) -> Self {
        FromExtendedValue::from_extended_value(&value.effective_value.clone().unwrap())
    }
}
