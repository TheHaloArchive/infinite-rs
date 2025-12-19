//! Kraken decompressor wrapper.
//!
//! Originally from: <https://github.com/rfuzzo/red4lib>

use crate::Result;
use crate::common::errors::DecompressionError;

#[link(name = "kraken_static")]
unsafe extern "C" {
    // EXPORT int Kraken_Decompress(const byte *src, size_t src_len, byte *dst, size_t dst_len)
    fn Kraken_Decompress(
        buffer: *const u8,
        bufferSize: usize,
        outputBuffer: *mut u8,
        outputBufferSize: usize,
    ) -> i32;
}

/// UNSAFE: Decompresses a Kraken-compressed buffer.
///
/// # Arguments
///
/// * `compressed_buffer` - A vector containing the compressed data.
/// * `output_buffer` - A mutable reference to a vector where the decompressed data will be stored.
/// * `size` - The expected size of the decompressed data.
///
/// # Returns
///
/// Offset of `compressed_buffer` after the compressed data has been read, or -1 if decompression has failed.
///
/// # Errors
/// - If the decompression fails [`DecompressionError::DecompressionFailed`]
/// - If the decompressed buffer size exceeds the maximum size of [`i32`] [`DecompressionError::BufferSizeOverflow`]
/// - If the decompressed buffer size exceeds the maximum size of [`usize`] [`DecompressionError::BufferSizeOverflow`]
///
/// # Safety
///
/// This function is unsafe because it calls an external C function [`Kraken_Decompress`] which operates on raw pointers.
/// The caller must ensure that the `compressed_buffer` and `output_buffer` are valid and properly sized.
pub unsafe fn decompress(
    compressed_buffer: &[u8],
    output_buffer: &mut Vec<u8>,
    size: usize,
) -> Result<i32> {
    unsafe {
        let mut buffer = vec![0; size + 8]; // HACK: Ensures that pointer for memory buffer is aligned.
        let result = Kraken_Decompress(
            compressed_buffer.as_ptr(),
            compressed_buffer.len(),
            buffer.as_mut_ptr(),
            size,
        );

        if result < 0 {
            return Err(DecompressionError::DecompressionFailed(result).into());
        }

        let result_usize =
            usize::try_from(result).map_err(|_| DecompressionError::BufferSizeOverflow)?;

        if result_usize > buffer.len() {
            return Err(DecompressionError::BufferSizeOverflow.into());
        }

        buffer.resize(result_usize, 0);
        *output_buffer = buffer;
        Ok(result)
    }
}
