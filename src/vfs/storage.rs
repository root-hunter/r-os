use idb::{Database, DatabaseEvent, Error, Factory, IndexParams, KeyPath, ObjectStoreParams};

pub async fn init_storage() -> Result<Database, Error> {
    let factory = Factory::new()?;

 let mut open_request = factory.open("vfs", Some(1)).unwrap();

    // Add an upgrade handler for database
    open_request.on_upgrade_needed(|event| {
        // Get database instance from event
        let database = event.database().unwrap();

        // Prepare object store params
        let mut store_params = ObjectStoreParams::new();
        store_params.auto_increment(false);
        store_params.key_path(Some(KeyPath::new_single("abs_path")));

        let store = database
            .create_object_store("vol_0", store_params)
            .unwrap();

        let mut index_params = IndexParams::new();
        index_params.unique(true);

        store
            .create_index("abs_path", KeyPath::new_single("abs_path"), Some(index_params))
            .unwrap();
    });

    open_request.await
}