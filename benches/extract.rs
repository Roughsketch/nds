#[macro_use] extern crate criterion;
extern crate nds;

use criterion::Criterion;

fn bench_extract_big(c: &mut Criterion) {
    use nds::Extractor;

    let ex = Extractor::new("big.nds").unwrap();

    c.bench_function("extract big", move |b| b.iter(|| {
        ex.extract("tmp/big").unwrap();
    }));
}

fn bench_extract_small(c: &mut Criterion) {
    use nds::Extractor;

    let ex = Extractor::new("small.nds").unwrap();

    c.bench_function("extract small", move |b| b.iter(|| {
        ex.extract("tmp/small").unwrap();
    }));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(25);
    targets = bench_extract_big, bench_extract_small
}

criterion_main!(benches);