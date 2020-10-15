use goose::goose::GooseResponse;
use goose::prelude::*;

use rand::seq::SliceRandom;
use regex::Regex;

/// The Umami website defines three content types.
enum ContentType {
    Article,
    BasicPage,
    Recipe,
}

/// Details tracked about individual nodes used to run load test and validate
/// that pages are being correctly loaded.
struct Node<'a> {
    nid: u8,
    url_en: &'a str,
    url_es: &'a str,
    title_en: &'a str,
    title_es: &'a str,
}

/// Defines the actual load test. Each task set simulates a type of user.
///  - Anonymous English user: loads the English version of all pages
///  - Anonymous Spanish user: loads the Spanish version of all pages
fn main() -> Result<(), GooseError> {
    let _goose_metrics = GooseAttack::initialize()?
        .register_taskset(
            taskset!("Anonymous English user")
                .register_task(task!(front_page_en).set_name("anon /").set_weight(2)?)
                .register_task(task!(article_listing_en).set_name("anon /en/articles/"))
                .register_task(
                    task!(article_en)
                        .set_name("anon /en/articles/%")
                        .set_weight(2)?,
                )
                .register_task(task!(recipe_listing_en).set_name("anon /en/recipes/"))
                .register_task(
                    task!(recipe_en)
                        .set_name("anon /en/recipes/%")
                        .set_weight(4)?,
                )
                .register_task(task!(basic_page_en).set_name("anon /en/basicpage"))
                .register_task(task!(page_by_nid).set_name("anon /node/%nid"))
                .set_weight(6)?,
        )
        .register_taskset(
            taskset!("Anonymous Spanish user")
                .register_task(task!(front_page_es).set_name("anon /es/").set_weight(2)?)
                .register_task(task!(article_listing_es).set_name("anon /es/articles/"))
                .register_task(
                    task!(article_es)
                        .set_name("anon /es/articles/%")
                        .set_weight(2)?,
                )
                .register_task(task!(recipe_listing_es).set_name("anon /es/recipes/"))
                .register_task(
                    task!(recipe_es)
                        .set_name("anon /es/recipe/%")
                        .set_weight(4)?,
                )
                .register_task(task!(basic_page_es).set_name("anon /es/basicpage"))
                .register_task(task!(page_by_nid).set_name("anon /node/%nid"))
                .set_weight(2)?,
        )
        .set_default(GooseDefault::Host, "https://drupal-9.0.7.ddev.site/")?
        .execute()?
        .print();

    Ok(())
}

