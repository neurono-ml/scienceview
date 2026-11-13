//! The figure JavaScript constructs: a canvas handle and the values it holds.
//!
//! JavaScript creates a figure from a canvas id and values, pushes new data,
//! and asks for a render. The model stays in Rust: the figure forwards data to
//! [`SVWasmBackend`] and never keeps a duplicate figure.

use crate::SVBrowserCanvasHandle;
use crate::SVWasmBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

/// A figure presented on a browser canvas.
#[wasm_bindgen]
pub struct SVWasmFigure {
    backend: SVWasmBackend,
    values: Vec<f64>,
}

#[wasm_bindgen]
impl SVWasmFigure {
    /// Constructs a figure on the canvas with `canvas_id`.
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, values: Vec<f64>) -> Result<SVWasmFigure, JsValue> {
        let canvas = SVBrowserCanvasHandle::open(canvas_id).map_err(|error| sv_error(&error))?;
        let backend = SVWasmBackend::new().with_surface(Arc::new(canvas));

        Ok(Self { backend, values })
    }

    /// Replaces the data and presents the frame the model produced from it.
    pub fn update(&mut self, values: Vec<f64>) -> Result<(), JsValue> {
        self.values = values;
        self.render()
    }

    /// Presents the current figure.
    pub fn render(&self) -> Result<(), JsValue> {
        self.backend
            .show(&SVShowRequest {
                backend_kind: SVBackendKind::Wasm,
                execution_mode: SVExecutionMode::Automatic,
                view: SVColumnView::Dense(&self.values),
            })
            .map_err(|error| sv_error(&error))
    }
}

fn sv_error(error: &scienceview_core::sv_error::SVError) -> JsValue {
    JsValue::from_str(&error.to_string())
}
