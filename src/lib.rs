pub mod metadata;
pub mod api;
pub mod utils;
pub mod server;
mod config;
// async fn task(fut: impl std::future::Future<Output = ()> + Send + 'static){
//     match tokio::spawn(fut).await{
//         Ok(_) => {}
//         Err(e) => {
//             error!("Error in task: {}", e);
//         }
//     }
//
// }
//
//
// // #[tokio::main]
// // async fn main() {
// //     let exporter = opentelemetry_stdout::LogExporter::default();
// //     let logger_provider = LoggerProvider::builder()
// //         .with_resource(Resource::new([KeyValue::new(
// //             SERVICE_NAME,
// //             "jzfs-logs-collect",
// //         )]))
// //         .with_simple_exporter(exporter)
// //         .build();
// //     let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
// //     log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
// //     log::set_max_level(Level::Info.to_level_filter());
// // }
