#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::path::Path;

pub mod common;

fn character(c: &mut Criterion) {
    c.bench_function("character module", |b| {
        b.iter(|| common::render_module("character").output())
    });
}

fn directory(c: &mut Criterion) {
    c.bench_function("directory module – home dir", |b| {
        b.iter(|| common::render_module("directory").arg("--path=~").output())
    });
}

fn nodejs(c: &mut Criterion) {
    c.bench_function("node module – node project", |b| {
        let node_project_path = Path::new("tests").join("fixtures/nodejs_project");
        let node_project = &node_project_path.to_str().unwrap();

        b.iter(|| {
            common::render_module("directory")
                .arg("--path")
                .arg(node_project)
                .output()
        })
    });
}

fn full_prompt(c: &mut Criterion) {
    c.bench_function("full prompt", |b| {
        b.iter(|| common::render_prompt().output())
    });
}

fn config() -> Criterion {
    let criterion: Criterion = Default::default();
    // Default of 100 samples takes > 5 mins to benchmark
    criterion.sample_size(20)
}

criterion_group! {
    name = benches;
    config = config();
    targets = character, directory, nodejs, full_prompt
}
criterion_main!(benches);
