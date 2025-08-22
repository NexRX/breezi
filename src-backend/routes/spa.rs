use axum_embed::{FallbackBehavior, ServeEmbed};
use rust_embed::RustEmbed;
use tracing::trace;

#[derive(RustEmbed, Clone)]
#[folder = "dist/"]
pub struct Spa;

impl Spa {
    pub(crate) fn service(fallback: FallbackBehavior) -> ServeEmbed<Spa> {
        let service = match fallback {
            FallbackBehavior::NotFound => ServeEmbed::<Spa>::with_parameters(None, fallback, None),
            _ => ServeEmbed::<Spa>::with_parameters(
                Some("/".to_string()),
                fallback,
                Some("index.html".to_string()),
            ),
        };

        for file in Spa::iter() {
            trace!("Router's embedded asset: /{}", file.as_ref());
        }

        service
    }
}
