# Grocery-to-Json

taking images from grocery reciepts and transforming them into json for logging purposes


## Image Processing Components

- [Sobel Operator](en.wikipedia.org/wiki/Sobel_operator)
- [Hough Transform](https://en.wikipedia.org/wiki/Hough_transform)

## OCR available

- Tesseract

## Text Processors for Grocery stores

- Aldi
- WholeFoods

## Usage

### Building
```Bash
cargo build --release
```

### Running
```Bash
cd target/release
./grocery-to-json

```

### Testing
cargo test --release -- --nocapture
tests will pass even if not 1-1 perfect but should fail at a certain point
use the nocapture flag to show the true accuracy