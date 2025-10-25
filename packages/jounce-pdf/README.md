# jounce-pdf

PDF generation and document creation for Jounce - create PDFs with text, images, tables, shapes, and more.

## Features

- ✅ **Document Creation** - Create PDF documents with metadata
- ✅ **Page Management** - Multiple pages with custom sizes and orientation
- ✅ **Text Rendering** - Render text with fonts, sizes, and colors
- ✅ **Shapes & Graphics** - Draw rectangles, lines, circles
- ✅ **Images** - Embed images in PDFs
- ✅ **Tables** - Create tables with rows, cells, and styling
- ✅ **Links** - Add clickable hyperlinks
- ✅ **Colors** - Full RGB color support with alpha
- ✅ **Fonts** - Helvetica, Times, Courier with styles
- ✅ **Fluent Builder API** - Chainable PDF construction
- ✅ **Page Sizes** - A4, Letter, Legal, A3, A5, Tabloid, Custom
- ✅ **Utility Functions** - Unit conversions (points, mm, inches)

## Installation

```bash
jnc pkg add jounce-pdf
```

## Quick Start

```jounce
use jounce_pdf::{PdfBuilder, PageSize, Orientation, Font, Color};

// Create a simple PDF
let pdf = PdfBuilder::new()
    .with_title("My Document")
    .with_author("John Doe")
    .add_text("Hello, PDF!", 100.0, 700.0)
    .build();

// Generate PDF output
let output = pdf.generate();
```

## Usage

### Creating Documents

```jounce
use jounce_pdf::{PdfDocument, Page, PageSize, Orientation};

// Create document manually
let doc = PdfDocument::new()
    .with_title("Report 2025")
    .with_author("Jane Smith")
    .with_subject("Monthly Report")
    .new_page(PageSize::A4, Orientation::Portrait);

// Or use builder (recommended)
let doc = PdfBuilder::new()
    .with_title("Report 2025")
    .build();
```

### Page Sizes and Orientation

```jounce
use jounce_pdf::{PageSize, Orientation, Page};

// Available page sizes
let a4 = PageSize::A4;           // 210 x 297 mm
let letter = PageSize::Letter;   // 8.5 x 11 inches
let legal = PageSize::Legal;     // 8.5 x 14 inches
let a3 = PageSize::A3;           // 297 x 420 mm
let a5 = PageSize::A5;           // 148 x 210 mm
let tabloid = PageSize::Tabloid; // 11 x 17 inches

// Get dimensions in points
let (width, height) = a4.dimensions();  // (595.0, 842.0)

// Create page with orientation
let portrait = Page::new(PageSize::A4, Orientation::Portrait);
let landscape = Page::new(PageSize::A4, Orientation::Landscape);

// Custom margins
let page = Page::new(PageSize::Letter, Orientation::Portrait)
    .with_margins(50.0, 50.0, 50.0, 50.0);

// Get content dimensions
let content_width = page.content_width();
let content_height = page.content_height();
```

### Colors

```jounce
use jounce_pdf::Color;

// Create colors
let red = Color::new(255, 0, 0);
let green = Color::new(0, 255, 0);
let blue = Color::new(0, 0, 255);

// Color presets
let black = Color::black();
let white = Color::white();
let red = Color::red();
let green = Color::green();
let blue = Color::blue();
let gray = Color::gray(128);

// With transparency
let semi_transparent = Color::new(255, 0, 0)
    .with_alpha(0.5);

// Convert to RGB string
let rgb = red.to_rgb_string();  // "rgb(255,0,0)"
```

### Fonts

```jounce
use jounce_pdf::{Font, FontFamily, FontStyle};

// Font presets
let helvetica = Font::helvetica(12.0);
let times = Font::times(14.0);
let courier = Font::courier(10.0);

// With styles
let bold = Font::helvetica(12.0)
    .with_style(FontStyle::Bold);

let italic = Font::times(14.0)
    .with_style(FontStyle::Italic);

let bold_italic = Font::helvetica(16.0)
    .with_style(FontStyle::BoldItalic);

// Custom font
let custom = Font::new(FontFamily::Times, 18.0)
    .with_style(FontStyle::Bold);
```

### Adding Text

```jounce
use jounce_pdf::{PdfBuilder, TextElement, Font, Color, TextAlign};

// Simple text
let doc = PdfBuilder::new()
    .add_text("Hello World", 100.0, 700.0)
    .build();

// Text with custom styling
let text = TextElement::new("Styled Text", 100.0, 650.0)
    .with_font(Font::helvetica(16.0).with_style(FontStyle::Bold))
    .with_color(Color::blue())
    .with_align(TextAlign::Center);
```

