fn sobel(input: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>>
{
    let width: u32 = input.width() - 2;
    let height: u32 = input.height() - 2;
    let mut buff: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for i in 0..width {
        for j in 0..height {
            /* Unwrap those loops! */
            let val0 = input.get_pixel(i, j).data[0] as i32;
            let val1 = input.get_pixel(i + 1 , j).data[0] as i32;
            let val2 = input.get_pixel(i + 2, j).data[0] as i32;
            let val3 = input.get_pixel(i, j + 1).data[0] as i32;
            let val5 = input.get_pixel(i + 2, j + 1).data[0] as i32;
            let val6 = input.get_pixel(i, j + 2).data[0] as i32;
            let val7 = input.get_pixel(i + 1, j + 2).data[0] as i32;
            let val8 = input.get_pixel(i + 2, j + 2).data[0] as i32;
            /* Apply Sobel kernels */
            let gx = (-1 * val0) + (-2 * val3) + (-1 * val6) + val2 + (2 * val5) + val8;
            let gy = (-1 * val0) + (-2 * val1) + (-1 * val2) + val6 + (2 * val7) + val8;
            let mut mag = ((gx as f64).powi(2) + (gy as f64).powi(2)).sqrt();

            if mag > 255.0 {
                mag = 255.0;
            }

            buff.put_pixel(i, j, Luma([mag as u8]));
        }
    }

    return buff;
}