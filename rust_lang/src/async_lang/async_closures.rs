use std::future::Future;

async fn for_each_city<F, Fut>(mut f: F)
where
    F: for<'c> FnMut(&'c str) -> Fut,
    Fut: Future<Output = ()>,
{
    for x in ["New York", "London", "Tokyo"] {
        f(x).await;
    }
}

async fn do_something(city_name: &str) { todo!() }


#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    pub async fn mismatched_types() {
        // cannot compile
        // for_each_city(do_something).await;
    }
}