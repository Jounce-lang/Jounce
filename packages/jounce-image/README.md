# jounce-image

Image processing utilities for Jounce - resize, crop, rotate, format conversion, thumbnails, and optimization.

## Features

- ✅ **Multiple Formats** - JPEG, PNG, GIF, WEBP, BMP support
- ✅ **Resize & Scale** - Resize with aspect ratio preservation
- ✅ **Crop & Transform** - Crop, rotate (90°, 180°, 270°), flip
- ✅ **Thumbnail Generation** - Smart thumbnails with aspect ratio
- ✅ **Image Optimization** - Web optimization with quality control
- ✅ **Metadata Management** - Title, description, author, DPI
- ✅ **Fluent Builder API** - Chainable image operations
- ✅ **Format Conversion** - Convert between image formats
- ✅ **Image Filters** - Grayscale, sepia, blur, sharpen, etc.

## Installation

```bash
jnc pkg add jounce-image
```

## Quick Start

```jounce
use jounce_image::{Image, ImageFormat, ThumbnailGenerator, ImageOptimizer};

// Create and resize image
let image = Image::new(1920, 1080, ImageFormat::JPEG)
    .resize_width(800);  // Maintains aspect ratio

// Generate thumbnail
let gen = ThumbnailGenerator::new(200, 200);
let thumb = gen.generate(image);

// Optimize for web
let opt = ImageOptimizer::for_web();
let optimized = opt.optimize(image);

// Builder pattern
let processed = ImageBuilder::new(1600, 1200)
    .resize_width(800)
    .rotate_90()
    .with_format(ImageFormat::WEBP)
    .build();
```

## Usage

### Image Formats

```jounce
use jounce_image::{ImageFormat};

// Supported formats
let jpeg = ImageFormat::JPEG;
let png = ImageFormat::PNG;
let gif = ImageFormat::GIF;
let webp = ImageFormat::WEBP;
let bmp = ImageFormat::BMP;

// Convert from extension
let format = ImageFormat::from_extension("jpg");  // JPEG

// Get extension
let ext = ImageFormat::PNG.to_extension();  // "png"

// Get MIME type
let mime = ImageFormat::JPEG.to_mime_type();  // "image/jpeg"
```

### Creating Images

```jounce
use jounce_image::{Image, ImageFormat};

// Create new image
let image = Image::new(1920, 1080, ImageFormat::PNG);

// Create with data
let image = Image::new(800, 600, ImageFormat::JPEG)
    .with_data(binary_data);

// Get dimensions
let width = image.width;
let height = image.height;

// Get aspect ratio
let ratio = image.aspect_ratio();  // 1.777... for 16:9

// Get file size
let size = image.file_size();  // Length of data in bytes
```

### Resizing Images

```jounce
use jounce_image::Image;

// Resize to exact dimensions
let resized = image.resize(800, 600);

// Resize width (maintains aspect ratio)
let resized = image.resize_width(800);
// If original is 1920x1080, result is 800x450

// Resize height (maintains aspect ratio)
let resized = image.resize_height(600);
// If original is 1920x1080, result is 1067x600
```

### Cropping Images

```jounce
// Crop to region (x, y, width, height)
let cropped = image.crop(100, 100, 400, 300);
// Crops 400x300 region starting at (100, 100)
```

### Rotating Images

```jounce
// Rotate 90 degrees clockwise
let rotated = image.rotate_90();

// Rotate 180 degrees
let rotated = image.rotate_180();

// Rotate 270 degrees (90 counter-clockwise)
let rotated = image.rotate_270();
```

### Flipping Images

```jounce
// Flip horizontally (mirror)
let flipped = image.flip_horizontal();

// Flip vertically
let flipped = image.flip_vertical();
```

### Format Conversion

```jounce
// Convert to different format
let converted = image.convert_format(ImageFormat::WEBP);

// Convert PNG to JPEG
let jpeg = png_image.convert_format(ImageFormat::JPEG);
```

### Thumbnail Generation

