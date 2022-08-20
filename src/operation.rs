use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operation {
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationReply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Operation {
    pub fn new(operation: impl ToString, id: Option<i64>, request: Option<Value>) -> Self {
        let operation = operation.to_string();
        Operation {
            operation,
            id,
            request,
        }
    }

    pub fn make_result(&self, results: Option<Value>) -> OperationReply {
        OperationReply::new_result(self.id, results)
    }

    pub fn make_error(&self, error: impl ToString) -> OperationReply {
        OperationReply::new_error(self.id, error.to_string())
    }
}

impl OperationReply {
    pub fn new_result(id: Option<i64>, results: Option<Value>) -> Self {
        Self {
            id,
            success: true,
            results,
            error: None,
        }
    }

    pub fn new_error(id: Option<i64>, error: impl ToString) -> Self {
        Self {
            id,
            success: false,
            results: None,
            error: Some(error.to_string()),
        }
    }
}
