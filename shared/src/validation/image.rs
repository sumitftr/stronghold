use super::ValidationError;

struct ImageData {
    height: u32,
    width: u32,
    format: ImageFormat,
}

#[derive(Copy, Clone)]
enum ImageFormat {
    Jpg,
    Png,
    Webp,
    Gif,
}

pub fn is_icon_valid(filepath: &mut String, data: &bytes::Bytes) -> Result<String, ValidationError> {
    let imgdata = get_image_data(filepath, data)?;
    if imgdata.height != imgdata.width {
        return Err(ValidationError::InvalidData("Icon should be of ratio 1:1".to_string()));
    }
    if imgdata.height < 96 {
        return Err(ValidationError::InvalidData("Icon too small: min 96x96".to_string()));
    }
    if imgdata.height > 2048 {
        return Err(ValidationError::InvalidData("Icon too large: max 2048x2048".to_string()));
    }
    Ok(imgdata.format.to_string())
}

pub fn is_banner_valid(
    filepath: &mut String,
    data: &bytes::Bytes,
) -> Result<String, ValidationError> {
    let imgdata = get_image_data(filepath, data)?;
    Ok(imgdata.format.to_string())
}

fn get_image_data(filepath: &mut String, data: &bytes::Bytes) -> Result<ImageData, ValidationError> {
    let _ = std::mem::replace(filepath, filepath.replace("/", "-"));

    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        if !filepath.ends_with(".jpg") && !filepath.ends_with(".jpeg") {
            filepath.push_str(".jpg");
        }
        return jpeg_resolution(data);
    }

    if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        if !filepath.ends_with(".png") {
            filepath.push_str(".png");
        }
        return png_resolution(data);
    }

    if data.starts_with(&[0x52, 0x49, 0x46, 0x46]) && data[8..12] == [0x57, 0x45, 0x42, 0x50] {
        if !filepath.ends_with(".webp") {
            filepath.push_str(".webp");
        }
        return webp_resolution(data);
    }

    // if data.starts_with(&[0x47, 0x49, 0x46, 0x38])
    //     && data.len() >= 6
    //     && (data[4] == 0x37 || data[4] == 0x39)
    //     && data[5] == 0x61
    // {
    //     if !filepath.ends_with(".gif") {
    //         filepath.push_str(".gif");
    //     }
    //     return Ok(ImageExt::Gif);
    // }

    Err(ValidationError::UnknownImageFormat)
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFormat::Jpg => write!(f, "image/jpeg"),
            ImageFormat::Png => write!(f, "image/png"),
            ImageFormat::Webp => write!(f, "image/webp"),
            ImageFormat::Gif => write!(f, "image/gif"),
        }
    }
}

fn jpeg_resolution(data: &bytes::Bytes) -> Result<ImageData, ValidationError> {
    let len = data.len();

    // JPEG must start with FF D8
    if len < 2 {
        return Err(ValidationError::ImageTooShort { needed: 2, got: len });
    }

    // JPEG SOF markers contain width/height
    let mut i = 2; // skip FF D8
    while i + 9 <= len {
        if data[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = data[i + 1];
        i += 2;

        // SOF0..SOF15 (0xC0-0xCF except 0xC4,0xC8,0xCC)
        if (0xC0..=0xCF).contains(&marker) && marker != 0xC4 && marker != 0xC8 && marker != 0xCC {
            let needed = i + 7;
            if needed > len {
                return Err(ValidationError::ImageTooShort { needed, got: len });
            }
            // Skip 2 bytes (segment length) + 1 byte (precision)
            let height = u16::from_be_bytes([data[i + 3], data[i + 4]]) as u32;
            let width = u16::from_be_bytes([data[i + 5], data[i + 6]]) as u32;
            return Ok(ImageData { height, width, format: ImageFormat::Jpg });
        }
        // skip segment length
        if i + 2 > len {
            break;
        }
        let seg_len = u16::from_be_bytes([data[i], data[i + 1]]) as usize;
        i += seg_len; // len already includes the 2 bytes for the length field itself
    }
    Err(ValidationError::InvalidImageFormat {
        format: "JPEG".to_string(),
        reason: "no Start of Frame marker found".to_string(),
    })
}

fn png_resolution(data: &bytes::Bytes) -> Result<ImageData, ValidationError> {
    let len = data.len();
    // PNG: signature (8 bytes) + chunk length (4) + "IHDR" (4) + IHDR data (13)
    const MIN_SIZE: usize = 29;

    if len < MIN_SIZE {
        return Err(ValidationError::ImageTooShort { needed: MIN_SIZE, got: len });
    }

    // Verify IHDR chunk is present
    if &data[12..16] != b"IHDR" {
        return Err(ValidationError::InvalidImageFormat {
            format: "PNG".to_string(),
            reason: "missing IHDR chunk".to_string(),
        });
    }

    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);

    if width == 0 || height == 0 {
        return Err(ValidationError::InvalidImageFormat {
            format: "PNG".to_string(),
            reason: "width or height is zero".to_string(),
        });
    }

    Ok(ImageData { height, width, format: ImageFormat::Png })
}

