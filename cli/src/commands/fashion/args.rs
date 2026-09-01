#[derive(clap::Args, Debug)]
pub struct FashionFields {
    /// Name of the fashion template.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Associated character.
    #[arg(short, long, alias = "char")]
    pub character: Option<String>,

    /// Description of the fashion template.
    #[arg(short, long, value_name = "TEXT")]
    pub description: Option<String>,

    /// Chatlink of the wardrobe template.
    #[arg(long, value_name = "CHATLINK")]
    pub wardrobe: Option<String>,

    /// Chatlink of the travel template.
    #[arg(long, value_name = "CHATLINK")]
    pub travel: Option<String>,

    /// Tags
    #[arg(short, long = "tag", value_name = "TAG")]
    pub tags: Vec<String>,
}

#[derive(clap::Args, Debug)]
#[command(group(clap::ArgGroup::new("identifier").multiple(false)))]
pub struct FashionIdentifier {
    /// Id of the fashion template.
    #[arg(long, value_name = "UUID", group = "identifier")]
    id: Option<String>,

    /// Name of the fashion template.
    #[arg(short, long, group = "identifier")]
    pub name: Option<String>,

    /// Associated character.
    #[arg(short, long, requires = "name", conflicts_with = "id", alias = "char")]
    pub character: Option<String>,
}
