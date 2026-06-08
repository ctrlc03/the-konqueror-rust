use std::future::Future;

use uuid::Uuid;

use crate::error::Result;
use crate::types::{Implant, Listener, Operator, Task, TaskResult};

/// Storage backend trait. Implement this for SQLite, Postgres, in-memory, etc.
///
/// All methods are async to support both real databases and test mocks.
/// Implementations must be Send + Sync to be shared across async tasks.
pub trait Storage: Send + Sync {
    // --- Operators ---

    fn create_operator(
        &self,
        operator: &Operator,
    ) -> impl Future<Output = Result<()>> + Send;

    fn get_operator_by_credentials(
        &self,
        username: &str,
        password_hash: &str,
    ) -> impl Future<Output = Result<Operator>> + Send;

    fn get_operator_by_api_key(
        &self,
        api_key: &str,
    ) -> impl Future<Output = Result<Operator>> + Send;

    fn set_operator_logged_in(
        &self,
        operator_id: Uuid,
        logged_in: bool,
    ) -> impl Future<Output = Result<()>> + Send;

    // --- Listeners ---

    fn create_listener(&self, listener: &Listener) -> impl Future<Output = Result<()>> + Send;

    fn get_listener(&self, id: Uuid) -> impl Future<Output = Result<Listener>> + Send;

    fn list_active_listeners(&self) -> impl Future<Output = Result<Vec<Listener>>> + Send;

    fn delete_listener(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    // --- Implants ---

    fn create_implant(&self, implant: &Implant) -> impl Future<Output = Result<()>> + Send;

    fn get_implant(&self, id: Uuid) -> impl Future<Output = Result<Implant>> + Send;

    fn get_implant_by_listener(
        &self,
        listener_id: Uuid,
    ) -> impl Future<Output = Result<Option<Implant>>> + Send;

    fn list_active_implants(&self) -> impl Future<Output = Result<Vec<Implant>>> + Send;

    fn update_implant_field(
        &self,
        implant_id: Uuid,
        field: &str,
        value: &str,
    ) -> impl Future<Output = Result<()>> + Send;

    fn update_implant_last_seen(&self, implant_id: Uuid)
        -> impl Future<Output = Result<()>> + Send;

    fn delete_implant(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    // --- Tasks ---

    fn create_task(&self, task: &Task) -> impl Future<Output = Result<()>> + Send;

    fn get_task(&self, id: Uuid) -> impl Future<Output = Result<Task>> + Send;

    fn get_pending_tasks_for_listener(
        &self,
        listener_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Task>>> + Send;

    fn complete_task(
        &self,
        task_id: Uuid,
        result: &TaskResult,
    ) -> impl Future<Output = Result<()>> + Send;
}
