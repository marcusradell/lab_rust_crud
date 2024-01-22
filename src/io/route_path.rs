macro_rules! route_path {
    () => {
        &format!("/{}", module_path!().split("::").last().unwrap())
    };
}

pub(crate) use route_path;
