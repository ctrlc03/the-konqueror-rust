use konqueror_common::error::{KonquerorError, Result};
use konqueror_common::storage::Storage;
use konqueror_common::types::{ComponentStatus, Implant, Listener, Operator, Task, TaskResult};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct InMemoryStorage {
    operators: RwLock<HashMap<Uuid, Operator>>,
    listeners: RwLock<HashMap<Uuid, Listener>>,
    implants: RwLock<HashMap<Uuid, Implant>>,
    tasks: RwLock<HashMap<Uuid, Task>>,
}

impl Storage for InMemoryStorage {
    fn create_operator(&self, operator: &Operator) -> impl Future<Output = Result<()>> + Send {
        let operators = &self.operators;

        async move {
            let mut store = operators.write().await;
            if store.values().any(|op| op.username == operator.username) {
                return Err(KonquerorError::AlreadyExists(operator.username.clone()));
            }
            store.insert(operator.id, operator.clone());
            Ok(())
        }
    }

    fn get_operator_by_credentials(
        &self,
        username: &str,
        password_hash: &str,
    ) -> impl Future<Output = Result<Operator>> + Send {
        let operators = &self.operators;

        async move {
            let store = operators.read().await;
            let operator = store
                .values()
                .find(|op| op.username == username && op.password_hash == password_hash);
            match operator {
                Some(op) => Ok(op.clone()),
                None => Err(KonquerorError::NotFound(username.to_string())),
            }
        }
    }

    fn get_operator_by_api_key(
        &self,
        api_key: &str,
    ) -> impl Future<Output = Result<Operator>> + Send {
        let operators = &self.operators;

        async move {
            let store = operators.read().await;
            let operator = store
                .values()
                .find(|op| op.api_key == Some(api_key.to_string()));

            match operator {
                Some(op) => Ok(op.clone()),
                None => Err(KonquerorError::NotFound(api_key.to_string())),
            }
        }
    }

    fn set_operator_logged_in(
        &self,
        operator_id: Uuid,
        logged_in: bool,
    ) -> impl Future<Output = Result<()>> + Send {
        let operators = &self.operators;

        async move {
            let mut store = operators.write().await;
            let operator = store
                .get_mut(&operator_id)
                .ok_or_else(|| KonquerorError::NotFound(operator_id.to_string()))?;

            operator.logged_in = logged_in;
            Ok(())
        }
    }

    fn create_listener(&self, listener: &Listener) -> impl Future<Output = Result<()>> + Send {
        let listeners = &self.listeners;

        async move {
            let mut store = listeners.write().await;

            store.insert(listener.id, listener.clone());
            Ok(())
        }
    }

    fn get_listener(&self, id: Uuid) -> impl Future<Output = Result<Listener>> + Send {
        let listeners = &self.listeners;

        async move {
            let store = listeners.read().await;

            let listener = store
                .get(&id)
                .ok_or_else(|| KonquerorError::NotFound(id.to_string()))?;
            Ok(listener.clone())
        }
    }

    fn list_active_listeners(&self) -> impl Future<Output = Result<Vec<Listener>>> + Send {
        let listeners = &self.listeners;

        async move {
            let store = listeners.read().await;

            let listeners: Vec<Listener> = store
                .values()
                .filter(|l| l.status == ComponentStatus::Active)
                .cloned()
                .collect();

            Ok(listeners)
        }
    }

