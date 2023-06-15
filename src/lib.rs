use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::info;
use serde::{ Deserialize, Serialize };

proxy_wasm::main! {
    {
        proxy_wasm::set_log_level(LogLevel::Trace);
        proxy_wasm::set_root_context(
            |_| -> Box<dyn RootContext> {
                Box::new(HttpRedirectHeaderRoot {
                    redirect_header_name: String::new(),
                })
            }
        );
    }
}

struct HttpRedirectHeader {
    redirect_header_name: String,
}

impl Context for HttpRedirectHeader {}

impl HttpContext for HttpRedirectHeader {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_headers");

        let redirect_header_value: Option<String> = self.get_http_request_header(
            &self.redirect_header_name
        );
        if redirect_header_value.is_none() {
            return Action::Continue;
        }

        let redirect_header_value = redirect_header_value.unwrap();

        info!(
            "on_http_request_headers - {} = {}",
            self.redirect_header_name,
            redirect_header_value
        );

        let request_host = self.get_http_request_header("host").unwrap_or_default();
        let request_path = self.get_http_request_header(":path").unwrap_or_default();

        info!("Host: {}", request_host);
        info!("Path: {}", request_path);

        // if redirection already occurred
        if request_path.contains(&redirect_header_value) {
            info!("Redirection already occurred");
            return Action::Continue;
        }

        let redirect_url = format!("{}{}", request_path, redirect_header_value);
        let headers = vec![("Location", redirect_url.as_str())];

        info!("redirect_url: {}", redirect_url);

        self.set_http_request_header(&self.redirect_header_name, None);
        self.send_http_response(302, headers, None);

        Action::Pause
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_body");
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        info!("on_http_response_headers");
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_response_body");
        Action::Continue
    }
}

#[derive(Serialize, Deserialize)]
struct PolicyConfig {
    #[serde(alias = "redirect_header_name")]
    redirect_header_name: String,
}

struct HttpRedirectHeaderRoot {
    redirect_header_name: String,
}

impl Context for HttpRedirectHeaderRoot {}

impl RootContext for HttpRedirectHeaderRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            let config: PolicyConfig = serde_json::from_slice(config_bytes.as_slice()).unwrap();
            self.redirect_header_name = config.redirect_header_name;
            info!("redirect_header_name = {}", self.redirect_header_name);
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(
            Box::new(HttpRedirectHeader {
                redirect_header_name: self.redirect_header_name.clone(),
            })
        )
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
