#![feature(test)]
#![feature(exact_chunks)]

extern crate test;
extern crate faster;

use faster::*;

const RGB_Y: [u32; 4] = [19595, 38470, 7471, 0];

pub fn bgrx_to_gray_chunks_no_asserts_faster_unstrided(in_data: &[u8], out_data: &mut [u8]) {
    // Relies on vector width which is a multiple of 4
    assert!(u8s::WIDTH % 4 == 0 && u32s::WIDTH % 4 == 0);

    const RGB_Y: [u32; 16] = [19595, 38470, 7471, 0, 19595, 38470, 7471, 0, 19595, 38470, 7471, 0, 19595, 38470, 7471, 0];
    let rgbvec = u32s::load(&RGB_Y, 0);
    in_data.simd_iter(u8s(0)).simd_map(|v| {
        let (a, b) = v.upcast();
        let (a32, b32) = a.upcast();
        let (c32, d32) = b.upcast();

        let grey32a = a32 * rgbvec / u32s(65536);
        let grey32b = b32 * rgbvec / u32s(65536);
        let grey32c = c32 * rgbvec / u32s(65536);
        let grey32d = d32 * rgbvec / u32s(65536);

        let grey16a = grey32a.saturating_downcast(grey32b);
        let grey16b = grey32c.saturating_downcast(grey32d);

        let grey = grey16a.saturating_downcast(grey16b);
        grey
    }).scalar_fill(out_data);
}

pub fn bgrx_to_gray_chunks_no_asserts_faster(in_data: &[u8], out_data: &mut [u8]) {
    // Sane, but slowed down by faster's current striding implementation.
    in_data.stride_four(tuplify!(4, u8s(0))).zip().simd_map(|(r, g, b, _)| {
        let (r16a, r16b) = r.upcast();
        let (r32a, r32b) = r16a.upcast();
        let (r32c, r32d) = r16b.upcast();

        let (g16a, g16b) = g.upcast();
        let (g32a, g32b) = g16a.upcast();
        let (g32c, g32d) = g16b.upcast();

        let (b16a, b16b) = b.upcast();
        let (b32a, b32b) = b16a.upcast();
        let (b32c, b32d) = b16b.upcast();

        let grey32a = (r32a * u32s(19595) + g32a * u32s(38470) + b32a * u32s(7471)) / u32s(65536);
        let grey32b = (r32b * u32s(19595) + g32b * u32s(38470) + b32b * u32s(7471)) / u32s(65536);
        let grey32c = (r32c * u32s(19595) + g32c * u32s(38470) + b32c * u32s(7471)) / u32s(65536);
        let grey32d = (r32d * u32s(19595) + g32d * u32s(38470) + b32d * u32s(7471)) / u32s(65536);

        let grey16a = grey32a.saturating_downcast(grey32b);
        let grey16b = grey32c.saturating_downcast(grey32d);

        let grey = grey16a.saturating_downcast(grey16b);
        grey
    }).scalar_fill(out_data);
}

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

pub fn bgrx_to_gray_split_at(
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
        let mut in_pp: &[u8] = in_line[..in_line_bytes].as_ref();
        let mut out_pp: &mut [u8] = out_line[..out_line_bytes].as_mut();
        assert!(in_pp.len() == out_pp.len());

        while in_pp.len() >= 4 {
            let (in_p, in_tmp) = in_pp.split_at(4);
            let (out_p, out_tmp) = { out_pp }.split_at_mut(4);
            in_pp = in_tmp;
            out_pp = out_tmp;

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
    fn bench_chunks_1920x1080_no_asserts_faster(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_chunks_no_asserts_faster(&i, &mut o));
    }

    #[bench]
    fn bench_chunks_1920x1080_no_asserts_faster_unstrided(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_chunks_no_asserts_faster_unstrided(&i, &mut o));
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
    fn bench_split_at_1920x1080(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_split_at(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }

    #[bench]
    fn bench_exact_chunks_1920x1080(b: &mut Bencher) {
        let i = test::black_box(create_vec(1920, 1080));
        let mut o = test::black_box(create_vec(1920, 1080));

        b.iter(|| bgrx_to_gray_exact_chunks(&i, &mut o, 1920 * 4, 1920 * 4, 1920));
    }

    #[test]
    fn test_scalar_vs_faster() {
        let in_both = create_vec(1920, 1080);
        let mut out_scalar = create_vec(1920, 1080);
        let mut out_faster = create_vec(1920, 1080);

        bgrx_to_gray_chunks_no_asserts(&in_both, &mut out_scalar, 1920 * 4, 1920 * 4, 1920);
        bgrx_to_gray_chunks_no_asserts_faster(&in_both, &mut out_faster);

        for i in 0..out_scalar.len() {
            assert_eq!(out_faster[i], out_scalar[i]);
        }
    }
}
