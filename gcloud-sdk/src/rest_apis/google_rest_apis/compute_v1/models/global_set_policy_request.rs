/*
 * Compute Engine API
 *
 * Creates and runs virtual machines on Google Cloud Platform.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::compute_v1::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalSetPolicyRequest {
    /// Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify bindings.
    #[serde(rename = "bindings", skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<models::Binding>>,
    /// Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify the etag.
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Vec<u8>>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Box<models::Policy>>,
}

impl GlobalSetPolicyRequest {
    pub fn new() -> GlobalSetPolicyRequest {
        GlobalSetPolicyRequest {
            bindings: None,
            etag: None,
            policy: None,
        }
    }
}
