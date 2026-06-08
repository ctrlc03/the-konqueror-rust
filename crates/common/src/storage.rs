use async_trait::async_trait;
use uuid::Uuid;

use crate::error::Result;
use crate::types::{Implant, Listener, Operator, Task, TaskResult};

/// Storage backend trait. Implement this for SQLite, Postgres, in-memory, etc.
///
/// All methods are async to support both real databases and test mocks.
/// Implementations must be Send + Sync to be shared across async tasks.
#[async_trait]
pub trait Storage: Send + Sync {
    // --- Operators ---

    async fn create_operator(&self, operator: &Operator) -> Result<()>;

    async fn get_operator_by_credentials(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<Operator>;

    async fn get_operator_by_api_key(&self, api_key: &str) -> Result<Operator>;

    async fn set_operator_logged_in(&self, operator_id: Uuid, logged_in: bool) -> Result<()>;

    // --- Listeners ---

    async fn create_listener(&self, listener: &Listener) -> Result<()>;

    async fn get_listener(&self, id: Uuid) -> Result<Listener>;

    async fn list_active_listeners(&self) -> Result<Vec<Listener>>;

    async fn delete_listener(&self, id: Uuid) -> Result<()>;

    // --- Implants ---

    async fn create_implant(&self, implant: &Implant) -> Result<()>;

    async fn get_implant(&self, id: Uuid) -> Result<Implant>;

    async fn get_implant_by_listener(&self, listener_id: Uuid) -> Result<Option<Implant>>;

    async fn list_active_implants(&self) -> Result<Vec<Implant>>;

    async fn update_implant_field(&self, implant_id: Uuid, field: &str, value: &str) -> Result<()>;

    async fn update_implant_last_seen(&self, implant_id: Uuid) -> Result<()>;

    async fn delete_implant(&self, id: Uuid) -> Result<()>;

    // --- Tasks ---

    async fn create_task(&self, task: &Task) -> Result<()>;

    async fn get_task(&self, id: Uuid) -> Result<Task>;

    async fn get_pending_tasks_for_listener(&self, listener_id: Uuid) -> Result<Vec<Task>>;

    async fn complete_task(&self, task_id: Uuid, result: &TaskResult) -> Result<()>;
}
