use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mx20022_model::generated::pacs::pacs_008_001_13::Document as Pacs008;
use mx20022_parse::de;
use mx20022_parse::envelope;
use mx20022_parse::ser;

// The testdata directory sits three levels above this crate root.
const PACS008_XML: &str = include_str!("../../../testdata/xml/pacs/pacs_008_001_13_minimal.xml");

fn bench_parse_pacs008(c: &mut Criterion) {
    c.bench_function("parse_pacs008", |b| {
        b.iter(|| {
            let _: Pacs008 =
                de::from_str(black_box(PACS008_XML)).expect("parse must not fail in benchmark");
        });
    });
}

fn bench_detect_message_type(c: &mut Criterion) {
    c.bench_function("detect_message_type", |b| {
        b.iter(|| {
            envelope::detect_message_type(black_box(PACS008_XML))
                .expect("detect must not fail in benchmark")
        });
    });
}

fn bench_serialize_pacs008(c: &mut Criterion) {
    // Parse once outside the loop so we only time serialization.
    let doc: Pacs008 = de::from_str(PACS008_XML).expect("setup parse must succeed");

    c.bench_function("serialize_pacs008", |b| {
        b.iter(|| ser::to_string(black_box(&doc)).expect("serialize must not fail in benchmark"));
    });
}

criterion_group!(
    parse_benches,
    bench_parse_pacs008,
    bench_detect_message_type,
    bench_serialize_pacs008
);
criterion_main!(parse_benches);
