use std::sync::Arc;

use konqueror_common::storage::Storage;

struct AppState {
    storage: Arc<dyn Storage>,
}