```jounce
use jounce_image::ThumbnailGenerator;

// Create generator
let gen = ThumbnailGenerator::new(200, 200);

// Generate thumbnail (maintains aspect ratio by default)
let thumb = gen.generate(image);
// If original is 1920x1080, thumbnail is 200x112

// Custom quality
let gen = ThumbnailGenerator::new(200, 200)
    .with_quality(90);

// Disable aspect ratio (exact dimensions)
let gen = ThumbnailGenerator::new(200, 200)
    .without_aspect_ratio();

let thumb = gen.generate(image);
// Always exactly 200x200 (may distort)
```

### Image Optimization

```jounce
use jounce_image::{ImageOptimizer, ImageFormat};

// Create optimizer with defaults
let opt = ImageOptimizer::new();
// Quality: 85, Max: 2048x2048, Format: JPEG

// Optimize for web (preset)
let opt = ImageOptimizer::for_web();
// Quality: 80, Max: 1920x1080, Format: WEBP

// Custom optimization
let opt = ImageOptimizer::new()
    .with_format(ImageFormat::WEBP)
    .with_quality(90)
    .with_max_dimensions(1024, 768)
    .strip_metadata();

// Optimize image
let optimized = opt.optimize(image);
// Resizes if too large, converts format, strips metadata
```

### Image Metadata

```jounce
use jounce_image::ImageMetadata;

// Create metadata
let metadata = ImageMetadata::new()
    .with_title("Sunset Photo")
    .with_description("Beautiful sunset over mountains")
    .with_author("John Doe")
    .with_dpi(300);

// Attach to image
let mut image = Image::new(800, 600, ImageFormat::JPEG);
image.metadata = metadata;

// Access metadata
println("Title: " + image.metadata.title);
println("Author: " + image.metadata.author);
println("DPI: " + image.metadata.dpi.to_string());
```

### Image Dimensions

```jounce
use jounce_image::Dimensions;

// Create dimensions
let dim = Dimensions::new(1920, 1080);

// Calculate aspect ratio
let ratio = dim.aspect_ratio();  // 1.777...

// Get total pixels
let pixels = dim.total_pixels();  // 2,073,600

// Check orientation
if dim.is_landscape() {
    println("Landscape image");
}

if dim.is_portrait() {
    println("Portrait image");
}

if dim.is_square() {
    println("Square image");
}
```

### Image Builder (Fluent API)

```jounce
use jounce_image::{ImageBuilder, ImageFormat, ImageMetadata};

// Chain multiple operations
let image = ImageBuilder::new(1920, 1080)
    .with_format(ImageFormat::PNG)
    .resize_width(800)
    .rotate_90()
    .crop(0, 0, 400, 400)
    .with_data(binary_data)
    .build();

// Start from existing image
let processed = ImageBuilder::from_image(original_image)
    .resize(400, 300)
    .with_format(ImageFormat::WEBP)
    .build();

// Add metadata
let metadata = ImageMetadata::new().with_title("Processed");
let image = ImageBuilder::new(800, 600)
    .with_metadata(metadata)
    .build();
```

### Image Filters

```jounce
use jounce_image::{ImageFilter, FilterConfig};

// Available filters
let grayscale = ImageFilter::Grayscale;
let sepia = ImageFilter::Sepia;
let blur = ImageFilter::Blur;
let sharpen = ImageFilter::Sharpen;
let brightness = ImageFilter::Brightness;
let contrast = ImageFilter::Contrast;
let invert = ImageFilter::Invert;

// Create filter config
let config = FilterConfig::new(ImageFilter::Blur)
    .with_intensity(0.5);

// Apply filter (in future versions)
// let filtered = image.apply_filter(config);
```

### Utility Functions

```jounce
use jounce_image::{calculate_resize_dimensions, is_valid_dimension, is_valid_quality};

// Calculate resize dimensions with aspect ratio
let dim = calculate_resize_dimensions(1920, 1080, 640, 480);
// Returns Dimensions(640, 360) - maintains aspect ratio

// Validate dimensions
if is_valid_dimension(width) {
    // Valid: 1 to 10,000 pixels
}

// Validate quality
if is_valid_quality(quality) {
    // Valid: 1 to 100
}
```

### Complete Example: Photo Gallery

