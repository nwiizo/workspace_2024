use metrics::{describe_counter, describe_histogram, register_counter, register_histogram};
use metrics_exporter_prometheus::PrometheusBuilder;

pub fn setup_metrics() {
    // Counter metrics
    describe_counter!(
        "yubin_api_postal_lookups_total",
        "Total number of postal code lookups"
    );
    describe_counter!(
        "yubin_api_address_searches_total",
        "Total number of address searches"
    );

    // Histogram metrics
    describe_histogram!(
        "yubin_api_postal_lookup_duration_seconds",
        "Duration of postal code lookups in seconds"
    );
    describe_histogram!(
        "yubin_api_address_search_duration_seconds",
        "Duration of address searches in seconds"
    );

    register_counter!("yubin_api_postal_lookups_total");
    register_counter!("yubin_api_address_searches_total");
    register_histogram!("yubin_api_postal_lookup_duration_seconds");
    register_histogram!("yubin_api_address_search_duration_seconds");

    // Install global recorder
    PrometheusBuilder::new()
        .install()
        .expect("Failed to install Prometheus recorder");
}
