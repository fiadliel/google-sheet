# google-sheet

[Documentation](https://fiadliel.github.io/google-sheet/)

## Introduction

This library does some processing of a Google spreadsheet, and fills a vec of a predefined structure with data read from it.
The values added to the structure are done by matching the name of the field with the name of the column (as defined by the value in the first row).

## Usage

Annotate a structure:

```rust
#[derive(GoogleSheet)]
struct MyStruct {
  name: String,
  organization: Option<String>
}
```

This will derive an implementation:
```rust
let grid_data: &GridData = ...;

let _values: Result<Vec<MyStruct>, Error> = MyStruct::from_grid_data(&grid_data);
```