```jounce
use jounce_image::{Image, ImageFormat, ThumbnailGenerator, ImageOptimizer};

fn process_upload(photo_data: string, filename: string) -> Result<ProcessedPhoto, string> {
    // Create image from uploaded data
    let image = Image::new(0, 0, ImageFormat::from_extension(filename))
        .with_data(photo_data);

    // Generate thumbnail for gallery view
    let thumb_gen = ThumbnailGenerator::new(300, 300);
    let thumbnail = thumb_gen.generate(image.clone());

    // Optimize full image for web
    let optimizer = ImageOptimizer::for_web()
        .with_quality(85)
        .strip_metadata();

    let optimized = optimizer.optimize(image);

    // Return processed images
    Ok(ProcessedPhoto {
        full: optimized,
        thumbnail: thumbnail,
    })
}

struct ProcessedPhoto {
    pub full: Image,
    pub thumbnail: Image,
}
```

### Complete Example: Image Editor

```jounce
use jounce_image::{ImageBuilder, ImageFormat, ImageMetadata};

fn edit_photo(original: Image, operations: EditOperations) -> Image {
    let mut builder = ImageBuilder::from_image(original);

    // Apply resize if specified
    if operations.resize_width > 0 {
        builder = builder.resize_width(operations.resize_width);
    }

    // Apply rotation if specified
    if operations.rotate == 90 {
        builder = builder.rotate_90();
    } else if operations.rotate == 180 {
        builder = builder.rotate_180();
    } else if operations.rotate == 270 {
        builder = builder.rotate_270();
    }

    // Apply crop if specified
    if operations.crop_enabled {
        builder = builder.crop(
            operations.crop_x,
            operations.crop_y,
            operations.crop_width,
            operations.crop_height
        );
    }

    // Convert format if specified
    if operations.output_format != ImageFormat::PNG {
        builder = builder.with_format(operations.output_format);
    }

    // Add metadata
    let metadata = ImageMetadata::new()
        .with_title(operations.title)
        .with_author("Image Editor");

    builder.with_metadata(metadata).build()
}

struct EditOperations {
    pub resize_width: int,
    pub rotate: int,
    pub crop_enabled: bool,
    pub crop_x: int,
    pub crop_y: int,
    pub crop_width: int,
    pub crop_height: int,
    pub output_format: ImageFormat,
    pub title: string,
}
```

### Complete Example: Profile Avatar

```jounce
use jounce_image::{Image, ImageFormat, ThumbnailGenerator};

fn create_avatar(uploaded_image: Image) -> Image {
    // Create square thumbnail
    let gen = ThumbnailGenerator::new(150, 150)
        .without_aspect_ratio()  // Force square
        .with_quality(90);

    let avatar = gen.generate(uploaded_image);

    // Convert to WEBP for smaller size
    avatar.convert_format(ImageFormat::WEBP)
}
```

## API Reference

### ImageFormat

```jounce
enum ImageFormat {
    JPEG, PNG, GIF, WEBP, BMP
}

ImageFormat::from_extension(ext: string) -> ImageFormat
ImageFormat::to_extension() -> string
ImageFormat::to_mime_type() -> string
```

### Dimensions

```jounce
struct Dimensions {
    width: int,
    height: int,
}

Dimensions::new(width: int, height: int) -> Dimensions
dimensions.aspect_ratio() -> float
dimensions.total_pixels() -> int
dimensions.is_landscape() -> bool
dimensions.is_portrait() -> bool
dimensions.is_square() -> bool
```

### Image

```jounce
struct Image {
    width: int,
    height: int,
    format: ImageFormat,
    data: string,
    metadata: ImageMetadata,
}

Image::new(width: int, height: int, format: ImageFormat) -> Image
image.with_data(data: string) -> Image
image.dimensions() -> Dimensions
image.aspect_ratio() -> float
image.file_size() -> int
image.resize(new_width: int, new_height: int) -> Image
image.resize_width(new_width: int) -> Image
image.resize_height(new_height: int) -> Image
image.crop(x: int, y: int, width: int, height: int) -> Image
image.rotate_90() -> Image
image.rotate_180() -> Image
image.rotate_270() -> Image
image.flip_horizontal() -> Image
image.flip_vertical() -> Image
image.convert_format(format: ImageFormat) -> Image
```

