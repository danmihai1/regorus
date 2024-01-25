// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Engine {
    engine: crate::Engine,
}

fn error_to_jsvalue<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&format!("{e}"))
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: crate::Engine::new(),
        }
    }

    pub fn clone_engine(&self) -> Self {
        Self {
            engine: self.engine.clone(),
        }
    }

    pub fn add_policy(&mut self, path: String, rego: String) -> Result<(), JsValue> {
        self.engine.add_policy(path, rego).map_err(error_to_jsvalue)
    }

    pub fn add_data(&mut self, data: String) -> Result<(), JsValue> {
        let data = crate::Value::from_json_str(&data).map_err(error_to_jsvalue)?;
        self.engine.add_data(data).map_err(error_to_jsvalue)
    }

    pub fn set_input(&mut self, input: String) -> Result<(), JsValue> {
        let input = crate::Value::from_json_str(&input).map_err(error_to_jsvalue)?;
        self.engine.set_input(input);
        Ok(())
    }

    pub fn eval_query(&mut self, query: String) -> Result<String, JsValue> {
        let results = self
            .engine
            .eval_query(query, false)
            .map_err(error_to_jsvalue)?;
        serde_json::to_string_pretty(&results).map_err(error_to_jsvalue)
    }
}