/// Returns a vector of all nodes of a specified content type.
fn get_nodes(content_type: &ContentType) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    match content_type {
        ContentType::Article => {
            nodes.push(Node {
                nid: 10,
                url_en: "/en/articles/give-it-a-go-and-grow-your-own-herbs",
                url_es: "/es/articles/prueba-y-cultiva-tus-propias-hierbas",
                title_en: "Give it a go and grow your own herbs",
                title_es: "Prueba y cultiva tus propias hierbas",
            });
            nodes.push(Node {
                nid: 11,
                url_en: "/en/articles/dairy-free-and-delicious-milk-chocolate",
                url_es: "/es/articles/delicioso-chocolate-sin-lactosa",
                title_en: "Dairy-free and delicious milk chocolate",
                title_es: "Delicioso chocolate sin lactosa",
            });
            nodes.push(Node {
                nid: 12,
                url_en: "/en/articles/the-real-deal-for-supermarket-savvy-shopping",
                url_es: "/es/articles/el-verdadeo-negocio-para-comprar-en-el-supermercado",
                title_en: "The real deal for supermarket savvy shopping",
                title_es: "El verdadero negocio para comprar en el supermercado",
            });
            nodes.push(Node {
                nid: 13,
                url_en: "/en/articles/the-umami-guide-to-our-favourite-mushrooms",
                url_es: "/es/articles/guia-umami-de-nuestras-setas-preferidas",
                title_en: "The Umami guide to our favorite mushrooms",
                title_es: "Guía Umami de nuestras setas preferidas",
            });
            nodes.push(Node {
                nid: 14,
                url_en: "/en/articles/lets-hear-it-for-carrots",
                url_es: "/es/articles/un-aplauso-para-las-zanahorias",
                title_en: "Let&#039;s hear it for carrots",
                title_es: "Un aplauso para las zanahorias",
            });
            nodes.push(Node {
                nid: 15,
                url_en: "/en/articles/baking-mishaps-our-troubleshooting-tips",
                url_es:
                    "/es/articles/percances-al-hornear-nuestros-consejos-para-solucionar-problemas",
                title_en: "Baking mishaps - our troubleshooting tips",
                title_es: "Percances al hornear - nuestros consejos para solucionar los problemas",
            });
            nodes.push(Node {
                nid: 16,
                url_en: "/en/articles/skip-the-spirits-with-delicious-mocktails",
                url_es: "/es/articles/salta-los-espiritus-con-deliciosos-cocteles-sin-alcohol",
                title_en: "Skip the spirits with delicious mocktails",
                title_es: "Salta los espíritus con deliciosos cócteles sin alcohol",
            });
            nodes.push(Node {
                nid: 17,
                url_en: "/en/articles/give-your-oatmeal-the-ultimate-makeover",
                url_es: "/es/articles/dale-a-tu-avena-el-cambio-de-imagen-definitivo",
                title_en: "Give your oatmeal the ultimate makeover",
                title_es: "Dale a tu avena el cambio de imagen definitivo",
            });
        }
        ContentType::BasicPage => {
            nodes.push(Node {
                nid: 18,
                url_en: "/en/about-umami",
                url_es: "/es/acerca-de-umami",
                title_en: "About Umami",
                title_es: "Acerca de Umami",
            });
        }
        ContentType::Recipe => {
            nodes.push(Node {
                nid: 1,
                url_en: "/en/recipes/deep-mediterranean-quiche",
                url_es: "/es/recipes/quiche-mediterráneo-profundo",
                title_en: "Deep mediterranean quiche",
                title_es: "Quiche mediterráneo profundo",
            });
            nodes.push(Node {
                nid: 2,
                url_en: "/en/recipes/vegan-chocolate-and-nut-brownies",
                url_es: "/es/recipes/bizcochos-veganos-de-chocolate-y-nueces",
                title_en: "Vegan chocolate and nut brownies",
                title_es: "Bizcochos veganos de chocolate y nueces",
            });
            nodes.push(Node {
                nid: 3,
                url_en: "/en/recipes/super-easy-vegetarian-pasta-bake",
                url_es: "/es/recipes/pasta-vegetariana-horno-super-facil",
                title_en: "Super easy vegetarian pasta bake",
                title_es: "Pasta vegetariana al horno súper fácil",
            });
            nodes.push(Node {
                nid: 4,
                url_en: "/en/recipes/watercress-soup",
                url_es: "/es/recipes/sopa-de-berro",
                title_en: "Watercress soup",
                title_es: "Sopa de berro",
            });
            nodes.push(Node {
                nid: 5,
                url_en: "/en/recipes/victoria-sponge-cake",
                url_es: "/es/recipes/pastel-victoria",
                title_en: "Victoria sponge cake",
                title_es: "Pastel Victoria",
            });
            nodes.push(Node {
                nid: 6,
                url_en: "/en/recipes/gluten-free-pizza",
                url_es: "/es/recipes/pizza-sin-gluten",
                title_en: "Gluten free pizza",
                title_es: "Pizza sin gluten",
            });
            nodes.push(Node {
                nid: 7,
                url_en: "/en/recipes/thai-green-curry",
                url_es: "/es/recipes/curry-verde-tailandes",
                title_en: "Thai green curry",
                title_es: "Curry verde tailandés",
            });
            nodes.push(Node {
                nid: 8,
                url_en: "/en/recipes/crema-catalana",
                url_es: "/es/recipes/crema-catalana",
                title_en: "Crema catalana",
                title_es: "Crema catalana",
            });
            nodes.push(Node {
                nid: 9,
                url_en: "/en/recipes/fiery-chili-sauce",
                url_es: "/es/recipes/salsa-de-chile-ardiente",
                title_en: "Fiery chili sauce",
                title_es: "Salsa de chile ardiente",
            });
        }
    }

    nodes
}

