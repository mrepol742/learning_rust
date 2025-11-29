fn main() {
    let some_product = Some("laptop");
    let mut product_vec = vec!["cellphone", "battery", "charger"];

    // match some_product {
    //     Some(product) => product_vec.push(product),
    //     _ => ()
    // }

    // if let Some(product) = some_product {
    //     product_vec.push(product);
    // }

    // product_vec.extend(some_product);
    // println!("{:?}", product_vec);

    // let products_iter = product_vec.iter().chain(some_product.iter());
    // for prod in products_iter {
    //     println!("{:?}", prod);
    // }
    //

    let products = vec![Some("Carger"), Some("Battery"), Some("Cellphone")];
    // let mut prod_without_none = Vec::new();

    // for p in products {
    //     if p.is_some() {
    //         prod_without_none.push(p.unwrap());
    //     }
    // }
    // println!("{:?}", prod_without_none);
    //

    // let prod_without_none = Box::new(products.into_iter())
    //     .filter(|x|x.as_ref().is_some())
    //     .map(|x|x.unwrap())
    //     .collect::<Vec<&str>>();

    // println!("{:?}", prod_without_none);
    //

    let products: Vec<&str> = products.into_iter().flatten().collect();
    println!("{:?}", products);

}