### Drawing Shapes

```jounce
use jounce_pdf::{Rectangle, Line, Circle, Color};

// Rectangle
let rect = Rectangle::new(50.0, 50.0, 200.0, 100.0)
    .with_fill(Color::blue())
    .with_stroke(Color::black(), 2.0);

// Line
let line = Line::new(0.0, 0.0, 595.0, 0.0)
    .with_color(Color::red())
    .with_width(3.0);

// Circle
let circle = Circle::new(297.5, 421.0, 50.0)
    .with_fill(Color::green())
    .with_stroke(Color::black(), 1.0);

// Add to document
let doc = PdfBuilder::new()
    .add_rectangle(rect)
    .add_line(line)
    .add_circle(circle)
    .build();
```

### Adding Images

```jounce
use jounce_pdf::PdfImage;

// Add image
let image = PdfImage::new("photo.jpg", 100.0, 400.0, 300.0, 200.0);

let doc = PdfBuilder::new()
    .add_image(image)
    .build();
```

### Creating Tables

```jounce
use jounce_pdf::{Table, TableRow, TableCell, Color};

// Create table cells
let header1 = TableCell::new("Name");
let header2 = TableCell::new("Email");
let header3 = TableCell::new("Phone");

// Create header row
let header_row = TableRow::new()
    .add_cell(header1)
    .add_cell(header2)
    .add_cell(header3)
    .with_height(30.0);

// Create data row
let data_row = TableRow::new()
    .add_cell(TableCell::new("John Doe"))
    .add_cell(TableCell::new("john@example.com"))
    .add_cell(TableCell::new("555-1234"));

// Create table
let mut widths = Array::new();
widths.push(150.0);
widths.push(200.0);
widths.push(150.0);

let table = Table::new(50.0, 700.0)
    .with_column_widths(widths)
    .add_row(header_row)
    .add_row(data_row)
    .with_border(1.0, Color::black());

// Add to document
let doc = PdfBuilder::new()
    .add_table(table)
    .build();
```

### Adding Links

```jounce
use jounce_pdf::{Link, Font, Color};

// Create hyperlink
let link = Link::new("Visit Website", "https://example.com", 100.0, 500.0)
    .with_font(Font::helvetica(12.0))
    .with_color(Color::blue());
```

### Multiple Pages

```jounce
use jounce_pdf::{PdfBuilder, PageSize, Orientation};

// Create multi-page document
let doc = PdfBuilder::new()
    .with_title("Multi-Page Report")
    .add_text("Page 1", 100.0, 700.0)
    .add_page(PageSize::A4, Orientation::Portrait)
    .add_text("Page 2", 100.0, 700.0)
    .add_page(PageSize::A4, Orientation::Portrait)
    .add_text("Page 3", 100.0, 700.0)
    .build();

println("Pages: " + doc.page_count().to_string());
```

### Utility Functions

```jounce
use jounce_pdf::{points_to_mm, mm_to_points, points_to_inches, inches_to_points};

// Convert points to millimeters
let mm = points_to_mm(72.0);  // ~25.4mm

// Convert millimeters to points
let points = mm_to_points(210.0);  // A4 width in points

// Convert points to inches
let inches = points_to_inches(72.0);  // 1.0 inch

// Convert inches to points
let points = inches_to_points(8.5);  // Letter width in points
```

### Complete Example: Invoice

```jounce
use jounce_pdf::{PdfBuilder, PageSize, Orientation, Font, FontStyle, Color, Rectangle, Line, Table, TableRow, TableCell, TextAlign};

fn create_invoice() -> PdfDocument {
    // Header line
    let header_line = Line::new(50.0, 750.0, 545.0, 750.0)
        .with_width(2.0)
        .with_color(Color::new(0, 51, 102));

    // Company info box
    let info_box = Rectangle::new(50.0, 680.0, 200.0, 60.0)
        .with_fill(Color::gray(240))
        .no_stroke();

    // Invoice table
    let mut widths = Array::new();
    widths.push(250.0);
    widths.push(100.0);
    widths.push(145.0);

    let header_row = TableRow::new()
        .add_cell(TableCell::new("Description"))
        .add_cell(TableCell::new("Quantity"))
        .add_cell(TableCell::new("Amount"))
        .with_height(30.0);

    let item_row = TableRow::new()
        .add_cell(TableCell::new("Web Development Services"))
        .add_cell(TableCell::new("40 hours"))
        .add_cell(TableCell::new("$4,000.00"));

    let total_row = TableRow::new()
        .add_cell(TableCell::new("").with_colspan(2))
        .add_cell(TableCell::new("Total: $4,000.00")
            .with_background(Color::gray(220)));

    let table = Table::new(50.0, 400.0)
        .with_column_widths(widths)
        .add_row(header_row)
        .add_row(item_row)
        .add_row(total_row)
        .with_border(1.0, Color::black());

    // Build document
    PdfBuilder::new()
        .with_title("Invoice #12345")
        .with_author("Company Name")
        .add_line(header_line)
        .add_text("INVOICE", 250.0, 760.0)
        .add_rectangle(info_box)
        .add_text("Company Name", 60.0, 720.0)
        .add_text("123 Business St", 60.0, 700.0)
        .add_text("City, ST 12345", 60.0, 690.0)
        .add_text("Invoice #: 12345", 400.0, 720.0)
        .add_text("Date: 2025-10-24", 400.0, 700.0)
        .add_table(table)
        .build()
}
```

