use criterion::{criterion_group, criterion_main, Criterion};
use tracking_params::{clean, clean_str};
use url::Url;

fn criterion_benchmark(c: &mut Criterion) {
    let input = "https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5Etfw&from=home#ref_src=twsrc%5Etfw&from=home";
    let url = Url::parse(input).unwrap();

    c.bench_function("clean", |b| b.iter(|| clean(url.clone())));
    c.bench_function("clean_str", |b| b.iter(|| clean_str(input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
