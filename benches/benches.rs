//! Demosaicing algorithm benchmarks.

#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use bayer::*;
    use std::io::Cursor;

    const IMG_W: usize = 128;
    const IMG_H: usize = 128;
    const SRC_U8: [u8; IMG_W * IMG_H] = [0u8; IMG_W * IMG_H];
    const SRC_U16: [u8; 2 * IMG_W * IMG_H] = [0u8; 2 * IMG_W * IMG_H];
    const BUF_SIZE_U8: usize = 3 * IMG_W * IMG_H;
    const BUF_SIZE_U16: usize = 2 * BUF_SIZE_U8;

    #[bench]
    fn bench_none_u8(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U8];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth8, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U8[..]),
                BayerDepth::Depth8,
                CFA::RGGB,
                Demosaic::None,
                &mut dst,
            )
        });
    }

    #[bench]
    fn bench_none_u16(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U16];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth16, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U16[..]),
                BayerDepth::Depth16LE,
                CFA::RGGB,
                Demosaic::None,
                &mut dst,
            )
        });
    }

    #[bench]
    fn bench_nearest_neighbour_u8(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U8];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth8, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U8[..]),
                BayerDepth::Depth8,
                CFA::RGGB,
                Demosaic::NearestNeighbour,
                &mut dst,
            )
        });
    }

    #[bench]
    fn bench_nearest_neighbour_u16(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U16];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth16, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U16[..]),
                BayerDepth::Depth16LE,
                CFA::RGGB,
                Demosaic::NearestNeighbour,
                &mut dst,
            )
        });
    }

    #[bench]
    fn bench_linear_u8(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U8];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth8, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U8[..]),
                BayerDepth::Depth8,
                CFA::RGGB,
                Demosaic::Linear,
                &mut dst,
            )
        });
    }

    #[bench]
    fn bench_linear_u16(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U16];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth16, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U16[..]),
                BayerDepth::Depth16LE,
                CFA::RGGB,
                Demosaic::Linear,
                &mut dst,
            )
        });
    }

    #[bench]
    fn bench_cubic_u8(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U8];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth8, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U8[..]),
                BayerDepth::Depth8,
                CFA::RGGB,
                Demosaic::Cubic,
                &mut dst,
            )
        });
    }

    #[bench]
    fn bench_cubic_u16(b: &mut test::Bencher) {
        let mut buf = [0; BUF_SIZE_U16];
        let mut dst = RasterMut::new(IMG_W, IMG_H, RasterDepth::Depth16, &mut buf);
        b.iter(|| {
            demosaic(
                &mut Cursor::new(&SRC_U16[..]),
                BayerDepth::Depth16LE,
                CFA::RGGB,
                Demosaic::Cubic,
                &mut dst,
            )
        });
    }
}