### Complete Example: Certificate

```jounce
use jounce_pdf::{PdfBuilder, Font, FontStyle, Color, Rectangle, Circle};

fn create_certificate(name: string) -> PdfDocument {
    // Border
    let border = Rectangle::new(30.0, 30.0, 535.0, 782.0)
        .with_stroke(Color::new(0, 51, 102), 3.0)
        .no_fill();

    // Decorative elements
    let corner1 = Circle::new(50.0, 792.0, 10.0)
        .with_fill(Color::new(0, 51, 102));

    let corner2 = Circle::new(545.0, 792.0, 10.0)
        .with_fill(Color::new(0, 51, 102));

    // Seal
    let seal = Circle::new(297.5, 200.0, 50.0)
        .with_fill(Color::new(0, 51, 102))
        .with_stroke(Color::new(0, 51, 102), 2.0);

    PdfBuilder::new()
        .with_title("Certificate of Achievement")
        .add_rectangle(border)
        .add_circle(corner1)
        .add_circle(corner2)
        .add_text("CERTIFICATE", 200.0, 700.0)
        .add_text("OF ACHIEVEMENT", 180.0, 660.0)
        .add_text("This certifies that", 200.0, 550.0)
        .add_text(name, 250.0, 500.0)
        .add_text("has successfully completed", 160.0, 450.0)
        .add_text("Advanced Web Development", 160.0, 410.0)
        .add_circle(seal)
        .build()
}
```

### Complete Example: Report with Graphics

```jounce
use jounce_pdf::{PdfBuilder, Rectangle, Line, Color};

fn create_report() -> PdfDocument {
    // Header section
    let header_bg = Rectangle::new(50.0, 740.0, 495.0, 60.0)
        .with_fill(Color::new(0, 51, 102))
        .no_stroke();

    let section_bg = Rectangle::new(50.0, 600.0, 495.0, 50.0)
        .with_fill(Color::gray(240))
        .no_stroke();

    // Divider lines
    let divider1 = Line::new(50.0, 650.0, 545.0, 650.0)
        .with_width(1.0)
        .with_color(Color::gray(200));

    let divider2 = Line::new(50.0, 550.0, 545.0, 550.0)
        .with_width(1.0)
        .with_color(Color::gray(200));

    PdfBuilder::new()
        .with_title("Monthly Report")
        .with_author("Analytics Team")
        .add_rectangle(header_bg)
        .add_text("MONTHLY REPORT", 180.0, 765.0)
        .add_text("October 2025", 210.0, 750.0)
        .add_rectangle(section_bg)
        .add_text("Executive Summary", 60.0, 620.0)
        .add_line(divider1)
        .add_text("Lorem ipsum dolor sit amet...", 60.0, 630.0)
        .add_line(divider2)
        .add_text("Key Metrics", 60.0, 570.0)
        .add_text("Revenue: $125,000", 60.0, 530.0)
        .add_text("Growth: +15%", 60.0, 510.0)
        .build()
}
```

## API Reference

### PdfDocument

```jounce
PdfDocument::new() -> PdfDocument
document.with_title(title: string) -> PdfDocument
document.with_author(author: string) -> PdfDocument
document.with_subject(subject: string) -> PdfDocument
document.add_page(page: Page) -> PdfDocument
document.new_page(size: PageSize, orientation: Orientation) -> PdfDocument
document.page_count() -> int
document.generate() -> string
```

### PdfBuilder

