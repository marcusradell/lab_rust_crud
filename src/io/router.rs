macro_rules! kit_router {
    ($($module:ident),*) => {
        impl Routable for Kit {
            fn router(&self) -> Router {
                Router::new().nest(
                    route_path!(),
                    Router::new()
                        $(
                            .merge($module::route(self.clone()))
                        )*
                )
            }
        }
    };
}

pub(crate) use kit_router;
