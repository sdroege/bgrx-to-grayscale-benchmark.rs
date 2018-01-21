#![feature(test)]
#![feature(exact_chunks)]

extern crate test;

const RGB_Y: [u32; 4] = [19595, 38470, 7471, 0];

pub fn bgrx_to_gray_chunks_no_asserts(
    in_data: &[u8],
    out_data: &mut [u8],
    in_stride: usize,
    out_stride: usize,
    width: usize,
) {
    let in_line_bytes = width * 4;
    let out_line_bytes = width * 4;

    for (in_line, out_line) in in_data
        .chunks(in_stride)
        .zip(out_data.chunks_mut(out_stride))
    {
        for (in_p, out_p) in in_line[..in_line_bytes]
            .chunks(4)
            .zip(out_line[..out_line_bytes].chunks_mut(4))
        {
            let b = u32::from(in_p[0]);
            let g = u32::from(in_p[1]);
            let r = u32::from(in_p[2]);
            let x = u32::from(in_p[3]);

            let grey = ((r * RGB_Y[0]) + (g * RGB_Y[1]) + (b * RGB_Y[2]) + (x * RGB_Y[3])) / 65536;
            let grey = grey as u8;
            out_p[0] = grey;
            out_p[1] = grey;
            out_p[2] = grey;
            out_p[3] = grey;
        }
    }
}

pub fn bgrx_to_gray_chunks_asserts(
    in_data: &[u8],
    out_data: &mut [u8],
    in_stride: usize,
    out_stride: usize,
    width: usize,
) {
    assert_eq!(in_data.len() % 4, 0);
    assert_eq!(out_data.len() % 4, 0);
    assert_eq!(out_data.len() / out_stride, in_data.len() / in_stride);

    let in_line_bytes = width * 4;
    let out_line_bytes = width * 4;

    assert!(in_line_bytes <= in_stride);
    assert!(out_line_bytes <= out_stride);

    for (in_line, out_line) in in_data
        .chunks(in_stride)
        .zip(out_data.chunks_mut(out_stride))
    {
        for (in_p, out_p) in in_line[..in_line_bytes]
            .chunks(4)
            .zip(out_line[..out_line_bytes].chunks_mut(4))
        {
            assert_eq!(in_p.len(), 4);
            assert_eq!(out_p.len(), 4);

            let b = u32::from(in_p[0]);
            let g = u32::from(in_p[1]);
            let r = u32::from(in_p[2]);
            let x = u32::from(in_p[3]);

            let grey = ((r * RGB_Y[0]) + (g * RGB_Y[1]) + (b * RGB_Y[2]) + (x * RGB_Y[3])) / 65536;
            let grey = grey as u8;
            out_p[0] = grey;
            out_p[1] = grey;
            out_p[2] = grey;
            out_p[3] = grey;
        }
    }
}

pub fn bgrx_to_gray_chunks_asserts_2(
    in_data: &[u8],
    out_data: &mut [u8],
    in_stride: usize,
    out_stride: usize,
    width: usize,
) {
    assert_eq!(in_data.len() % 4, 0);
    assert_eq!(out_data.len() % 4, 0);
    assert_eq!(out_data.len() / out_stride, in_data.len() / in_stride);

    let in_line_bytes = width * 4;
    let out_line_bytes = width * 4;

    assert!(in_line_bytes <= in_stride);
    assert!(out_line_bytes <= out_stride);

    for (in_line, out_line) in in_data
        .chunks(in_stride)
        .zip(out_data.chunks_mut(out_stride))
    {
        for (in_p, out_p) in in_line[..in_line_bytes]
            .chunks(4)
            .zip(out_line[..out_line_bytes].chunks_mut(4))
        {
            assert!(in_p.len() == 4);
            assert!(out_p.len() == 4);

            let b = u32::from(in_p[0]);
            let g = u32::from(in_p[1]);
            let r = u32::from(in_p[2]);
            let x = u32::from(in_p[3]);

            let grey = ((r * RGB_Y[0]) + (g * RGB_Y[1]) + (b * RGB_Y[2]) + (x * RGB_Y[3])) / 65536;
            let grey = grey as u8;
            out_p[0] = grey;
            out_p[1] = grey;
            out_p[2] = grey;
            out_p[3] = grey;
        }
    }
}

