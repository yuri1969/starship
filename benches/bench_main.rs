#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::path::Path;

pub mod common;

fn fixture_dir(fixture_name: &str) -> String {
    let path = Path::new("tests").join("fixtures").join(fixture_name);
    path.to_string_lossy().to_string()
}

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
    let node_project_dir = fixture_dir("nodejs_project");

    c.bench_function("nodejs module", move |b| {
        b.iter(|| {
            common::render_module("directory")
                .arg("--path")
                .arg(&node_project_dir)
                .output()
        })
    });
}

fn golang(c: &mut Criterion) {
    let golang_project_dir = fixture_dir("golang_projectt");
    
    c.bench_function("golang module", move |b| {
        b.iter(|| {
            common::render_module("directory")
                .arg("--path")
                .arg(&golang_project_dir)
                .output()
        })
    });
}

fn rust(c: &mut Criterion) {
    let rust_project_dir = fixture_dir("rust_project");
    
    c.bench_function("golang module", move |b| {
        b.iter(|| {
            common::render_module("directory")
                .arg("--path")
                .arg(&rust_project_dir)
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
    targets = character, directory, nodejs, golang, full_prompt
}
criterion_main!(benches);
