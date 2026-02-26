extern crate self as canonical_errors;

use std::collections::HashMap;
use std::fmt;

pub use canonical_errors_macros::resource_error;
use gts::schema::GtsSchema;
use gts_macros::struct_to_gts_schema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

// Workaround: the struct_to_gts_schema macro generates Deserialize that expects
// a `gts_type` field, but it's never in the JSON (skip_serializing). GtsSchemaId
// doesn't impl Default, so #[serde(skip)] can't be used. This dummy function
// provides a default for deserialization. The field is #[allow(dead_code)] and
// never read, so the value doesn't matter.
// TODO: Fix the gts macro to emit #[serde(skip)] with a built-in default.
fn dummy_gts_schema_id() -> gts::GtsSchemaId {
    serde_json::from_value(serde_json::Value::String("gts.placeholder~".into()))
        .expect("dummy GtsSchemaId construction should not fail")
}

// ---------------------------------------------------------------------------
// Context Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.field_violation.v1~",
    description = "A single field validation violation",
    properties = "field,description,reason"
)]
pub struct FieldViolationV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub field: String,
    pub description: String,
    pub reason: String,
}

pub type FieldViolation = FieldViolationV1;

impl FieldViolationV1 {
    pub fn new(
        field: impl Into<String>,
        description: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            field: field.into(),
            description: description.into(),
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Validation {
    FieldViolations {
        field_violations: Vec<FieldViolation>,
    },
    Format {
        format: String,
    },
    Constraint {
        constraint: String,
    },
}

impl GtsSchema for Validation {
    const SCHEMA_ID: &'static str = "gts.cf.core.errors.validation.v1~";

    fn gts_schema_with_refs() -> serde_json::Value {
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.validation.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "field_violations": {
                            "type": "array",
                            "items": { "$ref": "gts://gts.cf.core.errors.field_violation.v1~" }
                        }
                    },
                    "required": ["field_violations"]
                },
                {
                    "type": "object",
                    "properties": {
                        "format": { "type": "string" }
                    },
                    "required": ["format"]
                },
                {
                    "type": "object",
                    "properties": {
                        "constraint": { "type": "string" }
                    },
                    "required": ["constraint"]
                }
            ]
        })
    }
}

impl Validation {
    pub fn fields(violations: impl Into<Vec<FieldViolation>>) -> Self {
        Self::FieldViolations {
            field_violations: violations.into(),
        }
    }

    pub fn format(msg: impl Into<String>) -> Self {
        Self::Format { format: msg.into() }
    }

    pub fn constraint(msg: impl Into<String>) -> Self {
        Self::Constraint {
            constraint: msg.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.resource_info.v1~",
    description = "Resource identification context for resource-scoped errors",
    properties = "resource_type,resource_name,description"
)]
pub struct ResourceInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub resource_type: String,
    pub resource_name: String,
    pub description: String,
}

pub type ResourceInfo = ResourceInfoV1;

impl ResourceInfoV1 {
    pub fn new(resource_type: impl Into<String>, resource_name: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            resource_type: resource_type.into(),
            resource_name: resource_name.into(),
            description: String::from("Resource not found"),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.error_info.v1~",
    description = "Error information with reason, domain, and metadata",
    properties = "reason,domain,metadata"
)]
pub struct ErrorInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub reason: String,
    pub domain: String,
    pub metadata: HashMap<String, String>,
}

pub type ErrorInfo = ErrorInfoV1;

impl ErrorInfoV1 {
    pub fn new(reason: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            reason: reason.into(),
            domain: domain.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.quota_violation.v1~",
    description = "A single quota violation entry",
    properties = "subject,description"
)]
pub struct QuotaViolationV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub subject: String,
    pub description: String,
}

pub type QuotaViolation = QuotaViolationV1;

impl QuotaViolationV1 {
    pub fn new(subject: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            subject: subject.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.quota_failure.v1~",
    description = "Quota failure with one or more violations",
    properties = "violations"
)]
pub struct QuotaFailureV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub violations: Vec<QuotaViolation>,
}

pub type QuotaFailure = QuotaFailureV1;

impl QuotaFailureV1 {
    pub fn new(violations: impl Into<Vec<QuotaViolation>>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            violations: violations.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.precondition_violation.v1~",
    description = "A single precondition violation entry",
    properties = "precondition_type,subject,description"
)]
pub struct PreconditionViolationV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    #[serde(rename = "type")]
    pub precondition_type: String,
    pub subject: String,
    pub description: String,
}

pub type PreconditionViolation = PreconditionViolationV1;

impl PreconditionViolationV1 {
    pub fn new(
        precondition_type: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            precondition_type: precondition_type.into(),
            subject: subject.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.precondition_failure.v1~",
    description = "Precondition failure with one or more violations",
    properties = "violations"
)]
pub struct PreconditionFailureV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub violations: Vec<PreconditionViolation>,
}

pub type PreconditionFailure = PreconditionFailureV1;

impl PreconditionFailureV1 {
    pub fn new(violations: impl Into<Vec<PreconditionViolation>>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            violations: violations.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.debug_info.v1~",
    description = "Debug information with detail and stack trace",
    properties = "detail,stack_entries"
)]
pub struct DebugInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub detail: String,
    pub stack_entries: Vec<String>,
}

pub type DebugInfo = DebugInfoV1;

impl DebugInfoV1 {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            detail: detail.into(),
            stack_entries: Vec::new(),
        }
    }

    pub fn with_stack(mut self, entries: impl Into<Vec<String>>) -> Self {
        self.stack_entries = entries.into();
        self
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.retry_info.v1~",
    description = "Retry information for unavailable errors",
    properties = "retry_after_seconds"
)]
pub struct RetryInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub retry_after_seconds: u64,
}

pub type RetryInfo = RetryInfoV1;

impl RetryInfoV1 {
    pub fn after_seconds(seconds: u64) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            retry_after_seconds: seconds,
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.request_info.v1~",
    description = "Request identification context",
    properties = "request_id"
)]
pub struct RequestInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing, default = "dummy_gts_schema_id")]
    gts_type: gts::GtsSchemaId,
    pub request_id: String,
}

pub type RequestInfo = RequestInfoV1;