    fn delete_listener(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send {
        let listeners = &self.listeners;

        async move {
            let mut store = listeners.write().await;
            store.remove(&id);

            Ok(())
        }
    }

    fn create_implant(&self, implant: &Implant) -> impl Future<Output = Result<()>> + Send {
        let implants = &self.implants;

        async move {
            let mut store = implants.write().await;

            store.insert(implant.id, implant.clone());
            Ok(())
        }
    }

    fn get_implant(&self, id: Uuid) -> impl Future<Output = Result<Implant>> + Send {
        let implants = &self.implants;

        async move {
            let store = implants.write().await;

            let implant = store
                .get(&id)
                .ok_or_else(|| KonquerorError::NotFound(id.to_string()))?;
            Ok(implant.clone())
        }
    }

    fn get_implant_by_listener(
        &self,
        listener_id: Uuid,
    ) -> impl Future<Output = Result<Option<Implant>>> + Send {
        let implants = &self.implants;

        async move {
            let store = implants.read().await;
            let implant = store
                .values()
                .find(|ip| ip.listener_id == listener_id)
                .cloned();

            Ok(implant)
        }
    }

    fn list_active_implants(&self) -> impl Future<Output = Result<Vec<Implant>>> + Send {
        let implants = &self.implants;

        async move {
            let store = implants.read().await;
            let implants = store
                .values()
                .filter(|ip| ip.status == ComponentStatus::Active)
                .cloned()
                .collect();

            Ok(implants)
        }
    }

    fn update_implant_field(
        &self,
        implant_id: Uuid,
        field: &str,
        value: &str,
    ) -> impl Future<Output = Result<()>> + Send {
        let implants = &self.implants;

        async move {
            let mut store = implants.write().await;

            let implant = store
                .get_mut(&implant_id)
                .ok_or(KonquerorError::NotFound(implant_id.to_string()))?;

            match field {
                "status" => {
                    implant.status = ComponentStatus::try_from(value)?;
                }
                "os" => {
                    implant.os = value.to_string();
                }
                "arch" => {
                    implant.arch = value.to_string();
                }
                "hostname" => {
                    implant.hostname = value.to_string();
                }
                "username" => {
                    implant.username = value.to_string();
                }
                "user_id" => {
                    implant.user_id = value.to_string();
                }
                "pid" => {
                    implant.pid = value.parse::<u32>().map_err(|_| {
                        KonquerorError::InvalidInput(format!("invalid pid: {value}"))
                    })?;
                }
                "ppid" => {
                    implant.ppid = value.parse::<u32>().map_err(|_| {
                        KonquerorError::InvalidInput(format!("invalid ppid: {value}"))
                    })?;
                }
                "cwd" => {
                    implant.cwd = value.to_string();
                }
                "sleep_time_secs" => {
                    implant.sleep_time_secs = value.parse::<u64>().map_err(|_| {
                        KonquerorError::InvalidInput(format!("invalid sleep_time_secs: {value}"))
                    })?;
                }
                "jitter" => {
                    implant.jitter = value.parse::<u32>().map_err(|_| {
                        KonquerorError::InvalidInput(format!("invalid jitter: {value}"))
                    })?;
                }
                "max_retry" => {
                    implant.max_retry = value.parse::<u32>().map_err(|_| {
                        KonquerorError::InvalidInput(format!("invalid max_retry: {value}"))
                    })?;
                }
                "failed_checkins" => {
                    implant.failed_checkins = value.parse::<u32>().map_err(|_| {
                        KonquerorError::InvalidInput(format!("invalid failed_checkins: {value}"))
                    })?;
                }
                _ => {
                    return Err(KonquerorError::InvalidInput(format!("{value}")));
                }
            }

            Ok(())
        }
    }

    fn update_implant_last_seen(
        &self,
        implant_id: Uuid,
    ) -> impl Future<Output = Result<()>> + Send {
        let implants = &self.implants;

        async move {
            let mut store = implants.write().await;

            let implant = store
                .get_mut(&implant_id)
                .ok_or(KonquerorError::NotFound(implant_id.to_string()))?;

            implant.last_seen = chrono::Utc::now();

            Ok(())
        }
    }

    fn delete_implant(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send {
        let implants = &self.implants;

        async move {
            let mut store = implants.write().await;

            store
                .remove(&id)
                .ok_or(KonquerorError::NotFound(id.to_string()))?;

            Ok(())
        }
    }

    fn create_task(&self, task: &Task) -> impl Future<Output = Result<()>> + Send {
        let tasks = &self.tasks;

        async move {
            let mut store = tasks.write().await;

            store.insert(task.id, task.clone());

            Ok(())
        }
    }

    fn get_task(&self, id: Uuid) -> impl Future<Output = Result<Task>> + Send {
        let tasks = &self.tasks;

        async move {
            let store = tasks.read().await;

            let task = store
                .get(&id)
                .ok_or(KonquerorError::NotFound(id.to_string()))?;
            Ok(task.clone())
        }
    }

    fn get_pending_tasks_for_listener(
        &self,
        listener_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Task>>> + Send {
        let tasks = &self.tasks;

        async move {
            let store = tasks.read().await;

            let tasks: Vec<Task> = store
                .values()
                .filter(|t| t.listener_id == listener_id)
                .cloned()
                .collect();

            Ok(tasks)
        }
    }

    fn complete_task(
        &self,
        task_id: Uuid,
        result: &TaskResult,
    ) -> impl Future<Output = Result<()>> + Send {
        let tasks = &self.tasks;

        async move {
            let mut store = tasks.write().await;
            let task = store
                .get_mut(&task_id)
                .ok_or(KonquerorError::NotFound(task_id.to_string()))?;
            task.result = Some(result.clone());

            Ok(())
        }
    }
}
