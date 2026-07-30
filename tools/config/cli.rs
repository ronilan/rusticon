use std::env;

pub struct CliArgs {
    pub name: Option<String>,
    pub app_name: Option<String>,
    pub tagline: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
}

pub fn parse_args() -> CliArgs {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut cli = CliArgs {
        name: None,
        app_name: None,
        tagline: None,
        keywords: None,
        description: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--name" if i + 1 < args.len() => {
                cli.name = Some(args[i + 1].clone());
                i += 2;
            }
            "--app-name" if i + 1 < args.len() => {
                cli.app_name = Some(args[i + 1].clone());
                i += 2;
            }
            "--tagline" if i + 1 < args.len() => {
                cli.tagline = Some(args[i + 1].clone());
                i += 2;
            }
            "--keywords" if i + 1 < args.len() => {
                cli.keywords = Some(args[i + 1].clone());
                i += 2;
            }
            "--description" if i + 1 < args.len() => {
                cli.description = Some(args[i + 1].clone());
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }
    cli
}