```jounce
PdfBuilder::new() -> PdfBuilder
builder.with_title(title: string) -> PdfBuilder
builder.with_author(author: string) -> PdfBuilder
builder.add_page(size: PageSize, orientation: Orientation) -> PdfBuilder
builder.add_text(text: string, x: float, y: float) -> PdfBuilder
builder.add_rectangle(rect: Rectangle) -> PdfBuilder
builder.add_line(line: Line) -> PdfBuilder
builder.add_circle(circle: Circle) -> PdfBuilder
builder.add_image(image: PdfImage) -> PdfBuilder
builder.add_table(table: Table) -> PdfBuilder
builder.build() -> PdfDocument
```

### Page

```jounce
Page::new(size: PageSize, orientation: Orientation) -> Page
page.with_margins(top: float, right: float, bottom: float, left: float) -> Page
page.content_width() -> float
page.content_height() -> float
```

### Colors

```jounce
Color::new(r: int, g: int, b: int) -> Color
Color::black() -> Color
Color::white() -> Color
Color::red() -> Color
Color::green() -> Color
Color::blue() -> Color
Color::gray(value: int) -> Color
color.with_alpha(alpha: float) -> Color
color.to_rgb_string() -> string
```

### Fonts

```jounce
Font::new(family: FontFamily, size: float) -> Font
Font::helvetica(size: float) -> Font
Font::times(size: float) -> Font
Font::courier(size: float) -> Font
font.with_style(style: FontStyle) -> Font
font.to_string() -> string
```

### Shapes

```jounce
Rectangle::new(x: float, y: float, width: float, height: float) -> Rectangle
rect.with_fill(color: Color) -> Rectangle
rect.with_stroke(color: Color, width: float) -> Rectangle
rect.no_stroke() -> Rectangle

Line::new(x1: float, y1: float, x2: float, y2: float) -> Line
line.with_color(color: Color) -> Line
line.with_width(width: float) -> Line

Circle::new(x: float, y: float, radius: float) -> Circle
circle.with_fill(color: Color) -> Circle
circle.with_stroke(color: Color, width: float) -> Circle
```

### Tables

```jounce
Table::new(x: float, y: float) -> Table
table.add_row(row: TableRow) -> Table
table.with_column_widths(widths: Array<float>) -> Table
table.with_border(width: float, color: Color) -> Table
table.no_border() -> Table

TableRow::new() -> TableRow
row.add_cell(cell: TableCell) -> TableRow
row.with_height(height: float) -> TableRow

TableCell::new(content: string) -> TableCell
cell.with_colspan(colspan: int) -> TableCell
cell.with_background(color: Color) -> TableCell
```

### Utility Functions

```jounce
points_to_mm(points: float) -> float
mm_to_points(mm: float) -> float
points_to_inches(points: float) -> float
inches_to_points(inches: float) -> float
```

## Coordinate System

PDF uses a coordinate system where:
- Origin (0, 0) is at the **bottom-left** corner
- X increases to the right
- Y increases upward
- Units are in **points** (1 point = 1/72 inch)

For A4 page (595 x 842 points):
- Bottom-left: (0, 0)
- Top-left: (0, 842)
- Top-right: (595, 842)
- Bottom-right: (595, 0)

## Page Sizes

| Size | Width | Height | Points | Common Use |
|------|-------|--------|--------|------------|
| **A4** | 210mm | 297mm | 595 x 842 | International standard |
| **Letter** | 8.5" | 11" | 612 x 792 | US standard |
| **Legal** | 8.5" | 14" | 612 x 1008 | Legal documents |
| **A3** | 297mm | 420mm | 842 x 1191 | Posters, diagrams |
| **A5** | 148mm | 210mm | 420 x 595 | Booklets, flyers |
| **Tabloid** | 11" | 17" | 792 x 1224 | Newspapers |

## Best Practices

1. **Use Builder Pattern** - PdfBuilder is easier than manual construction
2. **Set Metadata** - Always set title, author, and subject
3. **Use Margins** - Leave margins for printing (at least 0.5 inches)
4. **Check Dimensions** - Use content_width() and content_height() for layout
5. **Standard Fonts** - Helvetica, Times, and Courier are always available
6. **RGB Colors** - Use 0-255 for RGB values
7. **Points for Layout** - All measurements are in points (72 DPI)
8. **Test Different Sizes** - Test on both A4 and Letter if targeting global audience

## Examples

See `tests/` directory for comprehensive usage examples including:
- Invoice generation
- Certificate creation
- Multi-page reports
- Tables and forms
- Graphics and shapes

## License

MIT

## Version

0.1.0
