use std::sync::Arc;

use axum::{
    body::Body,
    routing::{delete, get, post},
    Router,
};

use crate::state::State;

pub mod delete_recipe_handler;
pub mod insert_recipe_handler;
pub mod query_recipe_handler;
pub mod update_recipe_handler;

pub fn router(state: State) -> Router<(), Body> {
    let storage = ImagesSqliteDS::new(state.pool());
    let batch_delete_recipe_service = Arc::new(BatchDeleteImage::new(storage))
        as batch_delete_recipe_handler::DynBatchDeleteImageService;
    let storage = ImagesSqliteDS::new(state.pool());
    let upload_images_service = Arc::new(UploadImages::new(
        storage,
        state.images_base_path().to_string(),
    )) as upload_images_handler::DynUploadImagesService;
    let storage = ImagesSqliteDS::new(state.pool());
    let delete_recipe_service = Arc::new(DeleteImage::new(storage)) as DynDeleteImagesService;
    let storage = ImagesSqliteDS::new(state.pool());
    let query_recipe_service = Arc::new(QueryImage::new(storage)) as DynQueryImageService;
    let storage = ImagesSqliteDS::new(state.pool());
    let batch_query_recipe_service =
        Arc::new(BatchQueryImage::new(storage)) as DynBatchQueryImageService;
    let images_routes = Router::new()
        .route("/", post(upload_images_handler::upload_images_handler))
        .with_state(upload_images_service)
        .route(
            "/batch_delete",
            post(batch_delete_recipe_handler::batch_delete_recipe_handler),
        )
        .with_state(batch_delete_recipe_service)
        .route(
            "/:identifier",
            get(query_recipe_handler::query_recipe_handler),
        )
        .with_state(query_recipe_service.clone())
        .route(
            "/content/:identifier",
            get(get_recipe_content_handler::get_recipe_content_handler),
        )
        .with_state(query_recipe_service)
        .route(
            "/",
            get(batch_query_recipe_handler::batch_query_recipe_handler),
        )
        .with_state(batch_query_recipe_service)
        .route(
            "/:identifier",
            delete(delete_recipe_handler::delete_recipe_handler),
        )
        .with_state(delete_recipe_service);
    let images_router = Router::new().nest("/images", images_routes);
    Router::new().nest("/api/v1", images_router)
}