fn webp_resolution(data: &bytes::Bytes) -> Result<ImageData, ValidationError> {
    let len = data.len();

    if len < 30 {
        return Err(ValidationError::ImageTooShort { needed: 30, got: len });
    }

    // Check VP8X extended header (most reliable)
    if &data[12..16] == b"VP8X" {
        if len < 26 {
            return Err(ValidationError::ImageTooShort { needed: 26, got: len });
        }
        let w = 1 + ((data[22] as u32) << 16 | (data[21] as u32) << 8 | data[20] as u32);
        let h = 1 + ((data[25] as u32) << 16 | (data[24] as u32) << 8 | data[23] as u32);
        return Ok(ImageData { height: h, width: w, format: ImageFormat::Webp });
    }

    // Fallback: VP8 (lossy)
    if &data[12..16] == b"VP8 " {
        if len < 30 {
            return Err(ValidationError::ImageTooShort { needed: 30, got: len });
        }
        let width = u16::from_le_bytes([data[26], data[27]]) as u32 & 0x3FFF;
        let height = u16::from_le_bytes([data[28], data[29]]) as u32 & 0x3FFF;

        if width == 0 || height == 0 {
            return Err(ValidationError::InvalidImageFormat {
                format: "WebP".to_string(),
                reason: "VP8 width or height is zero".to_string(),
            });
        }

        return Ok(ImageData { height, width, format: ImageFormat::Webp });
    }

    // VP8L (lossless)
    if &data[12..16] == b"VP8L" {
        if len < 25 {
            return Err(ValidationError::ImageTooShort { needed: 25, got: len });
        }
        // Skip signature byte at data[20], start reading at data[21]
        // Bits 0-13: width-1, Bits 14-27: height-1
        let bits = u32::from_le_bytes([data[21], data[22], data[23], data[24]]);
        let w = 1 + (bits & 0x3FFF);
        let h = 1 + ((bits >> 14) & 0x3FFF);

        if w == 1 || h == 1 {
            return Err(ValidationError::InvalidImageFormat {
                format: "WebP".to_string(),
                reason: "VP8L width or height is invalid".to_string(),
            });
        }

        return Ok(ImageData { height: h, width: w, format: ImageFormat::Webp });
    }

    Err(ValidationError::InvalidImageFormat {
        format: "WebP".to_string(),
        reason: "unrecognized chunk type (expected VP8, VP8L, or VP8X)".to_string(),
    })
}

#[allow(dead_code)]
fn gif_resolution(data: &bytes::Bytes) -> Result<ImageData, ValidationError> {
    let len = data.len();
    const MIN_SIZE: usize = 10;

    if len < MIN_SIZE {
        return Err(ValidationError::ImageTooShort { needed: MIN_SIZE, got: len });
    }

    let width = u16::from_le_bytes([data[6], data[7]]) as u32;
    let height = u16::from_le_bytes([data[8], data[9]]) as u32;

    if width == 0 || height == 0 {
        return Err(ValidationError::InvalidImageFormat {
            format: "GIF".to_string(),
            reason: "width or height is zero".to_string(),
        });
    }

    Ok(ImageData { width, height, format: ImageFormat::Gif })
}
