# jounce-xlsx

Excel/XLSX spreadsheet generation for Jounce - create workbooks, worksheets, cells with formatting and formulas.

## Features

- ✅ **Workbooks** - Create Excel workbooks with multiple worksheets
- ✅ **Worksheets** - Add and manage worksheets
- ✅ **Cells** - Text, numbers, booleans, formulas, dates
- ✅ **Styling** - Bold, italic, fonts, colors, borders, alignment
- ✅ **Formulas** - SUM, AVERAGE, COUNT, MAX, MIN, IF, VLOOKUP
- ✅ **Ranges** - Cell range operations (A1:B10)
- ✅ **Tables** - Structured tables with headers and totals
- ✅ **Charts** - Column, bar, line, pie, scatter charts
- ✅ **Formatting** - Row heights, column widths, freeze panes
- ✅ **Fluent Builder API** - Chainable Excel construction

## Installation

```bash
jnc pkg add jounce-xlsx
```

## Quick Start

```jounce
use jounce_xlsx::{ExcelBuilder};

// Create spreadsheet
let workbook = ExcelBuilder::new()
    .add_sheet("Sales")
    .set_cell(1, 1, "Product")
    .set_cell(1, 2, "Price")
    .set_number(2, 1, 29.99)
    .set_number(2, 2, 49.99)
    .build();

workbook.save("sales.xlsx");
```

## Usage

### Creating Workbooks

```jounce
use jounce_xlsx::{Workbook, Worksheet};

// Create workbook
let wb = Workbook::new();

// Add worksheets
let wb = wb.create_worksheet("Sheet1")
           .create_worksheet("Sheet2");

// Save
wb.save("output.xlsx");
```

### Working with Cells

```jounce
use jounce_xlsx::{Cell, Worksheet};

// Text cell
let cell = Cell::text(1, 1, "Hello World");

// Number cell
let cell = Cell::number(2, 1, 42.5);

// Formula cell
let cell = Cell::formula(3, 1, "=SUM(A1:A10)");

// Boolean cell
let cell = Cell::boolean(4, 1, true);

// Add to worksheet
let ws = Worksheet::new("Data").set_cell(cell);
```

### Cell Styling

```jounce
use jounce_xlsx::{Cell, CellStyle, TextAlignment};

// Create styled cell
let style = CellStyle::new()
    .bold()
    .italic()
    .font_size(14)
    .font_color("#FF0000")
    .background("#FFFF00")
    .with_border()
    .align(TextAlignment::Center);

let cell = Cell::text(1, 1, "Styled Text")
    .with_style(style);
```

### Formulas

```jounce
use jounce_xlsx::{FormulaBuilder, Range};

// Create range
let range = Range::new(1, 1, 10, 1);  // A1:A10

// SUM formula
let formula = FormulaBuilder::sum(range);
// =SUM(A1:A10)

// AVERAGE
let formula = FormulaBuilder::average(range);

// COUNT
let formula = FormulaBuilder::count(range);

// MAX/MIN
let max_formula = FormulaBuilder::max(range);
let min_formula = FormulaBuilder::min(range);

// IF
let formula = FormulaBuilder::if_formula("A1>10", "High", "Low");
// =IF(A1>10,High,Low)

// VLOOKUP
let table_range = Range::new(1, 1, 100, 5);
let formula = FormulaBuilder::vlookup("A2", table_range, 3);
```

### Ranges

```jounce
use jounce_xlsx::Range;

// Create range
let range = Range::new(1, 1, 10, 5);  // A1:E10

// Get address
let address = range.to_address();  // "A1:E10"

// Count cells
let count = range.cell_count();  // 50

// Parse from address
let range = Range::from_address("A1:B10");
```

### Tables

```jounce
use jounce_xlsx::{Table, Range};

// Create table
let range = Range::new(1, 1, 100, 3);
let mut headers = Array::new();
headers.push("Name");
headers.push("Age");
headers.push("City");

let table = Table::new("DataTable", range)
    .with_headers(headers)
    .with_totals();

// Column count
let cols = table.column_count();
```

### Charts

```jounce
use jounce_xlsx::{Chart, ChartType, Range};

// Create chart
let data_range = Range::new(1, 1, 10, 2);

let chart = Chart::new(ChartType::Column, data_range)
    .with_title("Monthly Sales")
    .with_size(800, 600);

// Chart types
ChartType::Column    // Column chart
ChartType::Bar       // Bar chart
ChartType::Line      // Line chart
ChartType::Pie       // Pie chart
ChartType::Scatter   // Scatter plot
```

### Row and Column Formatting

```jounce
use jounce_xlsx::{Worksheet, Row, Column};

let ws = Worksheet::new("Data")
    // Set column width
    .set_column_width(1, 20.0)   // Column A = 20 units
    .set_column_width(2, 15.0);  // Column B = 15 units

// Create row with height
let row = Row::new(1).with_height(25.0);
let ws = ws.add_row(row);

// Freeze panes
let ws = ws.freeze_panes(1, 1);  // Freeze first row and column
```

### Excel Builder (Fluent API)

```jounce
use jounce_xlsx::ExcelBuilder;

let workbook = ExcelBuilder::new()
    .add_sheet("Sales Data")
    .set_cell(1, 1, "Product")
    .set_cell(1, 2, "Price")
    .set_cell(1, 3, "Quantity")
    .set_number(2, 1, 29.99)
    .set_number(2, 2, 5.0)
    .set_formula(2, 3, "=A2*B2")
    .set_column_width(1, 15.0)
    .build();
```

