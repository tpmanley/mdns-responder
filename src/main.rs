use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("mDNS Responder")
        .version("1.0")
        .author("Tom Manley <tom.manley@gmail.com>")
        .about("Advertises a service using mMDS")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .num_args(1)
                .default_value("80")
                .help("Sets the port"),
        )
        .arg(
            Arg::new("service_name")
                .short('n')
                .long("name")
                .num_args(1)
                .default_value("My Service")
                .help("Set the service name to advertise"),
        )
        .arg(
            Arg::new("service_type")
                .help("Set the type service to advertise, for example '_http._tcp'")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .action(ArgAction::Count)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let port = matches
        .get_one::<String>("port")
        .unwrap()
        .parse()
        .expect("port must be a number");

    let service_name = matches.get_one::<String>("service_name").unwrap().as_str();
    let service_type = matches.get_one::<String>("service_type").unwrap().as_str();

    let mut builder = env_logger::Builder::new();
    match matches.get_count("v") {
        0 => builder.parse_filters("libmdns=warn"),
        1 => builder.parse_filters("libmdns=info"),
        2 => builder.parse_filters("libmdns=debug"),
        _ => builder.parse_filters("libmdns=trace"),
    };
    builder.init();

    let responder = libmdns::Responder::new();
    let _svc = responder.register(service_type, service_name, port, &["path=/"]);

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
