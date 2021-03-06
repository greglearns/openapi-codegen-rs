#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Pet {
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    r#tag: Option<String>,
    #[serde(rename = "id")]
    r#id: i64,
    #[serde(rename = "name")]
    r#name: String,
}

impl r#Pet {
    pub fn new(
        r#id: i64,
        r#name: String,
    ) -> Self {
        Self {
          r#tag: None,
          r#id: id,
          r#name: name,
        }
    }

    pub fn set_tag(&mut self, r#tag: String) {
        self.r#tag = Some(r#tag);
    }

    pub fn with_tag(mut self, r#tag: String) -> Self {
        self.r#tag = Some(r#tag);
        self
    }

    pub fn with_option_tag(mut self, r#tag: Option<String>) -> Self {
        self.r#tag = r#tag;
        self
    }

    pub fn r#tag(&self) -> Option<&str> {
        self.r#tag.as_ref().map(|x| x.borrow())
    }

    pub fn reset_tag(&mut self) {
        self.r#tag = None;
    }

    pub fn set_id(&mut self, r#id: i64) {
        self.r#id = r#id;
    }

    pub fn with_id(mut self, r#id: i64) -> Self {
        self.r#id = r#id;
        self
    }

    pub fn r#id(&self) -> &i64 {
        self.r#id.borrow()
    }

    pub fn set_name(&mut self, r#name: String) {
        self.r#name = r#name;
    }

    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        self
    }

    pub fn r#name(&self) -> &str {
        self.r#name.borrow()
    }
}
