#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::path::Path;

pub mod common;

fn fixture_dir(fixture_name: &str) -> String {
    let path = Path::new("benches").join("fixtures").join(fixture_name);
    path.to_string_lossy().to_string()
}

fn character(c: &mut Criterion) {
    c.bench_function("character module", |b| {
        b.iter(|| common::render_module("character").output())
    });
}

fn line_break(c: &mut Criterion) {
    c.bench_function("line break module", |b| {
        b.iter(|| common::render_module("line_break").output())
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
            common::render_module("nodejs")
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
            common::render_module("golang")
                .arg("--path")
                .arg(&golang_project_dir)
                .output()
        })
    });
}

fn rust(c: &mut Criterion) {
    let rust_project_dir = fixture_dir("rust_project");
    c.bench_function("rust module", move |b| {
        b.iter(|| {
            common::render_module("rust")
                .arg("--path")
                .arg(&rust_project_dir)
                .output()
        })
    });
}

fn python(c: &mut Criterion) {
    let python_project_dir = fixture_dir("python_project");
    c.bench_function("python module", move |b| {
        b.iter(|| {
            common::render_module("python")
                .arg("--path")
                .arg(&python_project_dir)
                .output()
        })
    });
}

fn package(c: &mut Criterion) {
    let node_project_dir = fixture_dir("nodejs_project");
    let rust_project_dir = fixture_dir("rust_project");
    
    c.bench_function("package module – node project", move |b| {
        b.iter(|| {
            common::render_module("package")
                .arg("--path")
                .arg(&node_project_dir)
                .output()
        })
    });

    c.bench_function("package module – rust project", move |b| {
        b.iter(|| {
            common::render_module("package")
                .arg("--path")
                .arg(&rust_project_dir)
                .output()
        })
    });
}

fn username(c: &mut Criterion) {    
    c.bench_function("username module", move |b| {
        b.iter(|| {
            common::render_module("username")
                .env_clear()
                .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
                .output()
        })
    });
}

// TODO: Create initial commit for modules to work
// fn git(c: &mut Criterion) {
//     let new_repo = common::new_tempdir().unwrap();
//     Repository::init(&new_repo).unwrap();

//     let repo_dir = new_repo.path().to_string_lossy().to_string();
//     c.bench_function("git_branch", move |b| {
//         b.iter(|| {
//             common::render_module("package")
//                 .arg("--path")
//                 .arg(&repo_dir)
//                 .output()
//         })
//     });

//     //
//     let repo_dir = new_repo.path().to_string_lossy().to_string();
//     c.bench_function("git_status", move |b| {
//         b.iter(|| {
//             common::render_module("package")
//                 .arg("--path")
//                 .arg(&repo_dir)
//                 .output()
//         })
//     });
// }

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
    targets = character, line_break, directory, nodejs, golang, rust, python, package, username, full_prompt
}
criterion_main!(benches);
