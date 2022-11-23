#![warn(clippy::pedantic)]

pub use google_sheet_derive::GoogleSheet;
use google_sheets4::api::{CellData, ExtendedValue, GridData, RowData};

pub trait GoogleSheet<A> {
    /// Extract grid data into a vec of A.
    fn from_grid_data(sheet: &GridData) -> std::result::Result<Vec<A>, ()>;
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

pub fn get_data<A: FromCellData>(
    row_data: &RowData,
    index_map: &::smallmap::Map<String, usize>,
    field_name: &str,
) -> A {
    if let Some(idx) = index_map.get(field_name) {
        if let Some(cell_data) = row_data
            .values
            .clone()
            .unwrap_or_default()
            .get(idx.to_owned())
        {
            FromCellData::from_cell_data(cell_data)
        } else {
            todo!()
        }
    } else {
        todo!()
    }
}
