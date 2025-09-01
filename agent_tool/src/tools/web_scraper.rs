use super::Tool;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::{json, Value};

pub struct WebSearchTool;

#[derive(Deserialize)]
struct WebSearchArgs {
    query: String,
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &'static str { "web_search" }
    fn description(&self) -> &'static str { "Realiza una búsqueda web y extrae contenido de texto." }

    async fn execute(&self, args_json: &str) -> Result<Value> {
        let args: WebSearchArgs = serde_json::from_str(args_json)?;
        
        let url = format!("https://html.duckduckgo.com/html/?q={}", args.query);
        let response_html = reqwest::get(&url).await?.text().await?;
        
        let document = Html::parse_document(&response_html);
        let selector = Selector::parse("a.result__a").unwrap();
        
        let mut results = vec![];
        for element in document.select(&selector).take(5) {
            let title = element.text().collect::<String>().trim().to_string();
            if let Some(link) = element.value().attr("href") {
                 results.push(json!({"title": title, "link": link}));
            }
        }
        
        Ok(json!({ "results": results }))
    }
}