use goose::prelude::*;

use crate::common;

use rand::seq::SliceRandom;
use log::info;

/// Load the front page in Spanish and all static assets found on the page.
pub async fn front_page_es(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/es").await?;
    common::validate_and_load_static_assets(user, goose, "Inicio").await?;

    Ok(())
}

/// Load article listing in Spanish and all static assets found on the page.
pub async fn recipe_listing_es(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/es/recipes/").await?;
    common::validate_and_load_static_assets(user, goose, "Recetas").await?;

    Ok(())
}

/// Load a random recipe in Spanish and all static assets found on the page.
pub async fn recipe_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(recipe.unwrap().url_es).await?;
    common::validate_and_load_static_assets(user, goose, recipe.unwrap().title_es).await?;

    Ok(())
}

/// Load article listing in Spanish and all static assets found on the page.
pub async fn article_listing_es(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/es/articles/").await?;
    common::validate_and_load_static_assets(user, goose, "Artículos").await?;

    Ok(())
}

/// Load a random article in Spanish and all static assets found on the page.
pub async fn article_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(article.unwrap().url_es).await?;
    common::validate_and_load_static_assets(user, goose, article.unwrap().title_es).await?;

    Ok(())
}

/// Load a basic page in Spanish and all static assets found on the page.
pub async fn basic_page_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = common::get_nodes(&common::ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(page.unwrap().url_es).await?;
    common::validate_and_load_static_assets(user, goose, page.unwrap().title_es).await?;

    Ok(())
}

/// Anonymously load the contact form in Spanish and POST feedback.
pub async fn anonymous_contact_form_es(user: &GooseUser) -> GooseTaskResult {
    let contact_form_url = "/es/contact";
    let mut goose = user.get(contact_form_url).await?;

    // We can't invoke common::validate_and_load_static_assets as while it's important
    // to validate the page and load static elements, we then need to extra form elements
    // from the HTML of the page. So we duplicate some of the logic, enhancing it for form
    // processing.
    let contact_form;
    match goose.response {
        Ok(response) => {
            // Copy the headers so we have them for logging if there are errors.
            let headers = &response.headers().clone();
            match response.text().await {
                Ok(html) => {
                    // Be sure we've properly loaded the Contact form.
                    let title = "Comentarios sobre el sitio web";
                    if !common::valid_title(&html, title) {
                        return user.set_failure(
                            &format!("{}: title not found: {}", goose.request.url, title),
                            &mut goose.request,
                            Some(&headers),
                            Some(&html),
                        );
                    }

                    // Load all static elements on the page, as a real user would.
                    common::load_static_elements(user, &html).await;

                    // Scrape the HTML to get the values needed in order to POST to the
                    // contact form.
                    let form_build_id = common::get_form_value(&html, "form_build_id");
                    if form_build_id.is_none() {
                        return user.set_failure(
                            &format!("{}: no form_build_id on page", goose.request.url),
                            &mut goose.request,
                            Some(&headers),
                            Some(&html),
                        );
                    }

                    // Build contact form parameters.
                    let params = [
                        ("name", "@TODO: name"),
                        ("mail", "nobody@example.com"),
                        ("subject[0][value]", "@TODO: subject"),
                        ("message[0][value]", "@TODO: message"),
                        ("form_build_id", &form_build_id.unwrap()),
                        ("form_id", "contact_message_feedback_form"),
                        ("op", "Send+message"),
                    ];
                    let request_builder = user.goose_post(contact_form_url).await?;
                    contact_form = user.goose_send(request_builder.form(&params), None).await?;
                }
                Err(e) => {
                    return user.set_failure(
                        &format!("{}: failed to parse page: {}", goose.request.url, e),
                        &mut goose.request,
                        Some(&headers),
                        None,
                    );
                }
            }
        }
        Err(e) => {
            return user.set_failure(
                &format!("{}: no response from server: {}", goose.request.url, e),
                &mut goose.request,
                None,
                None,
            );
        }
    }

    // Drupal 9 throttles how many times an IP address can submit the contact form, so we
    // need special handling. We check the response, and issue an info! level message if
    // the form is throttled. This is a valid event to load test.
    match contact_form.response {
        Ok(response) => {
            // Copy the headers so we have them for logging if there are errors.
            let headers = &response.headers().clone();
            match response.text().await {
                Ok(html) => {
                    // If the contact form succeeded, we were redirected to the home page.
                    if html.contains("No le está permitido enviar más") {
                        info!("post to contact form was throttled: {}", contact_form.request.url);
                    }

                    // Either way, a "real" user would still load all static elements on
                    // the returned page.
                    common::load_static_elements(user, &html).await;
                },
                Err(e) => {
                    return user.set_failure(
                        &format!("{}: failed to parse page: {}", goose.request.url, e),
                        &mut goose.request,
                        Some(&headers),
                        None,
                    );
                }
            }
        }
        Err(e) => {
            return user.set_failure(
                &format!("{}: no response from server: {}", goose.request.url, e),
                &mut goose.request,
                None,
                None,
            );
        }
    }

    Ok(())
}

// @TODO: search up content, load from results

// @TODO: view listing by taxonomy term: ie /es/tags/huevo
