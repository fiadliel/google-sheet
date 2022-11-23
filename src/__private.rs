use google_sheets4::api::{CellData, RowData};

pub use smallmap;

use crate::{Error, FromExtendedValue};

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

pub fn create_index_map(row_data: &RowData) -> smallmap::Map<String, usize> {
    let mut indexes_for_fields = smallmap::Map::<String, usize>::new();
    for (index, item) in &mut row_data
        .values
        .clone()
        .unwrap_or_default()
        .iter()
        .enumerate()
    {
        if let Some(s) = item
            .effective_value
            .as_ref()
            .and_then(|v| v.string_value.clone())
        {
            indexes_for_fields.insert(s, index);
        };
    }

    indexes_for_fields
}

pub fn get_data<A: FromCellData + Sized>(
    row_data: &RowData,
    index_map: smallmap::Map<String, usize>,
    field_name: &str,
) -> Result<A, Error> {
    if let Some(idx) = index_map.get(field_name) {
        if let Some(cell_data) = row_data
            .values
            .clone()
            .unwrap_or_default()
            .get(idx.to_owned())
        {
            FromCellData::from_cell_data(cell_data)
        } else {
            Err(Error::NoDataInColumn)
        }
    } else {
        Err(Error::ColumnNameNotFound)
    }
}
