# Example Resources

This directory is reserved for local notes or scratch assets while trying the media examples.

## Files

### test.png
A sample image file for local experiments.

**Note**: The current group/C2C media APIs upload by remote URL (`post_group_file` / `post_c2c_file`), not by reading local bytes directly. To use a local file with the examples, host it somewhere reachable by QQ first, then pass that URL.

1. Add a PNG image file named `test.png` to this directory for manual testing.
2. Upload or serve it from a public URL.
3. Use that URL as the `file_url` in the group/C2C media examples.

### Usage in Examples

The rich media examples use remote URLs. After hosting a local test asset, pass that URL to the example:

```rust
let file_url = "https://example.com/test.png";
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

The platform media upload endpoint accepts a `file_type` plus a remote URL. Common file types include:
- Images: PNG, JPG, JPEG, GIF
- Documents: PDF, TXT, DOC, DOCX
- Audio: MP3, WAV, OGG
- Video: MP4, AVI, MOV

Supported formats and size limits are enforced by the QQ OpenAPI side.
