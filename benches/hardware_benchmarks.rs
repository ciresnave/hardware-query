use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use hardware_query::*;

fn cpu_benchmark(c: &mut Criterion) {
    c.bench_function("cpu_query", |b| {
        b.iter(|| {
            let cpu_info = black_box(CPUInfo::query().unwrap());
            black_box(cpu_info)
        })
    });
}

fn memory_benchmark(c: &mut Criterion) {
    c.bench_function("memory_query", |b| {
        b.iter(|| {
            let memory_info = black_box(MemoryInfo::query().unwrap());
            black_box(memory_info)
        })
    });
}

fn gpu_benchmark(c: &mut Criterion) {
    c.bench_function("gpu_query", |b| {
        b.iter(|| {
            let gpu_info = black_box(GPUInfo::query_all().unwrap());
            black_box(gpu_info)
        })
    });
}

fn hardware_info_benchmark(c: &mut Criterion) {
    c.bench_function("hardware_info_query", |b| {
        b.iter(|| {
            let hw_info = black_box(HardwareInfo::query().unwrap());
            black_box(hw_info)
        })
    });
}

fn serialization_benchmark(c: &mut Criterion) {
    let hw_info = HardwareInfo::query().unwrap();

    c.bench_function("serialization", |b| {
        b.iter(|| {
            let json = black_box(hw_info.to_json().unwrap());
            black_box(json)
        })
    });

    let json = hw_info.to_json().unwrap();
    c.bench_function("deserialization", |b| {
        b.iter(|| {
            let deserialized = black_box(HardwareInfo::from_json(&json).unwrap());
            black_box(deserialized)
        })
    });
}

criterion_group!(
    benches,
    cpu_benchmark,
    memory_benchmark,
    gpu_benchmark,
    hardware_info_benchmark,
    serialization_benchmark
);
criterion_main!(benches);
