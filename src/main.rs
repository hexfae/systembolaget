use inquire::{min_length, MultiSelect, Select};
use strum_macros::Display;
use systembolaget::{
    error::Result,
    systembolaget::{Product, ProductAssortment, ProductType},
    SortBy, SortDirection, Systembolaget,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().init();
    let systembolaget =
        Systembolaget::from_assortment(serde_json::from_str(include_str!("../assortment.json"))?);

    loop {
        match main_menu()? {
            MenuOption::Calculate => calculate(&systembolaget)?,
            MenuOption::Search => search(&systembolaget)?,
            MenuOption::Exit => break,
        };
    }
    Ok(())
}

fn main_menu() -> Result<MenuOption> {
    let options = vec![MenuOption::Calculate, MenuOption::Search, MenuOption::Exit];
    Ok(Select::new("Vad vill du göra?", options).prompt()?)
}

fn search(systembolaget: &Systembolaget) -> Result<()> {
    let products = systembolaget
        .products
        .values()
        .cloned()
        .map(|product| product.product_name_bold)
        .collect::<Vec<String>>();
    Select::new("Söka", products).prompt()?;
    Ok(())
}

fn calculate(systembolaget: &Systembolaget) -> Result<()> {
    let products = filter_products(systembolaget)?;
    let sorted = sort_products(products)?;
    println!("{}", sorted.join("\n"));
    Ok(())
}

fn sort_products(mut products: Vec<Product>) -> Result<Vec<String>> {
    let options = vec![SortDirection::Ascending, SortDirection::Descending];
    let sort_direction = Select::new("Hur vill du sortera produkterna?", options).prompt()?;
    let options = vec![
        SortBy::MlPerCrown,
        SortBy::CrownsPerMl,
        // SortBy::Score,
        SortBy::Price,
        SortBy::Name,
        SortBy::Volume,
        SortBy::Vintage,
        SortBy::ProductLaunchDate,
    ];
    let sort_by = Select::new("Vad vill du sortera efter?", options).prompt()?;
    match sort_by {
        // SortBy::Score => todo!(),
        SortBy::Price => products.sort_by(|first, second| first.price.total_cmp(&second.price)),
        SortBy::Name => {
            products
                .sort_by(|first, second| first.product_name_bold.cmp(&second.product_name_bold));
        }
        SortBy::Volume => {
            products
                .sort_by(|first, second| first.product_name_bold.cmp(&second.product_name_bold));
        }
        SortBy::Vintage => products.sort_by(|first, second| first.vintage.cmp(&second.vintage)),
        SortBy::ProductLaunchDate => products
            .sort_by(|first, second| first.product_launch_date.cmp(&second.product_launch_date)),
        SortBy::MlPerCrown => products
            .sort_by(|first, second| first.to_ml_per_crown().total_cmp(&second.to_ml_per_crown())),
        SortBy::CrownsPerMl => products.sort_by(|first, second| {
            first
                .to_crowns_per_ml()
                .total_cmp(&second.to_crowns_per_ml())
        }),
    };

    if sort_direction == SortDirection::Descending {
        products.reverse();
    }

    // doing this in a second match as to not make the first match so big
    let output = match sort_by {
        // SortBy::Score => todo!(),
        SortBy::Price => products
            .iter()
            .map(Product::link_display_price)
            .collect::<Vec<String>>(),
        SortBy::Name => products
            .iter()
            .map(Product::link_display_name)
            .collect::<Vec<String>>(),
        SortBy::Volume => products
            .iter()
            .map(Product::link_display_volume)
            .collect::<Vec<String>>(),
        SortBy::Vintage => products
            .iter()
            .map(Product::link_display_vintage)
            .collect::<Vec<String>>(),
        SortBy::ProductLaunchDate => products
            .iter()
            .map(Product::link_display_product_launch_date)
            .collect::<Vec<String>>(),
        SortBy::MlPerCrown => products
            .iter()
            .map(Product::link_display_ml_per_crown)
            .collect::<Vec<String>>(),
        SortBy::CrownsPerMl => products
            .iter()
            .map(Product::link_display_crowns_per_ml)
            .collect::<Vec<String>>(),
    };

    Ok(output)
}

fn filter_products(systembolaget: &Systembolaget) -> Result<Vec<Product>> {
    let options = vec![
        ProductAssortment::Fixed,
        ProductAssortment::OrderWare,
        ProductAssortment::Season,
        ProductAssortment::Temporary,
        ProductAssortment::Local,
        ProductAssortment::Web,
    ];
    let assortment_selections = MultiSelect::new("Vilka sortiment vill du söka i?", options)
        .with_default(&[0])
        .with_validator(min_length!(1, "Hallå, välj minst en!"))
        .prompt()?;
    let options = vec![
        ProductType::Wine,
        ProductType::Beer,
        ProductType::Spirit,
        ProductType::Cider,
        ProductType::AlcoholFree,
    ];
    let product_type_selections = MultiSelect::new("Vilka produkter vill du söka efter?", options)
        .with_default(&[0, 1, 2, 3])
        .with_validator(min_length!(1, "Hallå, välj minst en!"))
        .prompt()?
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    Ok(assortment_selections
        .iter()
        .flat_map(|assortment| {
            systembolaget
                .products
                .values()
                .filter(|product| {
                    product.assortment_text == assortment.to_string()
                        && product_type_selections.contains(&product.category_level1)
                })
                .cloned()
                .collect::<Vec<Product>>()
        })
        .collect::<Vec<Product>>())
}

#[derive(Display)]
enum MenuOption {
    #[strum(to_string = "Beräkna")]
    Calculate,
    #[strum(to_string = "Söka")]
    Search,
    #[strum(to_string = "Avsluta")]
    Exit,
}
