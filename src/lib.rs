#![warn(clippy::pedantic)]

#[doc(hidden)]
pub mod __private;

pub use google_sheet_derive::GoogleSheet;
pub use smallmap;

use google_sheets4::api::{CellData, ExtendedValue, GridData};
use std::result::Result::{self, *};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing effective value")]
    MissingEffectiveValue,
    #[error("Missing string value")]
    MissingStringValue,
}

pub trait GoogleSheet<A> {
    /// Extract grid data into a vec of A.
    fn from_grid_data(sheet: &GridData) -> Result<Vec<A>, Error>;
}

pub trait FromExtendedValue {
    fn from_extended_value(value: &ExtendedValue) -> Result<Self, Error>
    where
        Self: Sized;
}

impl FromExtendedValue for String {
    fn from_extended_value(value: &ExtendedValue) -> Result<String, Error> {
        value.string_value.clone().ok_or(Error::MissingStringValue)
    }
}

pub trait FromCellData {
    fn from_cell_data(value: &CellData) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<A: FromExtendedValue + Sized> FromCellData for Option<A> {
    fn from_cell_data(value: &CellData) -> Result<Option<A>, Error> {
        match &value.effective_value {
            Some(v) => FromExtendedValue::from_extended_value(&v).map(|v| Some(v)),
            None => Ok(None),
        }
    }
}

impl<A: FromExtendedValue> FromCellData for A {
    fn from_cell_data(value: &CellData) -> Result<A, Error> {
        value
            .effective_value
            .clone()
            .ok_or(Error::MissingEffectiveValue)
            .and_then(|v| FromExtendedValue::from_extended_value(&v))
    }
}
