#[macro_use] extern crate criterion;
extern crate nds;

use criterion::Criterion;

fn bench_create_extractor_big(c: &mut Criterion) {
    use nds::Extractor;

    c.bench_function("create extract big", move |b| b.iter(|| {
        let ex = Extractor::new("big.nds", true).unwrap();
    }));
}

fn bench_create_extractor_small(c: &mut Criterion) {
    use nds::Extractor;

    c.bench_function("create extract small", move |b| b.iter(|| {
        let ex = Extractor::new("small.nds", true).unwrap();
    }));
}

fn bench_extract_big(c: &mut Criterion) {
    use nds::Extractor;

    let ex = Extractor::new("big.nds", true).unwrap();

    c.bench_function("extract big", move |b| b.iter(|| {
        ex.extract("tmp/big").unwrap();
    }));
}

fn bench_extract_small(c: &mut Criterion) {
    use nds::Extractor;

    let ex = Extractor::new("small.nds", true).unwrap();

    c.bench_function("extract small", move |b| b.iter(|| {
        ex.extract("tmp/small").unwrap();
    }));
}

fn bench_crc16(c: &mut Criterion) {
    use nds::util::crc::crc16;

    let data = [0; 0x4000];

    c.bench_function("crc16 0x4000 bytes", move |b| b.iter(|| {
        crc16(&data)
    }));
}

criterion_group! {
    name = extractor;
    config = Criterion::default().sample_size(100);
    targets = bench_create_extractor_big, bench_create_extractor_small
}

criterion_group! {
    name = extract;
    config = Criterion::default().sample_size(25);
    targets = bench_extract_big, bench_extract_small
}

criterion_group! {
    name = util;
    config = Criterion::default().sample_size(100);
    targets = bench_crc16
}

criterion_main!(extractor, extract, util);
