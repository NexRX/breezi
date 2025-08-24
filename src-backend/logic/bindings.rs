use crate::{model::Config, routes::Routes};
use schemars::schema_for;
use std::{fs::write, path::Path};
use tracing::info;

pub fn generate_all_bindings(config: &Config) -> crate::Result {
    if config.bindings_generate {
        info!("Generating bindings");
        Routes::gen_bindings(&config.bindings_dir);
        gen_validations(&config.bindings_dir)?;
    }

    Ok(())
}

fn gen_validations(bindings_dir: &Path) -> crate::Result {
    let schema = schema_for!(crate::model::UserAll);

    write(bindings_dir.join("user.schema.json"), serde_json::to_string_pretty(&schema)?)?;

    Ok(())
}