pub fn bgrx_to_gray_chunks_iter_sum(
    in_data: &[u8],
    out_data: &mut [u8],
    in_stride: usize,
    out_stride: usize,
    width: usize,
) {
    assert_eq!(in_data.len() % 4, 0);
    assert_eq!(out_data.len() % 4, 0);
    assert_eq!(out_data.len() / out_stride, in_data.len() / in_stride);

    let in_line_bytes = width * 4;
    let out_line_bytes = width * 4;

    assert!(in_line_bytes <= in_stride);
    assert!(out_line_bytes <= out_stride);

    for (in_line, out_line) in in_data
        .chunks(in_stride)
        .zip(out_data.chunks_mut(out_stride))
    {
        for (in_p, out_p) in in_line[..in_line_bytes]
            .chunks(4)
            .zip(out_line[..out_line_bytes].chunks_mut(4))
        {
            assert!(out_p.len() == 4);

            let grey = in_p.iter()
                .zip(RGB_Y.iter())
                .map(|(i, c)| u32::from(*i) * c)
                .sum::<u32>() / 65536;
            let grey = grey as u8;
            out_p[0] = grey;
            out_p[1] = grey;
            out_p[2] = grey;
            out_p[3] = grey;
        }
    }
}

pub fn bgrx_to_gray_chunks_iter_sum_2(
    in_data: &[u8],
    out_data: &mut [u8],
    in_stride: usize,
    out_stride: usize,
    width: usize,
) {
    assert_eq!(in_data.len() % 4, 0);
    assert_eq!(out_data.len() % 4, 0);
    assert_eq!(out_data.len() / out_stride, in_data.len() / in_stride);

    let in_line_bytes = width * 4;
    let out_line_bytes = width * 4;

    assert!(in_line_bytes <= in_stride);
    assert!(out_line_bytes <= out_stride);

    for (in_line, out_line) in in_data
        .chunks(in_stride)
        .zip(out_data.chunks_mut(out_stride))
    {
        for (in_p, out_p) in in_line[..in_line_bytes]
            .chunks(4)
            .zip(out_line[..out_line_bytes].chunks_mut(4))
        {
            assert!(in_p.len() == 4);
            assert!(out_p.len() == 4);

            let grey = in_p.iter()
                .zip(RGB_Y.iter())
                .map(|(i, c)| u32::from(*i) * c)
                .sum::<u32>() / 65536;
            let grey = grey as u8;
            out_p[0] = grey;
            out_p[1] = grey;
            out_p[2] = grey;
            out_p[3] = grey;
        }
    }
}

pub fn bgrx_to_gray_exact_chunks(
    in_data: &[u8],
    out_data: &mut [u8],
    in_stride: usize,
    out_stride: usize,
    width: usize,
) {
    assert_eq!(in_data.len() % 4, 0);
    assert_eq!(out_data.len() % 4, 0);
    assert_eq!(out_data.len() / out_stride, in_data.len() / in_stride);

    let in_line_bytes = width * 4;
    let out_line_bytes = width * 4;

    assert!(in_line_bytes <= in_stride);
    assert!(out_line_bytes <= out_stride);

    for (in_line, out_line) in in_data
        .exact_chunks(in_stride)
        .zip(out_data.exact_chunks_mut(out_stride))
    {
        for (in_p, out_p) in in_line[..in_line_bytes]
            .exact_chunks(4)
            .zip(out_line[..out_line_bytes].exact_chunks_mut(4))
        {
            assert!(in_p.len() == 4);
            assert!(out_p.len() == 4);

            let b = u32::from(in_p[0]);
            let g = u32::from(in_p[1]);
            let r = u32::from(in_p[2]);
            let x = u32::from(in_p[3]);

            let grey = ((r * RGB_Y[0]) + (g * RGB_Y[1]) + (b * RGB_Y[2]) + (x * RGB_Y[3])) / 65536;
            let grey = grey as u8;
            out_p[0] = grey;
            out_p[1] = grey;
            out_p[2] = grey;
            out_p[3] = grey;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::iter;

    fn create_vec(w: usize, h: usize) -> Vec<u8> {
        iter::repeat(0).take(w * h * 4).collect::<_>()
    }

    #[bench]
    fn bench_chunks_1920x1080_no_asserts(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_chunks_no_asserts(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }

    #[bench]
    fn bench_chunks_1920x1080_asserts(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_chunks_asserts(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }

    #[bench]
    fn bench_chunks_1920x1080_asserts_2(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_chunks_asserts_2(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }

    #[bench]
    fn bench_chunks_1920x1080_iter_sum(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_chunks_iter_sum(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }

    #[bench]
    fn bench_chunks_1920x1080_iter_sum_2(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_chunks_iter_sum_2(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }

    #[bench]
    fn bench_exact_chunks_1920x1080(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_exact_chunks(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }
}