/// Validate the HTML response, confirming the expected title was returned, then load
/// all static assets found on the page.
async fn validate_and_load_static_assets(
    user: &GooseUser,
    mut goose: GooseResponse,
    title: &str,
) -> GooseTaskResult {
    match goose.response {
        Ok(response) => {
            // Copy the headers so we have them for logging if there are errors.
            let headers = &response.headers().clone();
            match response.text().await {
                Ok(html) => {
                    // Confirm "<title>foo" (where foo is the expected title) is in the
                    // returned HTML text.
                    if !html.contains(&("<title>".to_string() + title)) {
                        return user.set_failure(
                            &format!("{}: title not found: {}", goose.request.url, title),
                            &mut goose.request,
                            Some(&headers),
                            Some(&html),
                        );
                    }

                    // Collect all static image and js assets found in the src= HTML tags.
                    // @TODO: parse HTML5 srcset= also
                    let image = Regex::new(r#"src="(.*?)""#).unwrap();
                    let mut urls = Vec::new();
                    for url in image.captures_iter(&html) {
                        if url[1].starts_with("/sites") || url[1].starts_with("/core") {
                            urls.push(url[1].to_string());
                        }
                    }

                    // Collect all static css assets found in the link href= HTML tags.
                    let css = Regex::new(r#"href="(/sites/default/files/css/.*?)""#).unwrap();
                    for url in css.captures_iter(&html) {
                        urls.push(url[1].to_string());
                    }

                    // Load all static assets on the page.
                    for asset in &urls {
                        let _ = user.get_named(asset, "static asset").await;
                    }
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

    Ok(())
}

/// Load the front page in English and all static assets found on the page.
async fn front_page_en(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/").await?;
    validate_and_load_static_assets(user, goose, "Home").await?;

    Ok(())
}

/// Load recipe listing in English and all static assets found on the page.
async fn recipe_listing_en(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/en/recipes/").await?;
    validate_and_load_static_assets(user, goose, "Recipes").await?;

    Ok(())
}

/// Load a random recipe in English and all static assets found on the page.
async fn recipe_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(recipe.unwrap().url_en).await?;
    validate_and_load_static_assets(user, goose, recipe.unwrap().title_en).await?;

    Ok(())
}

/// Load article listing in English and all static assets found on the page.
async fn article_listing_en(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/en/articles/").await?;
    validate_and_load_static_assets(user, goose, "Articles").await?;

    Ok(())
}

/// Load a random article in English and all static assets found on the page.
async fn article_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(article.unwrap().url_en).await?;
    validate_and_load_static_assets(user, goose, article.unwrap().title_en).await?;

    Ok(())
}

/// Load a random basic page in English and all static assets found on the page.
async fn basic_page_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(page.unwrap().url_en).await?;
    validate_and_load_static_assets(user, goose, page.unwrap().title_en).await?;

    Ok(())
}

/// Load a random node by nid in English and all static assets found on the page.
async fn page_by_nid(user: &GooseUser) -> GooseTaskResult {
    // Randomly select a content type.
    let content_types = vec![
        ContentType::Article,
        ContentType::BasicPage,
        ContentType::Recipe,
    ];
    let content_type = content_types.choose(&mut rand::thread_rng());
    // Then randomly select a node of this content type.
    let nodes = get_nodes(&content_type.unwrap());
    let page = nodes.choose(&mut rand::thread_rng());
    // Load the page by nid instead of by URL.
    let goose = user
        .get(&("/node/".to_string() + &page.unwrap().nid.to_string()))
        .await?;
    validate_and_load_static_assets(user, goose, page.unwrap().title_en).await?;

    Ok(())
}

/// Load the front page in Spanish and all static assets found on the page.
pub async fn front_page_es(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/es").await?;
    validate_and_load_static_assets(user, goose, "Inicio").await?;

    Ok(())
}

/// Load article listing in Spanish and all static assets found on the page.
async fn recipe_listing_es(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/es/recipes/").await?;
    validate_and_load_static_assets(user, goose, "Recetas").await?;

    Ok(())
}

/// Load a random recipe in Spanish and all static assets found on the page.
async fn recipe_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(recipe.unwrap().url_es).await?;
    validate_and_load_static_assets(user, goose, recipe.unwrap().title_es).await?;

    Ok(())
}

/// Load article listing in Spanish and all static assets found on the page.
async fn article_listing_es(user: &GooseUser) -> GooseTaskResult {
    let goose = user.get("/es/articles/").await?;
    validate_and_load_static_assets(user, goose, "Artículos").await?;

    Ok(())
}

/// Load a random article in Spanish and all static assets found on the page.
async fn article_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(article.unwrap().url_es).await?;
    validate_and_load_static_assets(user, goose, article.unwrap().title_es).await?;

    Ok(())
}

/// Load a basic page in Spanish and all static assets found on the page.
async fn basic_page_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let goose = user.get(page.unwrap().url_es).await?;
    validate_and_load_static_assets(user, goose, page.unwrap().title_es).await?;

    Ok(())
}
