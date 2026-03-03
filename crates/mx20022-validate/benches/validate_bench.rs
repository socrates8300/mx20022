use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mx20022_validate::rules::RuleRegistry;
use mx20022_validate::schemes::{fednow::FedNowValidator, sepa::SepaValidator, SchemeValidator};

const FEDNOW_VALID_XML: &str = include_str!("../../../testdata/schemes/fednow/valid_pacs008.xml");
const SEPA_VALID_XML: &str = include_str!("../../../testdata/schemes/sepa/valid_pacs008.xml");

fn bench_iban_validation(c: &mut Criterion) {
    let registry = RuleRegistry::with_defaults();

    c.bench_function("validate_iban", |b| {
        b.iter(|| {
            registry.validate_field(
                black_box("GB82WEST12345698765432"),
                "/Document/DbtrAcct/Id/IBAN",
                &["IBAN_CHECK"],
            )
        });
    });
}

fn bench_bic_validation(c: &mut Criterion) {
    let registry = RuleRegistry::with_defaults();

    c.bench_function("validate_bic", |b| {
        b.iter(|| {
            registry.validate_field(
                black_box("AAAAGB2LXXX"),
                "/Document/DbtrAgt/FinInstnId/BICFI",
                &["BIC_CHECK"],
            )
        });
    });
}

fn bench_fednow_validation(c: &mut Criterion) {
    let validator = FedNowValidator::new();

    c.bench_function("fednow_validate_pacs008", |b| {
        b.iter(|| validator.validate(black_box(FEDNOW_VALID_XML), "pacs.008.001.13"));
    });
}

fn bench_sepa_validation(c: &mut Criterion) {
    let validator = SepaValidator::new();

    c.bench_function("sepa_validate_pacs008", |b| {
        b.iter(|| validator.validate(black_box(SEPA_VALID_XML), "pacs.008.001.13"));
    });
}

fn bench_rule_registry_construction(c: &mut Criterion) {
    c.bench_function("rule_registry_with_defaults", |b| {
        b.iter(RuleRegistry::with_defaults);
    });
}

criterion_group!(
    validate_benches,
    bench_iban_validation,
    bench_bic_validation,
    bench_fednow_validation,
    bench_sepa_validation,
    bench_rule_registry_construction,
);
criterion_main!(validate_benches);
