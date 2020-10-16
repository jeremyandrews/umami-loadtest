use goose::goose::GooseResponse;
use goose::prelude::*;

use regex::Regex;

/// The Umami website defines three content types.
pub enum ContentType {
    Article,
    BasicPage,
    Recipe,
}

/// Details tracked about individual nodes used to run load test and validate
/// that pages are being correctly loaded.
pub struct Node<'a> {
    pub nid: u8,
    pub url_en: &'a str,
    pub url_es: &'a str,
    pub title_en: &'a str,
    pub title_es: &'a str,
}

/// Returns a vector of all nodes of a specified content type.
pub fn get_nodes(content_type: &ContentType) -> Vec<Node> {
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

/// A valid title on this website starts with "<title>foo", where "foo" is the expected
/// title text. Returns true if the expected title is set, otherwise returns false.
pub fn valid_title(html: &str, title: &str) -> bool {
    if html.contains(&("<title>".to_string() + title)) {
        return true
    } else {
        return false;
    }
}

/// Finds all local static elements on the page and loads them asynchronously.
/// This default profile only has local assets, so we can use simple patterns.
pub async fn load_static_elements(user: &GooseUser, html: &str) {
    // Use a regular expression to find all src=<foo> in the HTML, where foo
    // is the URL to image and js assets.
    // @TODO: parse HTML5 srcset= also
    let image = Regex::new(r#"src="(.*?)""#).unwrap();
    let mut urls = Vec::new();
    for url in image.captures_iter(&html) {
        if url[1].starts_with("/sites") || url[1].starts_with("/core") {
            urls.push(url[1].to_string());
        }
    }

    // Use a regular expression to find all href=<foo> in the HTML, where foo
    // is the URL to css assets.
    let css = Regex::new(r#"href="(/sites/default/files/css/.*?)""#).unwrap();
    for url in css.captures_iter(&html) {
        urls.push(url[1].to_string());
    }

    // Load all the static assets found on the page.
    for asset in &urls {
        let _ = user.get_named(asset, "static asset").await;
    }

}

/// Validate the HTML response, confirming the expected title was returned, then load
/// all static assets found on the page.
pub async fn validate_and_load_static_assets(
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
                    if !valid_title(&html, &title) {
                        return user.set_failure(
                            &format!("{}: title not found: {}", goose.request.url, title),
                            &mut goose.request,
                            Some(&headers),
                            Some(&html),
                        );
                    }

                    load_static_elements(user, &html).await;
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

/// Use regular expression to get the value of a named form element.
pub fn get_form_value(html: &str, name: &str) -> Option<String> {
    let re = Regex::new(&format!(r#"name="{}" value=['"](.*?)['"]"#, name)).unwrap();
    match re.captures(&html) {
        Some(value) => Some(value[1].to_string()),
        None => None,
    }
}
