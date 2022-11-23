#![warn(clippy::pedantic)]

#[doc(hidden)]
pub mod __private;

pub use google_sheet_derive::GoogleSheet;
pub use smallmap;

use google_sheets4::api::{ExtendedValue, GridData};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing effective value")]
    MissingEffectiveValue,
    #[error("Missing string value")]
    MissingStringValue,
    #[error("No row data")]
    NoRowData,
    #[error("No data in column")]
    NoDataInColumn,
    #[error("Column name not found")]
    ColumnNameNotFound,
}

pub trait GoogleSheet<A> {
    /// Convert `GridData` from a Google spreadsheet into a vector of structs.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data has a shape
    /// that cannot be handled by this library.
    fn from_grid_data(sheet: &GridData) -> Result<Vec<A>, Error>;
}

pub trait FromExtendedValue {
    /// Describe how to convert an `ExtendedValue` into the Self type.
    ///
    /// # Errors
    ///
    /// This function will return an error if the conversion was not possible.
    fn from_extended_value(value: &ExtendedValue) -> Result<Self, Error>
    where
        Self: Sized;
}

impl FromExtendedValue for String {
    fn from_extended_value(value: &ExtendedValue) -> Result<String, Error> {
        value.string_value.clone().ok_or(Error::MissingStringValue)
    }
}
