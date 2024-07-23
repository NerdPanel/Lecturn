mod api;
mod parser;

#[cfg(test)]
mod tests {
    use crate::api;

    #[test]
    fn test() {
        println!("{}", format!("A list of all stable versions of Minecraft: \
        {}", api::list_minecraft_versions_all(true)));

        divider();

        println!("{}", format!("Is 1.21 stable: {}", api::is_stable("1.21".to_owned())));
        println!("{}", format!("Is 1.20.1 the latest: {}", api::is_latest("1.20.1".to_owned(), false)));

        divider();

        println!("Attempting to download Minecraft 1.21");
        api::download("1.21".to_owned());
    }

    fn divider() {
        println!("\
        \
        ----------------------------------------------------------\
        \
        ");
    }
}
