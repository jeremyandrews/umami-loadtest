use goose::prelude::*;

use rand::seq::SliceRandom; // 0.7.2

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
}

// Run the load test.
fn main() -> Result<(), GooseError> {
    let _goose_metrics = GooseAttack::initialize()?
        .register_taskset(taskset!("Anonymous English user")
            .register_task(task!(front_page_en).set_name("anon /").set_weight(2)?)
            .register_task(task!(article_page_en).set_name("anon /en/article").set_weight(2)?)
            .register_task(task!(recipe_page_en).set_name("anon /en/recipe").set_weight(4)?)
            .register_task(task!(basic_page_en).set_name("anon /en/basicpage"))
            .set_weight(6)?
        )
        .register_taskset(taskset!("Anonymous Spanish user")
            .register_task(task!(front_page_es).set_name("anon / spanish").set_weight(2)?)
            .register_task(task!(article_page_es).set_name("anon /es/article").set_weight(2)?)
            .register_task(task!(recipe_page_es).set_name("anon /es/recipe").set_weight(4)?)
            .register_task(task!(basic_page_es).set_name("anon /en/basicpage"))
            .set_weight(2)?
        )
        .set_default(GooseDefault::Host, "https://drupal-9.0.7.ddev.site/")?
        .execute()?
        .print();

    Ok(())
}

/// Returns a vector of nodes in whatever content type is specified.
fn get_nodes(content_type: &ContentType) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    match content_type {
        ContentType::Article => {
            nodes.push(Node {
                nid: 10,
                url_en: "/en/articles/give-it-a-go-and-grow-your-own-herbs",
                url_es: "/es/articles/prueba-y-cultiva-tus-propias-hierbas",
            });
            nodes.push(Node {
                nid: 11,
                url_en: "/en/articles/dairy-free-and-delicious-milk-chocolate",
                url_es: "/es/articles/delicioso-chocolate-sin-lactosa",
            });
            nodes.push(Node {
                nid: 12,
                url_en: "/en/articles/the-real-deal-for-supermarket-savvy-shopping",
                url_es: "/es/articles/el-verdadeo-negocio-para-comprar-en-el-supermercado",
            });
            nodes.push(Node {
                nid: 13,
                url_en: "/en/articles/the-umami-guide-to-our-favourite-mushrooms",
                url_es: "/es/articles/guia-umami-de-nuestras-setas-preferidas",
            });
            nodes.push(Node {
                nid: 14,
                url_en: "/en/articles/lets-hear-it-for-carrots",
                url_es: "/es/articles/un-aplauso-para-las-zanahorias",
            });
            nodes.push(Node {
                nid: 15,
                url_en: "/en/articles/baking-mishaps-our-troubleshooting-tips",
                url_es: "/es/articles/percances-al-hornear-nuestros-consejos-para-solucionar-problemas",
            });
            nodes.push(Node {
                nid: 16,
                url_en: "/en/articles/skip-the-spirits-with-delicious-mocktails",
                url_es: "/es/articles/salta-los-espiritus-con-deliciosos-cocteles-sin-alcohol",
            });
            nodes.push(Node {
                nid: 17,
                url_en: "/en/articles/give-your-oatmeal-the-ultimate-makeover",
                url_es: "/es/articles/dale-a-tu-avena-el-cambio-de-imagen-definitivo",
            });
        },
        ContentType::BasicPage => {
            nodes.push(Node {
                nid: 18,
                url_en: "/en/about-umami",
                url_es: "/es/acerca-de-umami",
            });
        },
        ContentType::Recipe => {
            nodes.push(Node {
                nid: 1,
                url_en: "/en/recipes/deep-mediterranean-quiche",
                url_es: "/es/recipes/quiche-mediterráneo-profundo",
            });
            nodes.push(Node {
                nid: 2,
                url_en: "/en/recipes/vegan-chocolate-and-nut-brownies",
                url_es: "/es/recipes/bizcochos-veganos-de-chocolate-y-nueces",
            });
            nodes.push(Node {
                nid: 3,
                url_en: "/en/recipes/super-easy-vegetarian-pasta-bake",
                url_es: "/es/recipes/pasta-vegetariana-horno-super-facil",
            });
            nodes.push(Node {
                nid: 4,
                url_en: "/en/recipes/watercress-soup",
                url_es: "/es/recipes/sopa-de-berro",
            });
            nodes.push(Node {
                nid: 5,
                url_en: "/en/recipes/victoria-sponge-cake",
                url_es: "/es/recipes/pastel-victoria",
            });
            nodes.push(Node {
                nid: 6,
                url_en: "/en/recipes/gluten-free-pizza",
                url_es: "/es/recipes/pizza-sin-gluten",
            });
            nodes.push(Node {
                nid: 7,
                url_en: "/en/recipes/thai-green-curry",
                url_es: "/es/recipes/curry-verde-tailandes",
            });
            nodes.push(Node {
                nid: 8,
                url_en: "/en/recipes/crema-catalana",
                url_es: "/es/recipes/crema-catalana",
            });
            nodes.push(Node {
                nid: 9,
                url_en: "/en/recipes/fiery-chili-sauce",
                url_es: "/es/recipes/salsa-de-chile-ardiente",
            });
        },
    }

    nodes
}

// Load the front page in English.
async fn front_page_en(user: &GooseUser) -> GooseTaskResult {
    let _goose = user.get("/").await?;

    Ok(())
}

// Load a recipe in English.
async fn recipe_page_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let _goose = user.get(recipe.unwrap().url_en).await?;

    Ok(())
}

// Load an article in English.
async fn article_page_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let _goose = user.get(article.unwrap().url_en).await?;

    Ok(())
}

// Load a basic page in English.
async fn basic_page_en(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let _goose = user.get(page.unwrap().url_en).await?;

    Ok(())
}

// Load the front page in Spanish.
pub async fn front_page_es(user: &GooseUser) -> GooseTaskResult {
    let _goose = user.get("/es").await?;

    Ok(())
}

// Load a recipe in Spanish.
async fn recipe_page_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Recipe);
    let recipe = nodes.choose(&mut rand::thread_rng());
    let _goose = user.get(recipe.unwrap().url_es).await?;

    Ok(())
}

// Load an article in Spanish.
async fn article_page_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::Article);
    let article = nodes.choose(&mut rand::thread_rng());
    let _goose = user.get(article.unwrap().url_es).await?;

    Ok(())
}

// Load a basic page in Spanish.
async fn basic_page_es(user: &GooseUser) -> GooseTaskResult {
    let nodes = get_nodes(&ContentType::BasicPage);
    let page = nodes.choose(&mut rand::thread_rng());
    let _goose = user.get(page.unwrap().url_es).await?;

    Ok(())
}
