use google_sheets4::api::RowData;

use crate::{Error, FromCellData};

pub fn create_index_map(row_data: &RowData) -> crate::smallmap::Map<String, usize> {
    let mut indexes_for_fields = crate::smallmap::Map::<String, usize>::new();
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
    index_map: &crate::smallmap::Map<String, usize>,
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
            todo!()
        }
    } else {
        todo!()
    }
}
