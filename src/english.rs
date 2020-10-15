use goose::prelude::*;

use crate::common;

use rand::seq::SliceRandom;

/// Load the front page in English and all static assets found on the page.
pub async fn front_page_en(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/").await?;
    common::validate_and_load_static_assets(user, goose, "Home").await?;

    Ok(())
}

/// Load recipe listing in English and all static assets found on the page.
pub async fn recipe_listing_en(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/en/recipes/").await?;
    common::validate_and_load_static_assets(user, goose, "Recipes").await?;

    Ok(())
}

/// Load a random recipe in English and all static assets found on the page.
pub async fn recipe_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(recipe.unwrap().url_en).await?;
    common::validate_and_load_static_assets(user, goose, recipe.unwrap().title_en).await?;

    Ok(())
}

/// Load article listing in English and all static assets found on the page.
pub async fn article_listing_en(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/en/articles/").await?;
    common::validate_and_load_static_assets(user, goose, "Articles").await?;

    Ok(())
}

/// Load a random article in English and all static assets found on the page.
pub async fn article_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(article.unwrap().url_en).await?;
    common::validate_and_load_static_assets(user, goose, article.unwrap().title_en).await?;

    Ok(())
}

/// Load a random basic page in English and all static assets found on the page.
pub async fn basic_page_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(page.unwrap().url_en).await?;
    common::validate_and_load_static_assets(user, goose, page.unwrap().title_en).await?;

    Ok(())
}

/// Load a random node by nid in English and all static assets found on the page.
pub async fn page_by_nid(user: &GooseUser) -> GooseTaskResult {
    // Randomly select a content type.
    let content_types = vec![
        common::ContentType::Article,
        common::ContentType::BasicPage,
        common::ContentType::Recipe,
    ];
    let content_type = content_types.choose(&mut rand::thread_rng());
    // Then randomly select a node of this content type.
    let nodes = common::get_nodes(&content_type.unwrap());
    let page = nodes.choose(&mut rand::thread_rng());
    // Load the page by nid instead of by URL.
    let goose = user
        .get(&("/node/".to_string() + &page.unwrap().nid.to_string()))
        .await?;
    common::validate_and_load_static_assets(user, goose, page.unwrap().title_en).await?;

    Ok(())
}
