use anyhow::Context;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmTask {
    SummarizeJd,
    RewriteMessage,
    RewriteBullet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub task: LlmTask,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub output: String,
    pub provider: String,
}

pub trait LlmAdapter {
    fn rewrite(&self, req: &LlmRequest) -> anyhow::Result<LlmResponse>;
}

#[derive(Debug, Clone)]
pub struct OllamaAdapter {
    pub base_url: String,
    pub model: String,
}

impl LlmAdapter for OllamaAdapter {
    fn rewrite(&self, req: &LlmRequest) -> anyhow::Result<LlmResponse> {
        #[derive(Serialize)]
        struct Body<'a> {
            model: &'a str,
            prompt: &'a str,
            stream: bool,
        }
        #[derive(Deserialize)]
        struct Resp {
            response: String,
        }

        let client = Client::new();
        let url = format!("{}/api/generate", self.base_url.trim_end_matches('/'));
        let resp = client
            .post(&url)
            .json(&Body { model: &self.model, prompt: &req.prompt, stream: false })
            .send()
            .with_context(|| format!("request to {url}"))?
            .error_for_status()
            .context("ollama non-2xx")?
            .json::<Resp>()
            .context("ollama response parse")?;

        Ok(LlmResponse { output: resp.response, provider: "ollama".to_string() })
    }
}

#[derive(Debug, Clone)]
pub struct OpenAiCompatAdapter {
    pub provider_name: String,
    pub base_url: String,
    pub model: String,
}

impl LlmAdapter for OpenAiCompatAdapter {
    fn rewrite(&self, req: &LlmRequest) -> anyhow::Result<LlmResponse> {
        #[derive(Serialize)]
        struct ChatBody<'a> {
            model: &'a str,
            messages: Vec<Message<'a>>,
            temperature: f32,
        }
        #[derive(Serialize)]
        struct Message<'a> {
            role: &'a str,
            content: &'a str,
        }

        #[derive(Deserialize)]
        struct ChatResp {
            choices: Vec<Choice>,
        }
        #[derive(Deserialize)]
        struct Choice {
            message: ChoiceMessage,
        }
        #[derive(Deserialize)]
        struct ChoiceMessage {
            content: String,
        }

        let client = Client::new();
        let url = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));
        let resp = client
            .post(&url)
            .json(&ChatBody {
                model: &self.model,
                messages: vec![Message { role: "user", content: &req.prompt }],
                temperature: 0.0,
            })
            .send()
            .with_context(|| format!("request to {url}"))?
            .error_for_status()
            .context("openai-compatible non-2xx")?
            .json::<ChatResp>()
            .context("openai-compatible response parse")?;

        let output = resp.choices.first().map(|c| c.message.content.clone()).unwrap_or_default();

        Ok(LlmResponse { output, provider: self.provider_name.clone() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    fn spawn_json_server(response_body: &'static str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut req_buf = [0_u8; 4096];
                let _ = stream.read(&mut req_buf);
                let body = response_body.as_bytes();
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(body);
            }
        });
        format!("http://{}", addr)
    }

    #[test]
    fn ollama_adapter_parses_response() {
        let base = spawn_json_server(r#"{ "response": "rewritten text" }"#);
        let adapter = OllamaAdapter { base_url: base, model: "llama3.2".to_string() };

        let response = adapter
            .rewrite(&LlmRequest { task: LlmTask::RewriteMessage, prompt: "hello".to_string() })
            .expect("rewrite");

        assert_eq!(response.output, "rewritten text");
        assert_eq!(response.provider, "ollama");
    }

    #[test]
    fn openai_compat_parses_response() {
        let base = spawn_json_server(
            r#"{ "choices": [ { "message": { "content": "rewritten text" } } ] }"#,
        );
        let adapter = OpenAiCompatAdapter {
            provider_name: "lm_studio".to_string(),
            base_url: base,
            model: "local".to_string(),
        };

        let response = adapter
            .rewrite(&LlmRequest { task: LlmTask::RewriteMessage, prompt: "hello".to_string() })
            .expect("rewrite");

        assert_eq!(response.output, "rewritten text");
        assert_eq!(response.provider, "lm_studio");
    }
}
