# Example Resources

This directory contains example files for demonstrating file upload functionality in BotRS examples.

## Files

### test.png
A sample image file for testing image upload functionality. 

**Note**: Since this is a Git repository, we don't include actual image files. To test file upload examples:

1. Add a PNG image file named `test.png` to this directory
2. The image should be a small test image (recommended size: under 1MB)
3. Common formats supported: PNG, JPG, JPEG, GIF

### Usage in Examples

The file upload examples (`demo_at_reply_file_data.rs`) will look for files in this directory:

```rust
// Method 1: Read file as bytes
let img_bytes = std::fs::read("examples/resource/test.png")?;

// Method 2: Use file path directly
let file_path = "examples/resource/test.png";
```

### Creating Test Files

You can create a simple test image using various methods:

#### Using ImageMagick:
```bash
convert -size 100x100 xc:lightblue examples/resource/test.png
```

#### Using Python:
```python
from PIL import Image
img = Image.new('RGB', (100, 100), color='lightblue')
img.save('examples/resource/test.png')
```

#### Or simply download any small PNG image and rename it to `test.png`

## File Upload Support

BotRS supports uploading various file types:
- Images: PNG, JPG, JPEG, GIF
- Documents: PDF, TXT, DOC, DOCX
- Audio: MP3, WAV, OGG
- Video: MP4, AVI, MOV

File size limits depend on the QQ Guild API restrictions.