use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mx20022_translate::mappings::mt103_to_pacs008::mt103_to_pacs008;
use mx20022_translate::mappings::pacs008_to_mt103::pacs008_to_mt103;
use mx20022_translate::mt;
use mx20022_translate::mt::fields::mt103::parse_mt103;

const MT103_TEXT: &str = include_str!("../../../testdata/mt/mt103.txt");

fn bench_mt_parse(c: &mut Criterion) {
    c.bench_function("mt103_block_parse", |b| {
        b.iter(|| mt::parse(black_box(MT103_TEXT)).expect("parse must not fail"));
    });
}

fn bench_mt103_fields_parse(c: &mut Criterion) {
    // Parse the block structure once outside the loop.
    let msg = mt::parse(MT103_TEXT).expect("setup parse must succeed");

    c.bench_function("mt103_fields_parse", |b| {
        b.iter(|| parse_mt103(black_box(&msg.block4)).expect("field parse must not fail"));
    });
}

fn bench_mt103_to_pacs008(c: &mut Criterion) {
    let msg = mt::parse(MT103_TEXT).expect("setup parse must succeed");
    let mt103 = parse_mt103(&msg.block4).expect("setup field parse must succeed");

    c.bench_function("mt103_to_pacs008", |b| {
        b.iter(|| {
            mt103_to_pacs008(black_box(&mt103), "BENCH-001", "2023-06-15T12:00:00")
                .expect("translation must not fail")
        });
    });
}

fn bench_pacs008_to_mt103(c: &mut Criterion) {
    let msg = mt::parse(MT103_TEXT).expect("setup parse must succeed");
    let mt103 = parse_mt103(&msg.block4).expect("setup field parse must succeed");
    let result = mt103_to_pacs008(&mt103, "BENCH-001", "2023-06-15T12:00:00")
        .expect("setup translation must succeed");
    let doc = result.message;

    c.bench_function("pacs008_to_mt103", |b| {
        b.iter(|| pacs008_to_mt103(black_box(&doc)).expect("reverse translation must not fail"));
    });
}

fn bench_full_roundtrip(c: &mut Criterion) {
    c.bench_function("mt103_pacs008_roundtrip", |b| {
        b.iter(|| {
            let msg = mt::parse(black_box(MT103_TEXT)).expect("parse must not fail");
            let mt103 = parse_mt103(&msg.block4).expect("field parse must not fail");
            let fwd = mt103_to_pacs008(&mt103, "RT-001", "2023-06-15T12:00:00")
                .expect("forward translation must not fail");
            pacs008_to_mt103(&fwd.message).expect("reverse translation must not fail")
        });
    });
}

criterion_group!(
    translate_benches,
    bench_mt_parse,
    bench_mt103_fields_parse,
    bench_mt103_to_pacs008,
    bench_pacs008_to_mt103,
    bench_full_roundtrip,
);
criterion_main!(translate_benches);