### Complete Example: Sales Report

```jounce
use jounce_xlsx::{ExcelBuilder, CellStyle, TextAlignment, FormulaBuilder, Range};

fn create_sales_report() -> Workbook {
    // Header style
    let header_style = CellStyle::new()
        .bold()
        .font_size(12)
        .background("#4472C4")
        .font_color("#FFFFFF")
        .with_border()
        .align(TextAlignment::Center);

    // Build workbook
    let mut builder = ExcelBuilder::new()
        .add_sheet("Q1 Sales");

    // Headers
    builder = builder
        .set_cell(1, 1, "Product")
        .set_cell(1, 2, "Jan")
        .set_cell(1, 3, "Feb")
        .set_cell(1, 4, "Mar")
        .set_cell(1, 5, "Total");

    // Data
    builder = builder
        .set_cell(2, 1, "Widget A")
        .set_number(2, 2, 1000.0)
        .set_number(2, 3, 1200.0)
        .set_number(2, 4, 1100.0)
        .set_formula(2, 5, "=SUM(B2:D2)");

    builder = builder
        .set_cell(3, 1, "Widget B")
        .set_number(3, 2, 800.0)
        .set_number(3, 3, 900.0)
        .set_number(3, 4, 850.0)
        .set_formula(3, 5, "=SUM(B3:D3)");

    // Totals
    builder = builder
        .set_cell(4, 1, "TOTAL")
        .set_formula(4, 2, "=SUM(B2:B3)")
        .set_formula(4, 3, "=SUM(C2:C3)")
        .set_formula(4, 4, "=SUM(D2:D3)")
        .set_formula(4, 5, "=SUM(E2:E3)");

    // Column widths
    builder = builder
        .set_column_width(1, 15.0)
        .set_column_width(2, 12.0)
        .set_column_width(3, 12.0)
        .set_column_width(4, 12.0)
        .set_column_width(5, 12.0);

    builder.build()
}
```

## API Reference

### Workbook
```jounce
Workbook::new() -> Workbook
workbook.add_worksheet(worksheet: Worksheet) -> Workbook
workbook.create_worksheet(name: string) -> Workbook
workbook.get_worksheet(index: int) -> Option<Worksheet>
workbook.get_worksheet_by_name(name: string) -> Option<Worksheet>
workbook.set_active_sheet(index: int) -> Workbook
workbook.worksheet_count() -> int
workbook.save(filename: string) -> bool
```

### Worksheet
```jounce
Worksheet::new(name: string) -> Worksheet
worksheet.set_cell(cell: Cell) -> Worksheet
worksheet.set_cell_value(row: int, column: int, value: string) -> Worksheet
worksheet.set_cell_number(row: int, column: int, number: float) -> Worksheet
worksheet.set_cell_formula(row: int, column: int, formula: string) -> Worksheet
worksheet.get_cell(row: int, column: int) -> Option<Cell>
worksheet.set_column_width(column_index: int, width: float) -> Worksheet
worksheet.freeze_panes(rows: int, columns: int) -> Worksheet
worksheet.cell_count() -> int
```

### Cell
```jounce
Cell::text(row: int, column: int, text: string) -> Cell
Cell::number(row: int, column: int, number: float) -> Cell
Cell::formula(row: int, column: int, formula: string) -> Cell
Cell::boolean(row: int, column: int, value: bool) -> Cell
cell.with_style(style: CellStyle) -> Cell
cell.get_address() -> string
```

### CellStyle
```jounce
CellStyle::new() -> CellStyle
style.bold() -> CellStyle
style.italic() -> CellStyle
style.font_size(size: int) -> CellStyle
style.font_color(color: string) -> CellStyle
style.background(color: string) -> CellStyle
style.with_border() -> CellStyle
style.align(alignment: TextAlignment) -> CellStyle
```

### FormulaBuilder
```jounce
FormulaBuilder::sum(range: Range) -> string
FormulaBuilder::average(range: Range) -> string
FormulaBuilder::count(range: Range) -> string
FormulaBuilder::max(range: Range) -> string
FormulaBuilder::min(range: Range) -> string
FormulaBuilder::if_formula(condition: string, true_val: string, false_val: string) -> string
FormulaBuilder::vlookup(lookup: string, range: Range, col: int) -> string
```

### Range
```jounce
Range::new(start_row: int, start_col: int, end_row: int, end_col: int) -> Range
Range::from_address(address: string) -> Range
range.to_address() -> string
range.cell_count() -> int
```

### ExcelBuilder
```jounce
ExcelBuilder::new() -> ExcelBuilder
builder.add_sheet(name: string) -> ExcelBuilder
builder.set_cell(row: int, col: int, value: string) -> ExcelBuilder
builder.set_number(row: int, col: int, number: float) -> ExcelBuilder
builder.set_formula(row: int, col: int, formula: string) -> ExcelBuilder
builder.set_column_width(col: int, width: float) -> ExcelBuilder
builder.build() -> Workbook
```

## Best Practices

1. **Use Builder Pattern** - ExcelBuilder is easier than manual construction
2. **Set Column Widths** - Improves readability
3. **Use Formulas** - Let Excel calculate values
4. **Style Headers** - Make tables easy to read
5. **Freeze Panes** - For large datasets
6. **Use Tables** - Structured data with filtering
7. **Formula Ranges** - Use Range for formulas

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
