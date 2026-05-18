use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

pub fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([
      Method::GET,
      Method::POST,
      Method::PUT,
      Method::PATCH,
      Method::DELETE,
      Method::OPTIONS,
    ])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    .allow_credentials(true)
}

// pub fn cors_layer(env: &str) -> CorsLayer {
//   match env {
//     // 🔧 Local development
//     "development" => CorsLayer::new()
//       .allow_origin(Any)
//       .allow_methods([
//         Method::GET,
//         Method::POST,
//         Method::PUT,
//         Method::PATCH,
//         Method::DELETE,
//         Method::OPTIONS,
//       ])
//       .allow_headers(Any),

//     // 🧪 Staging
//     "staging" => CorsLayer::new()
//       .allow_origin(
//         "https://staging.yourapp.com"
//           .parse::<HeaderValue>()
//           .unwrap(),
//       )
//       .allow_methods([
//         Method::GET,
//         Method::POST,
//         Method::PUT,
//         Method::PATCH,
//         Method::DELETE,
//       ])
//       .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
//       .allow_credentials(true),

//     // 🚀 Production
//     _ =>
//   }
// }
