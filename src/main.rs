extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("mDNS Responder")
        .version("1.0")
        .author("Tom Manley <tom.manley@gmail.com>")
        .about("Advertises a service using mMDS")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .default_value("80")
            .help("Sets the port"))
        .arg(Arg::with_name("service_name")
            .short("n")
            .long("name")
            .takes_value(true)
            .default_value("My Service")
            .help("Set the service name to advertise"))
        .arg(Arg::with_name("service_type")
            .help("Set the type service to advertise, for example '_http._tcp'")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let port = matches
        .value_of("port")
        .unwrap()
        .parse()
        .expect("port must be a number");

    let service_name = matches.value_of("service_name").unwrap();
    let service_type = matches.value_of("service_type").unwrap();

    let mut builder = env_logger::Builder::new();
    match matches.occurrences_of("v") {
        0 => builder.parse_filters("libmdns=warn"),
        1 => builder.parse_filters("libmdns=info"),
        2 => builder.parse_filters("libmdns=debug"),
        _ => builder.parse_filters("libmdns=trace"),
    };
    builder.init();

    // more program logic goes here...
    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        service_type.to_owned(),
        service_name.to_owned(),
        port,
        &["path=/"],
    );

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
