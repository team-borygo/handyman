use crate::{bookmark::Bookmark, environment::Environment};
use regex::Regex;
use reqwest::header::{self, ACCEPT, USER_AGENT};
use url::Url;

use super::Interpreter;

pub struct UrlInterpreter {}

impl UrlInterpreter {
  fn get_name(&self) -> String {
    "url".to_string()
  }
}

impl Interpreter for UrlInterpreter {
  fn check(&self, input: &str) -> bool {
    let url = Url::parse(input);

    match url {
      Ok(url) => url.scheme() == "https" || url.scheme() == "http",
      Err(_) => false,
    }
  }

  fn interpet(&self, _environment: &Environment, input: &str) -> crate::bookmark::Bookmark {
    let mut headers = header::HeaderMap::new();
    headers.insert(ACCEPT, header::HeaderValue::from_static("text/html"));
    headers.insert(
      USER_AGENT,
      header::HeaderValue::from_static(
        "handyman - url interpreter (https://github.com/soanvig/handyman)",
      ),
    );

    let client = reqwest::blocking::Client::builder()
      .default_headers(headers)
      .build()
      .unwrap();

    let page_content = client
      .get(input)
      .send()
      .and_then(|res| res.text())
      .expect("HTTP Request failed");

    let regexp = Regex::new(r"<title>(.+)</title>").unwrap();

    let title = regexp
      .captures(&page_content)
      .and_then(|group| group.get(1))
      .expect("Page has no title")
      .as_str();

    Bookmark::new(title.to_string(), input.to_string(), self.get_name())
  }

  fn belongs(&self, interpreted_by: &str) -> bool {
    interpreted_by == self.get_name()
  }
}
