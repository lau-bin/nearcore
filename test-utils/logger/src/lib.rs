mod tracing_capture;

use tracing_subscriber::EnvFilter;

pub use tracing_capture::TracingCapture;

fn setup_subscriber_from_filter(mut env_filter: EnvFilter) {
    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        for directive in rust_log.split(',').filter_map(|s| match s.parse() {
            Ok(directive) => Some(directive),
            Err(err) => {
                eprintln!("Ignoring directive `{}`: {}", s, err);
                None
            }
        }) {
            env_filter = env_filter.add_directive(directive);
        }
    }

    let _ = tracing_subscriber::fmt::Subscriber::builder()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .try_init();
}

pub fn init_test_logger() {
    let env_filter = EnvFilter::new("tokio_reactor=info,tokio_core=info,hyper=info,debug");
    setup_subscriber_from_filter(env_filter);
}

pub fn init_test_logger_allow_panic() {
    let env_filter = EnvFilter::new("tokio_reactor=info,tokio_core=info,hyper=info,debug");
    setup_subscriber_from_filter(env_filter);
}

pub fn init_test_module_logger(module: &str) {
    let env_filter =
        EnvFilter::new("tokio_reactor=info,tokio_core=info,hyper=info,cranelift_wasm=warn,info")
            .add_directive(format!("{}=info", module).parse().unwrap());
    setup_subscriber_from_filter(env_filter);
}

pub fn init_integration_logger() {
    let env_filter = EnvFilter::new("actix_web=warn,info");
    setup_subscriber_from_filter(env_filter);
}
