use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use symmetric_word_triples::auto_dir_sym_word_sol;

pub fn benchmark_dict_big(c: &mut Criterion) {
    let text_dir = Path::new("../data");
    let grid_range = (3, 3);
    let chunk_size_range = (3, 3);
    let input_dir = text_dir.join("input");
    let output_dir = text_dir.join("output");

    c.bench_function("dir symmetry bench", move |b| {
        b.iter(|| {
            auto_dir_sym_word_sol(&input_dir, &output_dir, grid_range, chunk_size_range).unwrap()
        })
    });
}
criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(30);
    targets = benchmark_dict_big
}
criterion_main!(benches);
