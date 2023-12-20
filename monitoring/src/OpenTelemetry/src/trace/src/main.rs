use tracing_subscriber::util::SubscriberInitExt;

#[tracing::instrument]
fn puyo() {
    tracing::debug!("puyo?")
}

#[tracing::instrument]
fn piyo() {
    tracing::debug!("piyo!");
    puyo()
}

fn init() {
    let propagator = opentelemetry_jaeger::Propagator::new();
    opentelemetry::global::set_text_map_propagator(propagator);

    use tracing_subscriber::layer::SubscriberExt;
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("piyopoyo")
        .with_endpoint("server:6831")
        .install_simple()
        .unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    use tracing_subscriber::EnvFilter;
    let layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into());
    tracing_subscriber::registry()
        .with(layer)
        .with(telemetry)
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::debug!("Initialized.");

    piyo();
    // =>
    // {"timestamp":"2023-04-01T00:00:00.000000Z","level":"DEBUG","fields":{"message":"piyo!"},"target":"trace","span":{"name":"piyo"},"spans":[{"name":"piyo"}]}
    // {"timestamp":"2023-04-01T00:00:00.000000Z","level":"DEBUG","fields":{"message":"puyo?"},"target":"trace","span":{"name":"puyo"},"spans":[{"name":"piyo"},{"name":"puyo"}]}
}

fn main() {
    init();
    piyo();
    // =>
    // {"timestamp":"2023-04-01T00:00:00.000000Z","level":"DEBUG","fields":{"message":"piyo!"},"target":"trace","span":{"name":"piyo"},"spans":[{"name":"piyo"}]}
    // {"timestamp":"2023-04-01T00:00:00.000000Z","level":"DEBUG","fields":{"message":"puyo?"},"target":"trace","span":{"name":"puyo"},"spans":[{"name":"piyo"},{"name":"puyo"}]}
}