impl RequestInfoV1 {
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            request_id: request_id.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// CanonicalError Enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum CanonicalError {
    Cancelled {
        ctx: RequestInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    Unknown {
        ctx: DebugInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    InvalidArgument {
        ctx: Validation,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    DeadlineExceeded {
        ctx: RequestInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    NotFound {
        ctx: ResourceInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    AlreadyExists {
        ctx: ResourceInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    PermissionDenied {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    ResourceExhausted {
        ctx: QuotaFailure,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    FailedPrecondition {
        ctx: PreconditionFailure,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    Aborted {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    OutOfRange {
        ctx: Validation,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    Unimplemented {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    Internal {
        ctx: DebugInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    ServiceUnavailable {
        ctx: RetryInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    DataLoss {
        ctx: ResourceInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
    Unauthenticated {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
        debug_info: Option<DebugInfo>,
    },
}

impl CanonicalError {
    // --- Ergonomic constructors (one per category) ---

    pub fn cancelled(ctx: RequestInfo) -> Self {
        Self::Cancelled {
            ctx,
            message: String::from("Operation cancelled by the client"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn unknown(detail: impl Into<String>) -> Self {
        let detail = detail.into();
        let message = detail.clone();
        Self::Unknown {
            ctx: DebugInfo::new(detail),
            message,
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn invalid_argument(ctx: Validation) -> Self {
        let message = match &ctx {
            Validation::FieldViolations { .. } => String::from("Request validation failed"),
            Validation::Format { format } => format.clone(),
            Validation::Constraint { constraint } => constraint.clone(),
        };
        Self::InvalidArgument {
            ctx,
            message,
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn deadline_exceeded(ctx: RequestInfo) -> Self {
        Self::DeadlineExceeded {
            ctx,
            message: String::from("Operation did not complete within the allowed time"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn not_found(ctx: ResourceInfo) -> Self {
        Self::NotFound {
            ctx,
            message: String::from("Resource not found"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn already_exists(ctx: ResourceInfo) -> Self {
        let message = ctx.description.clone();
        Self::AlreadyExists {
            ctx,
            message,
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn permission_denied(ctx: ErrorInfo) -> Self {
        Self::PermissionDenied {
            ctx,
            message: String::from("You do not have permission to perform this operation"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn resource_exhausted(ctx: QuotaFailure) -> Self {
        Self::ResourceExhausted {
            ctx,
            message: String::from("Quota exceeded"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn failed_precondition(ctx: PreconditionFailure) -> Self {
        Self::FailedPrecondition {
            ctx,
            message: String::from("Operation precondition not met"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn aborted(ctx: ErrorInfo) -> Self {
        Self::Aborted {
            ctx,
            message: String::from("Operation aborted due to concurrency conflict"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn out_of_range(ctx: Validation) -> Self {
        let message = match &ctx {
            Validation::FieldViolations { .. } => String::from("Value out of range"),
            Validation::Format { format } => format.clone(),
            Validation::Constraint { constraint } => constraint.clone(),
        };
        Self::OutOfRange {
            ctx,
            message,
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn unimplemented(ctx: ErrorInfo) -> Self {
        Self::Unimplemented {
            ctx,
            message: String::from("This operation is not implemented"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn internal(ctx: DebugInfo) -> Self {
        Self::Internal {
            ctx,
            message: String::from("An internal error occurred. Please retry later."),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn service_unavailable(ctx: RetryInfo) -> Self {
        Self::ServiceUnavailable {
            ctx,
            message: String::from("Service temporarily unavailable"),
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn data_loss(ctx: ResourceInfo) -> Self {
        let message = ctx.description.clone();
        Self::DataLoss {
            ctx,
            message,
            resource_type: None,
            debug_info: None,
        }
    }

    pub fn unauthenticated(ctx: ErrorInfo) -> Self {
        Self::Unauthenticated {
            ctx,
            message: String::from("Authentication required"),
            resource_type: None,
            debug_info: None,
        }
    }

    // --- Builder methods ---

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        let msg = msg.into();
        match &mut self {
            Self::Cancelled { message, .. }
            | Self::Unknown { message, .. }
            | Self::InvalidArgument { message, .. }
            | Self::DeadlineExceeded { message, .. }
            | Self::NotFound { message, .. }
            | Self::AlreadyExists { message, .. }
            | Self::PermissionDenied { message, .. }
            | Self::ResourceExhausted { message, .. }
            | Self::FailedPrecondition { message, .. }
            | Self::Aborted { message, .. }
            | Self::OutOfRange { message, .. }
            | Self::Unimplemented { message, .. }
            | Self::Internal { message, .. }
            | Self::ServiceUnavailable { message, .. }
            | Self::DataLoss { message, .. }
            | Self::Unauthenticated { message, .. } => *message = msg,
        }
        self
    }

    pub fn with_resource_type(mut self, rt: impl Into<String>) -> Self {
        let rt = Some(rt.into());
        match &mut self {
            Self::Cancelled { resource_type, .. }
            | Self::Unknown { resource_type, .. }
            | Self::InvalidArgument { resource_type, .. }
            | Self::DeadlineExceeded { resource_type, .. }
            | Self::NotFound { resource_type, .. }
            | Self::AlreadyExists { resource_type, .. }
            | Self::PermissionDenied { resource_type, .. }
            | Self::ResourceExhausted { resource_type, .. }
            | Self::FailedPrecondition { resource_type, .. }
            | Self::Aborted { resource_type, .. }
            | Self::OutOfRange { resource_type, .. }
            | Self::Unimplemented { resource_type, .. }
            | Self::Internal { resource_type, .. }
            | Self::ServiceUnavailable { resource_type, .. }
            | Self::DataLoss { resource_type, .. }
            | Self::Unauthenticated { resource_type, .. } => *resource_type = rt,
        }
        self
    }

    pub fn with_debug_info(mut self, info: DebugInfo) -> Self {
        match &mut self {
            Self::Cancelled { debug_info, .. }
            | Self::Unknown { debug_info, .. }
            | Self::InvalidArgument { debug_info, .. }
            | Self::DeadlineExceeded { debug_info, .. }
            | Self::NotFound { debug_info, .. }
            | Self::AlreadyExists { debug_info, .. }
            | Self::PermissionDenied { debug_info, .. }
            | Self::ResourceExhausted { debug_info, .. }
            | Self::FailedPrecondition { debug_info, .. }
            | Self::Aborted { debug_info, .. }
            | Self::OutOfRange { debug_info, .. }
            | Self::Unimplemented { debug_info, .. }
            | Self::Internal { debug_info, .. }
            | Self::ServiceUnavailable { debug_info, .. }
            | Self::DataLoss { debug_info, .. }
            | Self::Unauthenticated { debug_info, .. } => *debug_info = Some(info),
        }
        self
    }

    // --- Accessors ---

    pub fn message(&self) -> &str {
        match self {
            Self::Cancelled { message, .. }
            | Self::Unknown { message, .. }
            | Self::InvalidArgument { message, .. }
            | Self::DeadlineExceeded { message, .. }
            | Self::NotFound { message, .. }
            | Self::AlreadyExists { message, .. }
            | Self::PermissionDenied { message, .. }
            | Self::ResourceExhausted { message, .. }
            | Self::FailedPrecondition { message, .. }
            | Self::Aborted { message, .. }
            | Self::OutOfRange { message, .. }
            | Self::Unimplemented { message, .. }
            | Self::Internal { message, .. }
            | Self::ServiceUnavailable { message, .. }
            | Self::DataLoss { message, .. }
            | Self::Unauthenticated { message, .. } => message,
        }
    }

    pub fn resource_type(&self) -> Option<&str> {
        match self {
            Self::Cancelled { resource_type, .. }
            | Self::Unknown { resource_type, .. }
            | Self::InvalidArgument { resource_type, .. }
            | Self::DeadlineExceeded { resource_type, .. }
            | Self::NotFound { resource_type, .. }
            | Self::AlreadyExists { resource_type, .. }
            | Self::PermissionDenied { resource_type, .. }
            | Self::ResourceExhausted { resource_type, .. }
            | Self::FailedPrecondition { resource_type, .. }
            | Self::Aborted { resource_type, .. }
            | Self::OutOfRange { resource_type, .. }
            | Self::Unimplemented { resource_type, .. }
            | Self::Internal { resource_type, .. }
            | Self::ServiceUnavailable { resource_type, .. }
            | Self::DataLoss { resource_type, .. }
            | Self::Unauthenticated { resource_type, .. } => resource_type.as_deref(),
        }
    }

    pub fn debug_info(&self) -> Option<&DebugInfo> {
        match self {
            Self::Cancelled { debug_info, .. }
            | Self::Unknown { debug_info, .. }
            | Self::InvalidArgument { debug_info, .. }
            | Self::DeadlineExceeded { debug_info, .. }
            | Self::NotFound { debug_info, .. }
            | Self::AlreadyExists { debug_info, .. }
            | Self::PermissionDenied { debug_info, .. }
            | Self::ResourceExhausted { debug_info, .. }
            | Self::FailedPrecondition { debug_info, .. }
            | Self::Aborted { debug_info, .. }
            | Self::OutOfRange { debug_info, .. }
            | Self::Unimplemented { debug_info, .. }
            | Self::Internal { debug_info, .. }
            | Self::ServiceUnavailable { debug_info, .. }
            | Self::DataLoss { debug_info, .. }
            | Self::Unauthenticated { debug_info, .. } => debug_info.as_ref(),
        }
    }

    // --- GTS Catalog ---

    pub fn gts_type(&self) -> &'static str {
        match self {
            Self::Cancelled { .. } => "gts.cf.core.errors.err.v1~cf.core.errors.cancelled.v1~",
            Self::Unknown { .. } => "gts.cf.core.errors.err.v1~cf.core.errors.unknown.v1~",
            Self::InvalidArgument { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.invalid_argument.v1~"
            }
            Self::DeadlineExceeded { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.deadline_exceeded.v1~"
            }
            Self::NotFound { .. } => "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
            Self::AlreadyExists { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.already_exists.v1~"
            }
            Self::PermissionDenied { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.permission_denied.v1~"
            }
            Self::ResourceExhausted { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.resource_exhausted.v1~"
            }
            Self::FailedPrecondition { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.failed_precondition.v1~"
            }
            Self::Aborted { .. } => "gts.cf.core.errors.err.v1~cf.core.errors.aborted.v1~",
            Self::OutOfRange { .. } => "gts.cf.core.errors.err.v1~cf.core.errors.out_of_range.v1~",
            Self::Unimplemented { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.unimplemented.v1~"
            }
            Self::Internal { .. } => "gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~",
            Self::ServiceUnavailable { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.service_unavailable.v1~"
            }
            Self::DataLoss { .. } => "gts.cf.core.errors.err.v1~cf.core.errors.data_loss.v1~",
            Self::Unauthenticated { .. } => {
                "gts.cf.core.errors.err.v1~cf.core.errors.unauthenticated.v1~"
            }
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Self::Cancelled { .. } => 499,
            Self::Unknown { .. } => 500,
            Self::InvalidArgument { .. } => 400,
            Self::DeadlineExceeded { .. } => 504,
            Self::NotFound { .. } => 404,
            Self::AlreadyExists { .. } => 409,
            Self::PermissionDenied { .. } => 403,
            Self::ResourceExhausted { .. } => 429,
            Self::FailedPrecondition { .. } => 400,
            Self::Aborted { .. } => 409,
            Self::OutOfRange { .. } => 400,
            Self::Unimplemented { .. } => 501,
            Self::Internal { .. } => 500,
            Self::ServiceUnavailable { .. } => 503,
            Self::DataLoss { .. } => 500,
            Self::Unauthenticated { .. } => 401,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Cancelled { .. } => "Cancelled",
            Self::Unknown { .. } => "Unknown",
            Self::InvalidArgument { .. } => "Invalid Argument",
            Self::DeadlineExceeded { .. } => "Deadline Exceeded",
            Self::NotFound { .. } => "Not Found",
            Self::AlreadyExists { .. } => "Already Exists",
            Self::PermissionDenied { .. } => "Permission Denied",
            Self::ResourceExhausted { .. } => "Resource Exhausted",
            Self::FailedPrecondition { .. } => "Failed Precondition",
            Self::Aborted { .. } => "Aborted",
            Self::OutOfRange { .. } => "Out of Range",
            Self::Unimplemented { .. } => "Unimplemented",
            Self::Internal { .. } => "Internal",
            Self::ServiceUnavailable { .. } => "Unavailable",
            Self::DataLoss { .. } => "Data Loss",
            Self::Unauthenticated { .. } => "Unauthenticated",
        }
    }

    fn category_name(&self) -> &'static str {
        match self {
            Self::Cancelled { .. } => "cancelled",
            Self::Unknown { .. } => "unknown",
            Self::InvalidArgument { .. } => "invalid_argument",
            Self::DeadlineExceeded { .. } => "deadline_exceeded",
            Self::NotFound { .. } => "not_found",
            Self::AlreadyExists { .. } => "already_exists",
            Self::PermissionDenied { .. } => "permission_denied",
            Self::ResourceExhausted { .. } => "resource_exhausted",
            Self::FailedPrecondition { .. } => "failed_precondition",
            Self::Aborted { .. } => "aborted",
            Self::OutOfRange { .. } => "out_of_range",
            Self::Unimplemented { .. } => "unimplemented",
            Self::Internal { .. } => "internal",
            Self::ServiceUnavailable { .. } => "unavailable",
            Self::DataLoss { .. } => "data_loss",
            Self::Unauthenticated { .. } => "unauthenticated",
        }
    }
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.category_name(), self.message())
    }
}

impl std::error::Error for CanonicalError {}

impl GtsSchema for CanonicalError {
    const SCHEMA_ID: &'static str = "gts.cf.core.errors.canonical_error.v1~";

    fn gts_schema_with_refs() -> serde_json::Value {
        let variant = |name: &str, ctx_ref: &str| {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "category": { "const": name },
                    "message": { "type": "string" },
                    "resource_type": { "type": "string" },
                    "context": { "$ref": ctx_ref }
                },
                "required": ["category", "message", "context"]
            })
        };

        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.canonical_error.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "oneOf": [
                variant("cancelled",          &format!("gts://{}", RequestInfoV1::SCHEMA_ID)),
                variant("unknown",            &format!("gts://{}", DebugInfoV1::SCHEMA_ID)),
                variant("invalid_argument",   &format!("gts://{}", Validation::SCHEMA_ID)),
                variant("deadline_exceeded",   &format!("gts://{}", RequestInfoV1::SCHEMA_ID)),
                variant("not_found",          &format!("gts://{}", ResourceInfoV1::SCHEMA_ID)),
                variant("already_exists",     &format!("gts://{}", ResourceInfoV1::SCHEMA_ID)),
                variant("permission_denied",  &format!("gts://{}", ErrorInfoV1::SCHEMA_ID)),
                variant("resource_exhausted", &format!("gts://{}", QuotaFailureV1::SCHEMA_ID)),
                variant("failed_precondition", &format!("gts://{}", PreconditionFailureV1::SCHEMA_ID)),
                variant("aborted",            &format!("gts://{}", ErrorInfoV1::SCHEMA_ID)),
                variant("out_of_range",       &format!("gts://{}", Validation::SCHEMA_ID)),
                variant("unimplemented",      &format!("gts://{}", ErrorInfoV1::SCHEMA_ID)),
                variant("internal",           &format!("gts://{}", DebugInfoV1::SCHEMA_ID)),
                variant("unavailable",        &format!("gts://{}", RetryInfoV1::SCHEMA_ID)),
                variant("data_loss",          &format!("gts://{}", ResourceInfoV1::SCHEMA_ID)),
                variant("unauthenticated",    &format!("gts://{}", ErrorInfoV1::SCHEMA_ID))
            ]
        })
    }
}

// ---------------------------------------------------------------------------
// Problem (RFC 9457)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub context: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<serde_json::Value>,
}

impl Problem {
    /// Converts a `CanonicalError` into a `Problem` response (production mode).
    /// Debug info is always omitted.
    pub fn from_error(err: CanonicalError) -> Self {
        Self::build(err, false)
    }

    /// Converts a `CanonicalError` into a `Problem` response (debug mode).
    /// If the error carries `debug_info`, it is included as a top-level `"debug"` key.
    pub fn from_error_debug(err: CanonicalError) -> Self {
        Self::build(err, true)
    }

    fn build(err: CanonicalError, include_debug: bool) -> Self {
        let problem_type = err.gts_type().to_string();
        let title = err.title().to_string();
        let status = err.status_code();
        let detail = err.message().to_string();
        let mut context = match &err {
            CanonicalError::Cancelled { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Unknown { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::InvalidArgument { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::DeadlineExceeded { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::NotFound { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::AlreadyExists { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::PermissionDenied { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::ResourceExhausted { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::FailedPrecondition { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Aborted { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::OutOfRange { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Unimplemented { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Internal { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::ServiceUnavailable { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::DataLoss { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Unauthenticated { ctx, .. } => serde_json::to_value(ctx),
        }
        .expect("context serialization should not fail");

        if let Some(rt) = err.resource_type() {
            context["resource_type"] = serde_json::Value::String(rt.to_string());
        }

        let debug_value = if include_debug {
            err.debug_info()
                .map(|info| serde_json::to_value(info).expect("debug info serialization should not fail"))
        } else {
            None
        };

        Problem {
            problem_type,
            title,
            status,
            detail,
            instance: None,
            trace_id: None,
            context,
            debug: debug_value,
        }
    }
}

impl From<CanonicalError> for Problem {
    fn from(err: CanonicalError) -> Self {
        Problem::from_error(err)
    }
}

// ---------------------------------------------------------------------------
// Problem → CanonicalError (deserialization / round-trip)
// ---------------------------------------------------------------------------

/// Error returned when a `Problem` cannot be converted into a `CanonicalError`.
#[derive(Debug)]
pub enum ProblemConversionError {
    /// The `type` URI does not have the expected GTS prefix format.
    InvalidType(String),
    /// The category extracted from the `type` URI is not one of the 16 known categories.
    UnknownCategory(String),
    /// The `context` JSON could not be deserialized into the expected struct for this category.
    ContextDeserializationFailed {
        category: String,
        source: serde_json::Error,
    },
}

impl fmt::Display for ProblemConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidType(t) => write!(f, "invalid GTS type URI: {t}"),
            Self::UnknownCategory(c) => write!(f, "unknown canonical error category: {c}"),
            Self::ContextDeserializationFailed { category, source } => {
                write!(f, "failed to deserialize context for {category}: {source}")
            }
        }
    }
}

impl std::error::Error for ProblemConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ContextDeserializationFailed { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// GTS type URI prefix: `gts.cf.core.errors.err.v1~cf.core.errors.`
const GTS_TYPE_PREFIX: &str = "gts.cf.core.errors.err.v1~cf.core.errors.";
/// GTS type URI suffix: `.v1~`
const GTS_TYPE_SUFFIX: &str = ".v1~";

/// Parses a GTS compound type URI and returns the category name.
///
/// Example: `"gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~"` → `"not_found"`
fn parse_category(problem_type: &str) -> Result<&str, ProblemConversionError> {
    let without_prefix = problem_type
        .strip_prefix(GTS_TYPE_PREFIX)
        .ok_or_else(|| ProblemConversionError::InvalidType(problem_type.to_string()))?;
    let category = without_prefix
        .strip_suffix(GTS_TYPE_SUFFIX)
        .ok_or_else(|| ProblemConversionError::InvalidType(problem_type.to_string()))?;
    Ok(category)
}

/// Extracts `resource_type` from a context JSON value (if present) as `Option<String>`.
fn extract_resource_type(context: &serde_json::Value) -> Option<String> {
    context
        .get("resource_type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Deserializes a typed context from a `serde_json::Value`, mapping errors to `ProblemConversionError`.
fn deser_ctx<T: DeserializeOwned>(
    context: serde_json::Value,
    category: &str,
) -> Result<T, ProblemConversionError> {
    serde_json::from_value(context).map_err(|source| ProblemConversionError::ContextDeserializationFailed {
        category: category.to_string(),
        source,
    })
}

impl TryFrom<Problem> for CanonicalError {
    type Error = ProblemConversionError;

    fn try_from(problem: Problem) -> Result<Self, Self::Error> {
        let category = parse_category(&problem.problem_type)?;
        let resource_type = extract_resource_type(&problem.context);
        let debug_info: Option<DebugInfo> = problem
            .debug
            .map(|v| serde_json::from_value(v))
            .transpose()
            .map_err(|source| ProblemConversionError::ContextDeserializationFailed {
                category: category.to_string(),
                source,
            })?;
        let message = problem.detail;

        match category {
            "cancelled" => Ok(CanonicalError::Cancelled {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "unknown" => Ok(CanonicalError::Unknown {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "invalid_argument" => Ok(CanonicalError::InvalidArgument {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "deadline_exceeded" => Ok(CanonicalError::DeadlineExceeded {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "not_found" => Ok(CanonicalError::NotFound {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "already_exists" => Ok(CanonicalError::AlreadyExists {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "permission_denied" => Ok(CanonicalError::PermissionDenied {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "resource_exhausted" => Ok(CanonicalError::ResourceExhausted {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "failed_precondition" => Ok(CanonicalError::FailedPrecondition {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "aborted" => Ok(CanonicalError::Aborted {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "out_of_range" => Ok(CanonicalError::OutOfRange {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "unimplemented" => Ok(CanonicalError::Unimplemented {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "internal" => Ok(CanonicalError::Internal {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "service_unavailable" => Ok(CanonicalError::ServiceUnavailable {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "data_loss" => Ok(CanonicalError::DataLoss {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            "unauthenticated" => Ok(CanonicalError::Unauthenticated {
                ctx: deser_ctx(problem.context, category)?,
                message,
                resource_type,
                debug_info,
            }),
            _ => Err(ProblemConversionError::UnknownCategory(category.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_types_are_deserializable() {
        // FieldViolation
        let json = r#"{"field":"email","description":"is required","reason":"REQUIRED"}"#;
        let fv: FieldViolation = serde_json::from_str(json).unwrap();
        assert_eq!(fv.field, "email");
        assert_eq!(fv.description, "is required");
        assert_eq!(fv.reason, "REQUIRED");

        // ResourceInfo
        let json = r#"{"resource_type":"gts.cf.core.users.user.v1","resource_name":"user-123","description":"Resource not found"}"#;
        let ri: ResourceInfo = serde_json::from_str(json).unwrap();
        assert_eq!(ri.resource_name, "user-123");
    }

    #[test]
    fn not_found_gts_type() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
        assert_eq!(
            err.gts_type(),
            "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~"
        );
    }

    #[test]
    fn not_found_status_code() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
        assert_eq!(err.status_code(), 404);
    }

    #[test]
    fn not_found_title() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
        assert_eq!(err.title(), "Not Found");
    }

    #[test]
    fn display_includes_category_and_message() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_message("User not found");
        assert_eq!(format!("{err}"), "not_found: User not found");
    }

    #[test]
    fn with_message_overrides_default() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_message("custom detail");
        assert_eq!(err.message(), "custom detail");
    }

    #[test]
    fn problem_from_not_found_has_correct_fields() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
        let problem = Problem::from(err);
        assert_eq!(
            problem.problem_type,
            "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~"
        );
        assert_eq!(problem.title, "Not Found");
        assert_eq!(problem.status, 404);
        assert_eq!(problem.detail, "Resource not found");
        assert_eq!(
            problem.context["resource_type"],
            "gts.cf.core.users.user.v1"
        );
        assert_eq!(problem.context["resource_name"], "user-123");
    }

    #[test]
    fn problem_json_excludes_none_fields() {
        let err = CanonicalError::service_unavailable(RetryInfo::after_seconds(30));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();
        assert!(json.get("trace_id").is_none());
    }

    #[test]
    fn validation_field_violations_serialization() {
        let v = Validation::fields(vec![FieldViolation::new(
            "email",
            "must be valid",
            "INVALID_FORMAT",
        )]);
        let json = serde_json::to_value(&v).unwrap();
        assert!(json["field_violations"].is_array());
        assert_eq!(json["field_violations"][0]["field"], "email");
    }

    #[test]
    fn validation_format_serialization() {
        let v = Validation::format("bad json");
        let json = serde_json::to_value(&v).unwrap();
        assert_eq!(json["format"], "bad json");
    }

    #[test]
    fn validation_constraint_serialization() {
        let v = Validation::constraint("too many items");
        let json = serde_json::to_value(&v).unwrap();
        assert_eq!(json["constraint"], "too many items");
    }

    #[test]
    fn all_16_categories_convert_to_problem() {
        let errors: Vec<CanonicalError> = vec![
            CanonicalError::cancelled(RequestInfo::new("req-1")),
            CanonicalError::unknown("unknown error"),
            CanonicalError::invalid_argument(Validation::format("bad")),
            CanonicalError::deadline_exceeded(RequestInfo::new("req-2")),
            CanonicalError::not_found(ResourceInfo::new("t", "n")),
            CanonicalError::already_exists(ResourceInfo::new("t", "n")),
            CanonicalError::permission_denied(ErrorInfo::new("R", "D")),
            CanonicalError::resource_exhausted(QuotaFailure::new(vec![])),
            CanonicalError::failed_precondition(PreconditionFailure::new(vec![])),
            CanonicalError::aborted(ErrorInfo::new("R", "D")),
            CanonicalError::out_of_range(Validation::constraint("x")),
            CanonicalError::unimplemented(ErrorInfo::new("R", "D")),
            CanonicalError::internal(DebugInfo::new("bug")),
            CanonicalError::service_unavailable(RetryInfo::after_seconds(10)),
            CanonicalError::data_loss(ResourceInfo::new("t", "n")),
            CanonicalError::unauthenticated(ErrorInfo::new("R", "D")),
        ];
        assert_eq!(errors.len(), 16);
        for err in errors {
            let problem = Problem::from(err);
            assert!(!problem.problem_type.is_empty());
            assert!(!problem.title.is_empty());
            assert!(problem.status > 0);
        }
    }

    // --- New tests for resource_error! macro ---

    #[test]
    fn macro_not_found_has_correct_resource_type_and_resource_info() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct TestUserResourceError;

        let err = TestUserResourceError::not_found("user-123");
        assert_eq!(err.resource_type(), Some("gts.cf.core.users.user.v1"));
        assert_eq!(
            err.gts_type(),
            "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~"
        );
        let problem = Problem::from(err);
        assert_eq!(
            problem.context["resource_type"],
            "gts.cf.core.users.user.v1"
        );
        assert_eq!(problem.context["resource_name"], "user-123");
    }

    #[test]
    fn macro_permission_denied_has_correct_resource_type() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct TestUserResourceError;

        let err = TestUserResourceError::permission_denied(ErrorInfo::new(
            "CROSS_TENANT_ACCESS",
            "auth.cyberfabric.io",
        ));
        assert_eq!(err.resource_type(), Some("gts.cf.core.users.user.v1"));
        assert_eq!(
            err.gts_type(),
            "gts.cf.core.errors.err.v1~cf.core.errors.permission_denied.v1~"
        );
    }

    #[test]
    fn direct_constructor_has_no_resource_type() {
        let err = CanonicalError::service_unavailable(RetryInfo::after_seconds(30));
        assert_eq!(err.resource_type(), None);
        let _problem = Problem::from(err);
    }

    #[test]
    fn problem_json_includes_resource_type_when_set() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct TestUserResourceError;

        let err = TestUserResourceError::not_found("user-123");
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();
        assert_eq!(
            json["context"]["resource_type"],
            "gts.cf.core.users.user.v1"
        );
    }

    #[test]
    fn problem_json_excludes_resource_type_when_none() {
        let err = CanonicalError::unknown("some error");
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();
        assert!(json["context"].get("resource_type").is_none());
    }

    // --- debug_info tests ---

    #[test]
    fn with_debug_info_attaches_and_accessor_returns_it() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_debug_info(DebugInfo::new("SELECT * FROM users WHERE id = $1 returned 0 rows"));
        let info = err.debug_info().expect("debug_info should be Some");
        assert_eq!(info.detail, "SELECT * FROM users WHERE id = $1 returned 0 rows");
        // Verify other fields are unchanged
        assert_eq!(err.gts_type(), "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~");
        assert_eq!(err.message(), "Resource not found");
        assert_eq!(err.resource_type(), None);
        assert_eq!(err.status_code(), 404);
    }

    #[test]
    fn with_debug_info_preserves_stack_entries() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_debug_info(
                    DebugInfo::new("connection reset")
                        .with_stack(vec!["cf_users::repo::find_by_id (src/repo.rs:42)".into()]),
                );
        let info = err.debug_info().expect("debug_info should be Some");
        assert_eq!(info.detail, "connection reset");
        assert_eq!(info.stack_entries, vec!["cf_users::repo::find_by_id (src/repo.rs:42)"]);
    }

    #[test]
    fn default_construction_has_no_debug_info() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
        assert!(err.debug_info().is_none());
    }

    #[test]
    fn problem_from_error_debug_true_includes_debug_key() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_debug_info(DebugInfo::new("query returned 0 rows"));
        let problem = Problem::from_error_debug(err);
        let json = serde_json::to_value(&problem).unwrap();
        let debug = json.get("debug").expect("debug key should be present");
        assert_eq!(debug["detail"], "query returned 0 rows");
    }

    #[test]
    fn problem_from_error_debug_false_omits_debug_key() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_debug_info(DebugInfo::new("query returned 0 rows"));
        let problem = Problem::from_error(err);
        let json = serde_json::to_value(&problem).unwrap();
        assert!(json.get("debug").is_none(), "debug key should be absent");
    }

    #[test]
    fn problem_from_backward_compat_omits_debug_key() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_debug_info(DebugInfo::new("query returned 0 rows"));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();
        assert!(json.get("debug").is_none(), "From impl should not include debug");
    }

    #[test]
    fn problem_from_error_false_is_byte_identical_to_from() {
        let err1 =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
                .with_debug_info(DebugInfo::new("query returned 0 rows"));
        let err2 = err1.clone();
        let json_from = serde_json::to_string(&Problem::from(err1)).unwrap();
        let json_from_error = serde_json::to_string(&Problem::from_error(err2)).unwrap();
        assert_eq!(json_from, json_from_error, "from_error(err) should be byte-identical to From::from(err)");
    }

    #[test]
    fn problem_from_error_no_debug_info_debug_true_omits_debug_key() {
        let err =
            CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
        let problem = Problem::from_error_debug(err);
        let json = serde_json::to_value(&problem).unwrap();
        assert!(json.get("debug").is_none(), "no debug_info means no debug key regardless of flag");
    }

    // =========================================================================
    // Showcase tests — resource-scoped categories (macro-generated)
    // =========================================================================

    #[test]
    fn showcase_not_found() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct UserResourceError;

        let err = UserResourceError::not_found("user-123");
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
                "title": "Not Found",
                "status": 404,
                "detail": "Resource not found",
                "context": {
                    "resource_type": "gts.cf.core.users.user.v1",
                    "resource_name": "user-123",
                    "description": "Resource not found"
                }
            })
        );
    }

    #[test]
    fn showcase_already_exists() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct UserResourceError;

        let err = UserResourceError::already_exists("alice@example.com");
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.already_exists.v1~",
                "title": "Already Exists",
                "status": 409,
                "detail": "Resource already exists",
                "context": {
                    "resource_type": "gts.cf.core.users.user.v1",
                    "resource_name": "alice@example.com",
                    "description": "Resource already exists"
                }
            })
        );
    }

    #[test]
    fn showcase_data_loss() {
        #[resource_error("gts.cf.core.files.file.v1")]
        struct FileResourceError;

        let err = FileResourceError::data_loss("01JFILE-ABC");
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.data_loss.v1~",
                "title": "Data Loss",
                "status": 500,
                "detail": "Data loss detected",
                "context": {
                    "resource_type": "gts.cf.core.files.file.v1",
                    "resource_name": "01JFILE-ABC",
                    "description": "Data loss detected"
                }
            })
        );
    }

    #[test]
    fn showcase_invalid_argument() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct UserResourceError;

        // --- Simulated user input ---
        let email = "not-an-email";
        let age: u8 = 12;

        // --- Anticipated user code: validate fields, collect violations ---
        let mut violations = Vec::new();

        if !email.contains('@') {
            violations.push(FieldViolation::new(
                "email",
                "must be a valid email address",
                "INVALID_FORMAT",
            ));
        }
        if age < 18 {
            violations.push(FieldViolation::new(
                "age",
                "must be at least 18",
                "OUT_OF_RANGE",
            ));
        }

        assert!(!violations.is_empty());
        let err = UserResourceError::invalid_argument(Validation::fields(violations));

        // --- Wire format ---
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.invalid_argument.v1~",
                "title": "Invalid Argument",
                "status": 400,
                "detail": "Request validation failed",
                "context": {
                    "resource_type": "gts.cf.core.users.user.v1",
                    "field_violations": [
                        {
                            "field": "email",
                            "description": "must be a valid email address",
                            "reason": "INVALID_FORMAT"
                        },
                        {
                            "field": "age",
                            "description": "must be at least 18",
                            "reason": "OUT_OF_RANGE"
                        }
                    ]
                }
            })
        );
    }

    #[test]
    fn showcase_out_of_range() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct UserResourceError;

        let err = UserResourceError::out_of_range(Validation::constraint(
            "Page 50 is beyond the last page (12)",
        ));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.out_of_range.v1~",
                "title": "Out of Range",
                "status": 400,
                "detail": "Page 50 is beyond the last page (12)",
                "context": {
                    "resource_type": "gts.cf.core.users.user.v1",
                    "constraint": "Page 50 is beyond the last page (12)"
                }
            })
        );
    }

    #[test]
    fn showcase_permission_denied() {
        #[resource_error("gts.cf.core.tenants.tenant.v1")]
        struct TenantResourceError;

        let err = TenantResourceError::permission_denied(ErrorInfo::new(
            "CROSS_TENANT_ACCESS",
            "auth.cyberfabric.io",
        ));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.permission_denied.v1~",
                "title": "Permission Denied",
                "status": 403,
                "detail": "You do not have permission to perform this operation",
                "context": {
                    "resource_type": "gts.cf.core.tenants.tenant.v1",
                    "reason": "CROSS_TENANT_ACCESS",
                    "domain": "auth.cyberfabric.io",
                    "metadata": {}
                }
            })
        );
    }

    #[test]
    fn showcase_aborted() {
        #[resource_error("gts.cf.oagw.upstreams.upstream.v1")]
        struct UpstreamResourceError;

        let err = UpstreamResourceError::aborted(
            ErrorInfo::new("OPTIMISTIC_LOCK_FAILURE", "cf.oagw")
                .with_metadata("expected_version", "3")
                .with_metadata("actual_version", "5"),
        );
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.aborted.v1~",
                "title": "Aborted",
                "status": 409,
                "detail": "Operation aborted due to concurrency conflict",
                "context": {
                    "resource_type": "gts.cf.oagw.upstreams.upstream.v1",
                    "reason": "OPTIMISTIC_LOCK_FAILURE",
                    "domain": "cf.oagw",
                    "metadata": {
                        "expected_version": "3",
                        "actual_version": "5"
                    }
                }
            })
        );
    }

    #[test]
    fn showcase_unimplemented() {
        #[resource_error("gts.cf.oagw.upstreams.upstream.v1")]
        struct UpstreamResourceError;

        let err = UpstreamResourceError::unimplemented(ErrorInfo::new("GRPC_ROUTING", "cf.oagw"));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.unimplemented.v1~",
                "title": "Unimplemented",
                "status": 501,
                "detail": "This operation is not implemented",
                "context": {
                    "resource_type": "gts.cf.oagw.upstreams.upstream.v1",
                    "reason": "GRPC_ROUTING",
                    "domain": "cf.oagw",
                    "metadata": {}
                }
            })
        );
    }

    #[test]
    fn showcase_failed_precondition() {
        #[resource_error("gts.cf.core.tenants.tenant.v1")]
        struct TenantResourceError;

        let err = TenantResourceError::failed_precondition(PreconditionFailure::new(vec![
            PreconditionViolation::new(
                "STATE",
                "tenant.users",
                "Tenant must have zero active users before deletion",
            ),
        ]));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.failed_precondition.v1~",
                "title": "Failed Precondition",
                "status": 400,
                "detail": "Operation precondition not met",
                "context": {
                    "resource_type": "gts.cf.core.tenants.tenant.v1",
                    "violations": [
                        {
                            "type": "STATE",
                            "subject": "tenant.users",
                            "description": "Tenant must have zero active users before deletion"
                        }
                    ]
                }
            })
        );
    }

    #[test]
    fn showcase_internal() {
        #[resource_error("gts.cf.core.tenants.tenant.v1")]
        struct TenantResourceError;

        let err = TenantResourceError::internal(DebugInfo::new(
            "An internal error occurred. Please retry later.",
        ));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~",
                "title": "Internal",
                "status": 500,
                "detail": "An internal error occurred. Please retry later.",
                "context": {
                    "resource_type": "gts.cf.core.tenants.tenant.v1",
                    "detail": "An internal error occurred. Please retry later.",
                    "stack_entries": []
                }
            })
        );
    }

    #[test]
    fn showcase_deadline_exceeded() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct UserResourceError;

        let err = UserResourceError::deadline_exceeded(RequestInfo::new("01JREQ-ABC"));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.deadline_exceeded.v1~",
                "title": "Deadline Exceeded",
                "status": 504,
                "detail": "Operation did not complete within the allowed time",
                "context": {
                    "resource_type": "gts.cf.core.users.user.v1",
                    "request_id": "01JREQ-ABC"
                }
            })
        );
    }

    #[test]
    fn showcase_cancelled() {
        #[resource_error("gts.cf.oagw.upstreams.upstream.v1")]
        struct UpstreamResourceError;

        let err = UpstreamResourceError::cancelled(RequestInfo::new("01JREQ-DEF"));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.cancelled.v1~",
                "title": "Cancelled",
                "status": 499,
                "detail": "Operation cancelled by the client",
                "context": {
                    "resource_type": "gts.cf.oagw.upstreams.upstream.v1",
                    "request_id": "01JREQ-DEF"
                }
            })
        );
    }

    // =========================================================================
    // Showcase tests — system-level categories (direct constructors)
    // =========================================================================

    #[test]
    fn showcase_unauthenticated() {
        let err = CanonicalError::unauthenticated(
            ErrorInfo::new("TOKEN_EXPIRED", "auth.cyberfabric.io")
                .with_metadata("expires_at", "2026-02-25T10:00:00Z"),
        );
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.unauthenticated.v1~",
                "title": "Unauthenticated",
                "status": 401,
                "detail": "Authentication required",
                "context": {
                    "reason": "TOKEN_EXPIRED",
                    "domain": "auth.cyberfabric.io",
                    "metadata": {
                        "expires_at": "2026-02-25T10:00:00Z"
                    }
                }
            })
        );
    }

    #[test]
    fn showcase_resource_exhausted() {
        let err = CanonicalError::resource_exhausted(QuotaFailure::new(vec![QuotaViolation::new(
            "requests_per_minute",
            "Limit of 100 requests per minute exceeded",
        )]));
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.resource_exhausted.v1~",
                "title": "Resource Exhausted",
                "status": 429,
                "detail": "Quota exceeded",
                "context": {
                    "violations": [
                        {
                            "subject": "requests_per_minute",
                            "description": "Limit of 100 requests per minute exceeded"
                        }
                    ]
                }
            })
        );
    }

    #[test]
    fn showcase_unavailable() {
        let err = CanonicalError::service_unavailable(RetryInfo::after_seconds(30));

        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.service_unavailable.v1~",
                "title": "Unavailable",
                "status": 503,
                "detail": "Service temporarily unavailable",
                "context": {
                    "retry_after_seconds": 30
                }
            })
        );
    }

    #[test]
    fn showcase_unknown() {
        let err = CanonicalError::unknown("Unexpected response from payment provider");
        let problem = Problem::from(err);
        let json = serde_json::to_value(&problem).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.unknown.v1~",
                "title": "Unknown",
                "status": 500,
                "detail": "Unexpected response from payment provider",
                "context": {
                    "detail": "Unexpected response from payment provider",
                    "stack_entries": []
                }
            })
        );
    }

    // =========================================================================
    // Showcase tests — debug info (conditional output via from_error / from_error_debug)
    // =========================================================================

    #[test]
    fn showcase_not_found_with_debug_info() {
        #[resource_error("gts.cf.core.users.user.v1")]
        struct UserResourceError;

        let err = UserResourceError::not_found("user-123").with_debug_info(
            DebugInfo::new("SELECT * FROM users WHERE id = $1 returned 0 rows")
                .with_stack(vec![
                    "cf_users::repo::find_by_id (src/repo.rs:42)".into(),
                ]),
        );

        // Debug mode — includes top-level "debug" key
        let debug_problem = Problem::from_error_debug(err.clone());
        let debug_json = serde_json::to_value(&debug_problem).unwrap();

        assert_eq!(
            debug_json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
                "title": "Not Found",
                "status": 404,
                "detail": "Resource not found",
                "context": {
                    "resource_type": "gts.cf.core.users.user.v1",
                    "resource_name": "user-123",
                    "description": "Resource not found"
                },
                "debug": {
                    "detail": "SELECT * FROM users WHERE id = $1 returned 0 rows",
                    "stack_entries": [
                        "cf_users::repo::find_by_id (src/repo.rs:42)"
                    ]
                }
            })
        );

        // Production mode — no "debug" key, identical to Problem::from(err)
        let prod_problem = Problem::from_error(err);
        let prod_json = serde_json::to_value(&prod_problem).unwrap();

        assert_eq!(
            prod_json,
            serde_json::json!({
                "type": "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
                "title": "Not Found",
                "status": 404,
                "detail": "Resource not found",
                "context": {
                    "resource_type": "gts.cf.core.users.user.v1",
                    "resource_name": "user-123",
                    "description": "Resource not found"
                }
            })
        );
    }

    // =========================================================================
    // GTS Schema tests — full JSON comparison for each context type
    // =========================================================================

    #[test]
    fn schema_retry_info_v1() {
        use gts::schema::GtsSchema;
        let schema = RetryInfoV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.retry_info.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["retry_after_seconds"],
                "properties": {
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "retry_after_seconds": {
                        "format": "uint64",
                        "minimum": 0,
                        "type": "integer"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_request_info_v1() {
        use gts::schema::GtsSchema;
        let schema = RequestInfoV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.request_info.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["request_id"],
                "properties": {
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "request_id": {
                        "type": "string"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_resource_info_v1() {
        use gts::schema::GtsSchema;
        let schema = ResourceInfoV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.resource_info.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["resource_type", "resource_name", "description"],
                "properties": {
                    "description": {
                        "type": "string"
                    },
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "resource_name": {
                        "type": "string"
                    },
                    "resource_type": {
                        "type": "string"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_error_info_v1() {
        use gts::schema::GtsSchema;
        let schema = ErrorInfoV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.error_info.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["reason", "domain", "metadata"],
                "properties": {
                    "domain": {
                        "type": "string"
                    },
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "metadata": {
                        "additionalProperties": {
                            "type": "string"
                        },
                        "type": "object"
                    },
                    "reason": {
                        "type": "string"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_field_violation_v1() {
        use gts::schema::GtsSchema;
        let schema = FieldViolationV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.field_violation.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["field", "description", "reason"],
                "properties": {
                    "description": {
                        "type": "string"
                    },
                    "field": {
                        "type": "string"
                    },
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "reason": {
                        "type": "string"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_debug_info_v1() {
        use gts::schema::GtsSchema;
        let schema = DebugInfoV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.debug_info.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["detail", "stack_entries"],
                "properties": {
                    "detail": {
                        "type": "string"
                    },
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "stack_entries": {
                        "items": {
                            "type": "string"
                        },
                        "type": "array"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_quota_violation_v1() {
        use gts::schema::GtsSchema;
        let schema = QuotaViolationV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.quota_violation.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["subject", "description"],
                "properties": {
                    "description": {
                        "type": "string"
                    },
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "subject": {
                        "type": "string"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_quota_failure_v1() {
        use gts::schema::GtsSchema;
        let schema = QuotaFailureV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.quota_failure.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["violations"],
                "properties": {
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "violations": {
                        "items": {
                            "$ref": "#/$defs/QuotaViolationV1"
                        },
                        "type": "array"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_precondition_violation_v1() {
        use gts::schema::GtsSchema;
        let schema = PreconditionViolationV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.precondition_violation.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["type", "subject", "description"],
                "properties": {
                    "description": {
                        "type": "string"
                    },
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "subject": {
                        "type": "string"
                    },
                    "type": {
                        "type": "string"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_precondition_failure_v1() {
        use gts::schema::GtsSchema;
        let schema = PreconditionFailureV1::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.precondition_failure.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "type": "object",
                "required": ["violations"],
                "properties": {
                    "gts_type": {
                        "description": "GTS schema identifier",
                        "format": "gts-schema-id",
                        "title": "GTS Schema ID",
                        "type": "string",
                        "x-gts-ref": "gts.*"
                    },
                    "violations": {
                        "items": {
                            "$ref": "#/$defs/PreconditionViolationV1"
                        },
                        "type": "array"
                    }
                }
            })
        );
    }

    #[test]
    fn schema_validation() {
        use gts::schema::GtsSchema;
        let schema = Validation::gts_schema_with_refs();
        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.validation.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "oneOf": [
                    {
                        "type": "object",
                        "properties": {
                            "field_violations": {
                                "type": "array",
                                "items": {
                                    "$ref": "gts://gts.cf.core.errors.field_violation.v1~"
                                }
                            }
                        },
                        "required": ["field_violations"]
                    },
                    {
                        "type": "object",
                        "properties": {
                            "format": {
                                "type": "string"
                            }
                        },
                        "required": ["format"]
                    },
                    {
                        "type": "object",
                        "properties": {
                            "constraint": {
                                "type": "string"
                            }
                        },
                        "required": ["constraint"]
                    }
                ]
            })
        );
    }

    #[test]
    fn schema_canonical_error() {
        use gts::schema::GtsSchema;
        let schema = CanonicalError::gts_schema_with_refs();

        let variant = |name: &str, ctx_ref: &str| {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "category": { "const": name },
                    "message": { "type": "string" },
                    "resource_type": { "type": "string" },
                    "context": { "$ref": ctx_ref }
                },
                "required": ["category", "message", "context"]
            })
        };

        assert_eq!(
            schema,
            serde_json::json!({
                "$id": "gts://gts.cf.core.errors.canonical_error.v1~",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "oneOf": [
                    variant("cancelled",           "gts://gts.cf.core.errors.request_info.v1~"),
                    variant("unknown",             "gts://gts.cf.core.errors.debug_info.v1~"),
                    variant("invalid_argument",    "gts://gts.cf.core.errors.validation.v1~"),
                    variant("deadline_exceeded",   "gts://gts.cf.core.errors.request_info.v1~"),
                    variant("not_found",           "gts://gts.cf.core.errors.resource_info.v1~"),
                    variant("already_exists",      "gts://gts.cf.core.errors.resource_info.v1~"),
                    variant("permission_denied",   "gts://gts.cf.core.errors.error_info.v1~"),
                    variant("resource_exhausted",  "gts://gts.cf.core.errors.quota_failure.v1~"),
                    variant("failed_precondition", "gts://gts.cf.core.errors.precondition_failure.v1~"),
                    variant("aborted",             "gts://gts.cf.core.errors.error_info.v1~"),
                    variant("out_of_range",        "gts://gts.cf.core.errors.validation.v1~"),
                    variant("unimplemented",       "gts://gts.cf.core.errors.error_info.v1~"),
                    variant("internal",            "gts://gts.cf.core.errors.debug_info.v1~"),
                    variant("unavailable",         "gts://gts.cf.core.errors.retry_info.v1~"),
                    variant("data_loss",           "gts://gts.cf.core.errors.resource_info.v1~"),
                    variant("unauthenticated",     "gts://gts.cf.core.errors.error_info.v1~")
                ]
            })
        );
    }

    // =========================================================================
    // GTS ID validation — ensures all IDs in the crate are valid GTS identifiers
    // =========================================================================

    #[test]
    fn validate_all_gts_ids() {
        use gts::schema::GtsSchema;

        // Validate all 16 category GTS type IDs
        let errors = vec![
            CanonicalError::cancelled(RequestInfo::new("r")),
            CanonicalError::unknown("e"),
            CanonicalError::invalid_argument(Validation::format("f")),
            CanonicalError::deadline_exceeded(RequestInfo::new("r")),
            CanonicalError::not_found(ResourceInfo::new("t", "n")),
            CanonicalError::already_exists(ResourceInfo::new("t", "n")),
            CanonicalError::permission_denied(ErrorInfo::new("R", "D")),
            CanonicalError::resource_exhausted(QuotaFailure::new(vec![])),
            CanonicalError::failed_precondition(PreconditionFailure::new(vec![])),
            CanonicalError::aborted(ErrorInfo::new("R", "D")),
            CanonicalError::out_of_range(Validation::constraint("c")),
            CanonicalError::unimplemented(ErrorInfo::new("R", "D")),
            CanonicalError::internal(DebugInfo::new("d")),
            CanonicalError::service_unavailable(RetryInfo::after_seconds(1)),
            CanonicalError::data_loss(ResourceInfo::new("t", "n")),
            CanonicalError::unauthenticated(ErrorInfo::new("R", "D")),
        ];
        for err in &errors {
            let id = err.gts_type();
            assert!(id.ends_with('~'), "GTS type ID must end with ~: {id}");
            gts_id::validate_gts_id(id, false)
                .unwrap_or_else(|e| panic!("Invalid GTS type ID '{id}': {e}"));
        }

        // Validate all 11 context type schema IDs
        let schema_ids = [
            RetryInfoV1::SCHEMA_ID,
            RequestInfoV1::SCHEMA_ID,
            ResourceInfoV1::SCHEMA_ID,
            ErrorInfoV1::SCHEMA_ID,
            FieldViolationV1::SCHEMA_ID,
            DebugInfoV1::SCHEMA_ID,
            QuotaViolationV1::SCHEMA_ID,
            QuotaFailureV1::SCHEMA_ID,
            PreconditionViolationV1::SCHEMA_ID,
            PreconditionFailureV1::SCHEMA_ID,
            Validation::SCHEMA_ID,
        ];
        for id in &schema_ids {
            assert!(id.ends_with('~'), "Schema ID must end with ~: {id}");
            gts_id::validate_gts_id(id, false)
                .unwrap_or_else(|e| panic!("Invalid schema ID '{id}': {e}"));
        }
    }

    // =========================================================================
    // Round-trip tests: CanonicalError → Problem → CanonicalError
    // =========================================================================

    /// Helper: assert round-trip preserves category, message, status, resource_type.
    fn assert_roundtrip(original: &CanonicalError) {
        let problem = Problem::from_error(original.clone());
        let reconstructed = CanonicalError::try_from(problem)
            .expect("round-trip should succeed");
        assert_eq!(original.gts_type(), reconstructed.gts_type(), "gts_type mismatch");
        assert_eq!(original.message(), reconstructed.message(), "message mismatch");
        assert_eq!(original.status_code(), reconstructed.status_code(), "status_code mismatch");
    }

    #[test]
    fn roundtrip_cancelled() {
        assert_roundtrip(&CanonicalError::cancelled(RequestInfo::new("req-1")));
    }

    #[test]
    fn roundtrip_unknown() {
        assert_roundtrip(&CanonicalError::unknown("something went wrong"));
    }

    #[test]
    fn roundtrip_invalid_argument() {
        assert_roundtrip(&CanonicalError::invalid_argument(
            Validation::fields([FieldViolation::new("email", "is required", "REQUIRED")]),
        ));
    }

    #[test]
    fn roundtrip_deadline_exceeded() {
        assert_roundtrip(&CanonicalError::deadline_exceeded(RequestInfo::new("req-2")));
    }

    #[test]
    fn roundtrip_not_found() {
        assert_roundtrip(&CanonicalError::not_found(
            ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"),
        ));
    }

    #[test]
    fn roundtrip_already_exists() {
        assert_roundtrip(&CanonicalError::already_exists(
            ResourceInfo::new("gts.cf.core.users.user.v1", "bob@example.com"),
        ));
    }

    #[test]
    fn roundtrip_permission_denied() {
        assert_roundtrip(&CanonicalError::permission_denied(
            ErrorInfo::new("CROSS_TENANT_ACCESS", "auth.cyberfabric.io"),
        ));
    }

    #[test]
    fn roundtrip_resource_exhausted() {
        assert_roundtrip(&CanonicalError::resource_exhausted(
            QuotaFailure::new([QuotaViolation::new("requests", "Rate limit exceeded")]),
        ));
    }

    #[test]
    fn roundtrip_failed_precondition() {
        assert_roundtrip(&CanonicalError::failed_precondition(
            PreconditionFailure::new([PreconditionViolation::new(
                "STATE", "document.status", "Document is already published",
            )]),
        ));
    }

    #[test]
    fn roundtrip_aborted() {
        assert_roundtrip(&CanonicalError::aborted(
            ErrorInfo::new("OPTIMISTIC_LOCK_FAILURE", "cf.oagw")
                .with_metadata("expected_version", "3"),
        ));
    }

    #[test]
    fn roundtrip_out_of_range() {
        assert_roundtrip(&CanonicalError::out_of_range(
            Validation::constraint("Page 50 is beyond the last page (12)"),
        ));
    }

    #[test]
    fn roundtrip_unimplemented() {
        assert_roundtrip(&CanonicalError::unimplemented(
            ErrorInfo::new("GRPC_STREAMING", "cf.core"),
        ));
    }

    #[test]
    fn roundtrip_internal() {
        assert_roundtrip(&CanonicalError::internal(
            DebugInfo::new("null pointer in user service"),
        ));
    }

    #[test]
    fn roundtrip_service_unavailable() {
        assert_roundtrip(&CanonicalError::service_unavailable(
            RetryInfo::after_seconds(30),
        ));
    }

    #[test]
    fn roundtrip_data_loss() {
        assert_roundtrip(&CanonicalError::data_loss(
            ResourceInfo::new("gts.cf.core.data.backup.v1", "backup-42"),
        ));
    }

    #[test]
    fn roundtrip_unauthenticated() {
        assert_roundtrip(&CanonicalError::unauthenticated(
            ErrorInfo::new("TOKEN_EXPIRED", "auth.cyberfabric.io"),
        ));
    }

    // =========================================================================
    // Context preservation tests
    // =========================================================================

    #[test]
    fn roundtrip_preserves_resource_info_fields() {
        let original = CanonicalError::not_found(
            ResourceInfo::new("gts.cf.core.users.user.v1", "user-123")
                .with_description("User not found in database"),
        );
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        // Verify context fields via serialization
        let ctx_json = match &reconstructed {
            CanonicalError::NotFound { ctx, .. } => serde_json::to_value(ctx).unwrap(),
            _ => panic!("expected NotFound"),
        };
        assert_eq!(ctx_json["resource_name"], "user-123");
        assert_eq!(ctx_json["description"], "User not found in database");
    }

    #[test]
    fn roundtrip_preserves_error_info_with_metadata() {
        let original = CanonicalError::permission_denied(
            ErrorInfo::new("CROSS_TENANT_ACCESS", "auth.cyberfabric.io")
                .with_metadata("tenant_id", "t-1")
                .with_metadata("caller", "user-99"),
        );
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        let ctx_json = match &reconstructed {
            CanonicalError::PermissionDenied { ctx, .. } => serde_json::to_value(ctx).unwrap(),
            _ => panic!("expected PermissionDenied"),
        };
        assert_eq!(ctx_json["reason"], "CROSS_TENANT_ACCESS");
        assert_eq!(ctx_json["domain"], "auth.cyberfabric.io");
        assert_eq!(ctx_json["metadata"]["tenant_id"], "t-1");
        assert_eq!(ctx_json["metadata"]["caller"], "user-99");
    }

    #[test]
    fn roundtrip_preserves_validation_field_violations() {
        let original = CanonicalError::invalid_argument(Validation::fields([
            FieldViolation::new("email", "is required", "REQUIRED"),
            FieldViolation::new("name", "must be at least 2 characters", "TOO_SHORT"),
        ]));
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        let ctx_json = match &reconstructed {
            CanonicalError::InvalidArgument { ctx, .. } => serde_json::to_value(ctx).unwrap(),
            _ => panic!("expected InvalidArgument"),
        };
        let violations = ctx_json["field_violations"].as_array().unwrap();
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0]["field"], "email");
        assert_eq!(violations[0]["reason"], "REQUIRED");
        assert_eq!(violations[1]["field"], "name");
        assert_eq!(violations[1]["reason"], "TOO_SHORT");
    }

    #[test]
    fn roundtrip_preserves_quota_failure() {
        let original = CanonicalError::resource_exhausted(QuotaFailure::new([
            QuotaViolation::new("requests", "Rate limit exceeded"),
        ]));
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        let ctx_json = match &reconstructed {
            CanonicalError::ResourceExhausted { ctx, .. } => serde_json::to_value(ctx).unwrap(),
            _ => panic!("expected ResourceExhausted"),
        };
        let violations = ctx_json["violations"].as_array().unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0]["subject"], "requests");
    }

    #[test]
    fn roundtrip_preserves_precondition_failure() {
        let original = CanonicalError::failed_precondition(PreconditionFailure::new([
            PreconditionViolation::new("STATE", "tenant.users", "Remove all active users"),
        ]));
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        let ctx_json = match &reconstructed {
            CanonicalError::FailedPrecondition { ctx, .. } => serde_json::to_value(ctx).unwrap(),
            _ => panic!("expected FailedPrecondition"),
        };
        let violations = ctx_json["violations"].as_array().unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0]["subject"], "tenant.users");
    }

    // =========================================================================
    // resource_type round-trip tests
    // =========================================================================

    #[test]
    fn roundtrip_with_resource_type_set() {
        let original = CanonicalError::not_found(
            ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"),
        )
        .with_resource_type("gts.cf.core.users.user.v1");
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        assert_eq!(
            reconstructed.resource_type(),
            Some("gts.cf.core.users.user.v1")
        );
    }

    #[test]
    fn roundtrip_without_resource_type() {
        // Internal uses DebugInfo — no resource_type field in the context struct
        let original = CanonicalError::internal(DebugInfo::new("invariant violated"));
        assert_eq!(original.resource_type(), None);
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        assert_eq!(reconstructed.resource_type(), None);
    }

    // =========================================================================
    // debug_info round-trip tests
    // =========================================================================

    #[test]
    fn roundtrip_debug_info_preserved_via_from_error_debug() {
        let original = CanonicalError::not_found(
            ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"),
        )
        .with_debug_info(
            DebugInfo::new("SELECT * FROM users WHERE id = $1 returned 0 rows")
                .with_stack(vec!["repo.rs:42".into()]),
        );
        let problem = Problem::from_error_debug(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        let debug = reconstructed.debug_info().expect("debug_info should be present");
        assert_eq!(debug.detail, "SELECT * FROM users WHERE id = $1 returned 0 rows");
        assert_eq!(debug.stack_entries, vec!["repo.rs:42"]);
    }

    #[test]
    fn roundtrip_debug_info_stripped_via_from_error() {
        let original = CanonicalError::not_found(
            ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"),
        )
        .with_debug_info(DebugInfo::new("SQL query details"));
        // Production mode strips debug info
        let problem = Problem::from_error(original);
        let reconstructed = CanonicalError::try_from(problem).unwrap();
        assert!(reconstructed.debug_info().is_none(), "debug_info should be stripped in production mode");
    }

    // =========================================================================
    // Error case tests
    // =========================================================================

    #[test]
    fn try_from_malformed_type_uri() {
        let problem = Problem {
            problem_type: "not-a-gts-uri".to_string(),
            title: "Unknown".to_string(),
            status: 500,
            detail: "test".to_string(),
            instance: None,
            trace_id: None,
            context: serde_json::json!({}),
            debug: None,
        };
        let err = CanonicalError::try_from(problem).unwrap_err();
        assert!(matches!(err, ProblemConversionError::InvalidType(_)));
    }

    #[test]
    fn try_from_unknown_category() {
        let problem = Problem {
            problem_type: "gts.cf.core.errors.err.v1~cf.core.errors.nonexistent.v1~".to_string(),
            title: "Unknown".to_string(),
            status: 500,
            detail: "test".to_string(),
            instance: None,
            trace_id: None,
            context: serde_json::json!({}),
            debug: None,
        };
        let err = CanonicalError::try_from(problem).unwrap_err();
        match err {
            ProblemConversionError::UnknownCategory(c) => assert_eq!(c, "nonexistent"),
            other => panic!("expected UnknownCategory, got: {other:?}"),
        }
    }

    #[test]
    fn try_from_wrong_context_shape() {
        let problem = Problem {
            problem_type: "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~".to_string(),
            title: "Not Found".to_string(),
            status: 404,
            detail: "Resource not found".to_string(),
            instance: None,
            trace_id: None,
            context: serde_json::json!({"unexpected": "shape"}),
            debug: None,
        };
        let err = CanonicalError::try_from(problem).unwrap_err();
        assert!(matches!(err, ProblemConversionError::ContextDeserializationFailed { .. }));
    }

    // =========================================================================
    // SDK consumer pattern: JSON string → Problem → CanonicalError
    // =========================================================================

    #[test]
    fn sdk_consumer_pattern_json_to_canonical_error() {
        let json = r#"{
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
            "title": "Not Found",
            "status": 404,
            "detail": "Resource not found",
            "context": {
                "resource_type": "gts.cf.core.users.user.v1",
                "resource_name": "user-123",
                "description": "Resource not found"
            }
        }"#;

        // Step 1: JSON → Problem
        let problem: Problem = serde_json::from_str(json).unwrap();
        assert_eq!(problem.status, 404);

        // Step 2: Problem → CanonicalError
        let err = CanonicalError::try_from(problem).unwrap();
        assert_eq!(err.gts_type(), "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~");
        assert_eq!(err.message(), "Resource not found");
        assert_eq!(err.status_code(), 404);
        assert_eq!(err.resource_type(), Some("gts.cf.core.users.user.v1"));

        // Step 3: SDK consumer matches on status_code
        match err.status_code() {
            404 => {} // not_found — expected
            403 => panic!("should not be permission_denied"),
            _ => panic!("unexpected status"),
        }
    }

    // =========================================================================
    // Problem deserialization tests
    // =========================================================================

    #[test]
    fn problem_deserialize_roundtrip() {
        let original_err = CanonicalError::not_found(
            ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"),
        );
        let original_problem = Problem::from_error(original_err);
        let json = serde_json::to_string(&original_problem).unwrap();
        let deserialized: Problem = serde_json::from_str(&json).unwrap();
        assert_eq!(original_problem.problem_type, deserialized.problem_type);
        assert_eq!(original_problem.title, deserialized.title);
        assert_eq!(original_problem.status, deserialized.status);
        assert_eq!(original_problem.detail, deserialized.detail);
        assert_eq!(original_problem.context, deserialized.context);
        assert_eq!(original_problem.instance, deserialized.instance);
        assert_eq!(original_problem.trace_id, deserialized.trace_id);
    }

    #[test]
    fn problem_deserialize_missing_optional_fields() {
        let json = r#"{
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~",
            "title": "Internal",
            "status": 500,
            "detail": "An internal error occurred",
            "context": {"detail": "test", "stack_entries": []}
        }"#;
        let problem: Problem = serde_json::from_str(json).unwrap();
        assert!(problem.instance.is_none());
        assert!(problem.trace_id.is_none());
        assert!(problem.debug.is_none());
    }
}