### ImageMetadata

```jounce
struct ImageMetadata {
    title: string,
    description: string,
    author: string,
    created_at: int,
    dpi: int,
    color_space: string,
}

ImageMetadata::new() -> ImageMetadata
metadata.with_title(title: string) -> ImageMetadata
metadata.with_description(description: string) -> ImageMetadata
metadata.with_author(author: string) -> ImageMetadata
metadata.with_dpi(dpi: int) -> ImageMetadata
```

### ThumbnailGenerator

```jounce
struct ThumbnailGenerator {
    max_width: int,
    max_height: int,
    maintain_aspect: bool,
    quality: int,
}

ThumbnailGenerator::new(max_width: int, max_height: int) -> ThumbnailGenerator
generator.with_quality(quality: int) -> ThumbnailGenerator
generator.without_aspect_ratio() -> ThumbnailGenerator
generator.generate(image: Image) -> Image
```

### ImageOptimizer

```jounce
struct ImageOptimizer {
    target_format: ImageFormat,
    quality: int,
    max_width: int,
    max_height: int,
    strip_metadata: bool,
}

ImageOptimizer::new() -> ImageOptimizer
ImageOptimizer::for_web() -> ImageOptimizer
optimizer.with_format(format: ImageFormat) -> ImageOptimizer
optimizer.with_quality(quality: int) -> ImageOptimizer
optimizer.with_max_dimensions(width: int, height: int) -> ImageOptimizer
optimizer.strip_metadata() -> ImageOptimizer
optimizer.optimize(image: Image) -> Image
```

### ImageBuilder

```jounce
struct ImageBuilder {
    image: Image,
}

ImageBuilder::new(width: int, height: int) -> ImageBuilder
ImageBuilder::from_image(image: Image) -> ImageBuilder
builder.with_format(format: ImageFormat) -> ImageBuilder
builder.with_data(data: string) -> ImageBuilder
builder.resize(width: int, height: int) -> ImageBuilder
builder.resize_width(width: int) -> ImageBuilder
builder.resize_height(height: int) -> ImageBuilder
builder.crop(x: int, y: int, width: int, height: int) -> ImageBuilder
builder.rotate_90() -> ImageBuilder
builder.rotate_180() -> ImageBuilder
builder.rotate_270() -> ImageBuilder
builder.flip_horizontal() -> ImageBuilder
builder.flip_vertical() -> ImageBuilder
builder.with_metadata(metadata: ImageMetadata) -> ImageBuilder
builder.build() -> Image
```

### ImageFilter

```jounce
enum ImageFilter {
    Grayscale, Sepia, Blur, Sharpen,
    Brightness, Contrast, Invert,
}

struct FilterConfig {
    filter: ImageFilter,
    intensity: float,
}

FilterConfig::new(filter: ImageFilter) -> FilterConfig
config.with_intensity(intensity: float) -> FilterConfig
```

### Utility Functions

```jounce
calculate_resize_dimensions(
    original_width: int,
    original_height: int,
    max_width: int,
    max_height: int
) -> Dimensions

is_valid_dimension(dimension: int) -> bool
is_valid_quality(quality: int) -> bool
```

## Best Practices

1. **Preserve Aspect Ratio** - Use `resize_width()` or `resize_height()` for proportional scaling
2. **Optimize for Web** - Use `ImageOptimizer::for_web()` preset for web images
3. **Use WEBP** - Modern format with better compression than JPEG/PNG
4. **Thumbnail Quality** - Use quality 85-90 for thumbnails
5. **Strip Metadata** - Remove metadata for privacy and smaller file sizes
6. **Validate Dimensions** - Use `is_valid_dimension()` before processing
7. **Use Builder** - Chain multiple operations with `ImageBuilder` for clarity

## Performance Tips

- Resize before applying filters for better performance
- Use appropriate quality settings (80-85 for web, 90-95 for print)
- Convert to WEBP for 25-35% smaller file sizes
- Generate thumbnails once and cache them
- Strip metadata when not needed

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
