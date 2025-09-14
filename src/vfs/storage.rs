use idb::{Database, DatabaseEvent, Error, Factory, KeyPath, ObjectStoreParams};

pub async fn init_storage() -> Result<Database, Error> {
    let factory = Factory::new()?;

    let mut open_request = factory.open("vfs", Some(1)).unwrap();

    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        let mut store_params = ObjectStoreParams::new();
        store_params.key_path(Some(KeyPath::new_single("full_path")));
        store_params.auto_increment(false);

        database
            .create_object_store("vol_0", store_params)
            .unwrap();
    });

    open_request.await
}