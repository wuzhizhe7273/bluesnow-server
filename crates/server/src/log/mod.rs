use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, HttpMakeClassifier, TraceLayer};
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub struct Log{
    level:Level
}

#[derive(Debug,Clone)]
pub struct LogConfig{
    pub level:String
}
fn get_level(level:&str)->Level{
    match level {
        "debug"=>Level::DEBUG,
        "info"=>Level::INFO,
        "error"=>Level::ERROR,
        "trace"=>Level::TRACE,
        "warn"=>Level::WARN,
        _=>panic!("unsupported log level:{}",level)
    }
}
impl Log{
    pub fn new(config:&LogConfig)->Self{
        let level=get_level(&config.level);
        Self{
            level
        }
    }
    pub fn init(self)->Self{
        let fmt_layer=fmt::layer()
            .with_target(false);
        let filter_layer=EnvFilter::builder()
            .with_default_directive(LevelFilter::from_level(self.level.clone()).into())
            .from_env_lossy();
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
        self

    }
    fn get_http_layer(&self)->TraceLayer<HttpMakeClassifier>{
        TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new().include_headers(true)
            )
            .on_request(
                DefaultOnRequest::new()
            )
            .on_response(
                DefaultOnResponse::new()
            )
    }
    pub fn set_router(&self,router:Router)->Router{
        let log_layer=self.get_http_layer();
        let router=router.layer(log_layer);
        router
    }
}