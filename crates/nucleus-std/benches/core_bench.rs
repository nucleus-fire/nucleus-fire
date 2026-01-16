use criterion::{black_box, criterion_group, criterion_main, Criterion, Bencher};
use nucleus_std::neutron::{Signal, computed};

fn neutron_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("neutron");

    group.bench_function("signal_creation", |b: &mut Bencher| {
        b.iter(|| {
            Signal::new(black_box(10))
        })
    });

    group.bench_function("signal_update", |b: &mut Bencher| {
        let signal = Signal::new(0);
        b.iter(|| {
            signal.set(black_box(1));
        })
    });

    group.bench_function("computed_update", |b: &mut Bencher| {
        let signal = Signal::new(0);
        let _comp = computed(signal.clone(), |v| v * 2);
        
        b.iter(|| {
            signal.set(black_box(1));
        })
    });

    group.finish();
}

fn polyglot_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("polyglot");
    let mut p = nucleus_std::polyglot::Polyglot::new("en");
    p.add("greeting", "Hello, {{name}}! Welcome to {{place}}.");

    group.bench_function("interpolation", |b: &mut Bencher| {
        b.iter(|| {
            p.t_with("greeting", &[("name", "User"), ("place", "Nucleus")])
        })
    });
    group.finish();
}

fn serialization_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Data {
        id: String,
        count: i32,
        values: Vec<f64>,
    }
    
    let data = Data {
        id: "test-id".to_string(),
        count: 42,
        values: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    };
    
    group.bench_function("json_serialize", |b: &mut Bencher| {
        b.iter(|| {
            serde_json::to_string(black_box(&data)).unwrap()
        })
    });

    let json = serde_json::to_string(&data).unwrap();
    group.bench_function("json_deserialize", |b: &mut Bencher| {
        b.iter(|| {
             serde_json::from_str::<Data>(black_box(&json)).unwrap()
        })
    });
    group.finish();
}

// Router benchmark requires async runtime
fn router_benchmark(c: &mut Criterion) {
    use nucleus_std::axum::{Router, routing::get, body::Body, http::Request};
    use tower::ServiceExt; // for oneshot
    
    let mut group = c.benchmark_group("router");
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    group.bench_function("route_match", |b: &mut Bencher| {
        b.to_async(&rt).iter(|| async {
            let app = Router::new().route("/", get(|| async { "Hello" }));
            let req = Request::builder().uri("/").body(Body::empty()).unwrap();
            let _ = app.oneshot(req).await;
        })
    });
    group.finish();
}

criterion_group!(benches, neutron_benchmark, polyglot_benchmark, serialization_benchmark, router_benchmark);
criterion_main!(benches);
