#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use axum::extract::Request;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use Htrace::HTrace;
use http::header::*;

mod api;


#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use std::fs;
	use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
	use Hconfig::HConfigManager::HConfigManager;
	use hyultisfr::entry::{shell, App};
	use Htrace::CommandLine::{CommandLine, CommandLineConfig};
	use Htrace::HTracer::HTracer;
	use Htrace::Type::Type;

	let mut conf = get_configuration(None).unwrap();

	let _ = fs::create_dir("./config");
	let _ = fs::create_dir("./dynamic");
	let _ = fs::remove_dir_all("./dynamic/traces");

	HConfigManager::singleton().setConfPath("./config");
	if(conf.leptos_options.env==Env::PROD)
	{
		HTracer::minlvl_default(Type::NOTICE);
	}
	else
	{
		HTracer::minlvl_default(Type::DEBUG);
	}
	HTracer::appendModule("cli", CommandLine::new(CommandLineConfig::default())).unwrap();
	HTracer::appendModule("file", Htrace::File::File::new(Htrace::File::FileConfig {
		path: "./dynamic/traces".to_string(),
		bySrc: true,
		byThreadId: false,
		..Htrace::File::FileConfig::default()
	})).unwrap();

	//conf.leptos_options.site_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 3000);
    let addr = conf.leptos_options.site_addr;

	// redefining ENV options from ENV if existing
	if let Ok(env) = std::env::var("ENV")
	{
		if(env=="PROD")
		{
			conf.leptos_options.env = Env::PROD
		}
	}
	HTrace!((Type::DEBUG) "leptos option env : {:?}",conf.leptos_options.env);

    let leptos_options = conf.leptos_options.clone();

    let app = Router::new()
        .leptos_routes(&leptos_options, generate_route_list(App), {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
	    .layer(middleware::from_fn(tracing_request))
	    .layer(middleware::from_fn(http_good_practice))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn tracing_request(
	request: Request,
	next: Next,
) -> Response {
	let method = request.method().to_string();
	let uri = request.uri().to_string();

	let response = next.run(request).await;

	HTrace!("Request {} on {} : {}", method, uri, response.status());

	response
}


async fn http_good_practice(
	request: Request,
	next: Next,
) -> Response {
	let mut response = next.run(request).await;

	response.headers_mut().insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
	response.headers_mut().insert(CONTENT_SECURITY_POLICY, HeaderValue::from_static("frame-ancestors 'none'"));
	response.headers_mut().insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
	response.headers_mut().insert(STRICT_TRANSPORT_SECURITY, HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"));
	response.headers_mut().insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));

	response
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
