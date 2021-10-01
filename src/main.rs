mod data;
use crate::data::{Definitions, BANNER, DEFS};
use anyhow::anyhow;
use anyhow::Result as AnyResult;
use clap::crate_version;
use clap::{App, Arg, ArgMatches};
use console::style;
use service_policy_kit::data::Context;
use service_policy_kit::runner::{RunOptions, SequenceRunner};
use std::process::exit;

fn main() {
    env_logger::init();

    let app = App::new("keyscope")
        .version(env!("VERGEN_GIT_SEMVER"))
        .version(crate_version!())
        .about("Key validation and rotation toolkit")
        .arg(
            Arg::new("dry_run")
                .short('d')
                .long("dry-run")
                .value_name("EXAMPLE_KEY")
                .about("Dry run with examples given in EXAMPLE_KEY")
                .takes_value(true),
        )
        .arg(
            Arg::new("reporter")
                .short('r')
                .long("reproter")
                .value_name("REPORTER")
                .takes_value(true)
                .possible_values(&["console"])
                .about("Reporter to use (default: 'console')"),
        )
        .arg(
            Arg::new("definitions")
                .short('f')
                .long("definitions")
                .value_name("DEFINITIONS_FILE")
                .takes_value(true)
                .about("Path to custom definitions file"),
        )
        .arg(
            Arg::new("no_banner")
                .long("no-banner")
                .about("Don't show the banner")
                .takes_value(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .about("Show details about interactions")
                .takes_value(false),
        )
        .arg(
            Arg::new("vars_format")
                .long("vars-format")
                .value_name("VARS_FORMAT")
                .about("Specify vars replacement format, with 'var' e.g. --vars-format \"<<var>>\"")
                .takes_value(true),
        )
        .arg(
            Arg::new("flip")
                .long("flip")
                .about("Flip the meaning of success")
                .takes_value(false),
        )
        .subcommand(
            App::new("validate")
                .about("Validate keys")
                .arg(
                    Arg::new("provider")
                        .index(1)
                        .value_name("PROVIDER")
                        .about("Specify the provider to validate for")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("list")
                        .long("list")
                        .about("Show provider list")
                        .takes_value(false),
                )
                .arg(
                    Arg::new("requirements")
                        .long("requirements")
                        .about("Show provider requirements (params)")
                        .takes_value(false),
                )
                .arg(
                    Arg::new("params")
                        .long("params")
                        .short('p')
                        .value_name("PROVIDER")
                        .about("Specify the provider to validate for")
                        .multiple_values(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("csv_in")
                        .long("--csv-in")
                        .value_name("FILE")
                        .about("Read providers and params via CSV")
                        .takes_value(true),
                ),
        );
    let v = app.render_version();
    let matches = app.to_owned().get_matches();

    if !matches.is_present("no_banner") {
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }

    if let Some(validate_matches) = matches.subcommand_matches("validate") {
        match validate_command(&matches, validate_matches) {
            Ok(ok) => {
                exit(if ok { 0 } else { 1 });
            }
            Err(err) => {
                eprintln!("error: {}", err.to_string());
            }
        }
    }
}
fn validate_command(matches: &ArgMatches, cmd_matches: &ArgMatches) -> AnyResult<bool> {
    let defs: Definitions = match matches.value_of("definitions") {
        Some(defs_file) => serde_yaml::from_reader(std::fs::File::open(defs_file)?)?,
        None => serde_yaml::from_str(DEFS).unwrap(),
    };
    let opts = RunOptions::build(
        matches.value_of("dry_run").map(|s| s.to_string()),
        matches.is_present("flip"),
        matches.value_of("reporter").map(|s| s.to_string()),
        matches.is_present("verbose"),
    )?;
    if cmd_matches.is_present("list") {
        let mut vec = defs.providers.iter().collect::<Vec<_>>();
        vec.sort_by(|(p1, _), (p2, _)| p1.cmp(p2));
        vec.iter().for_each(|(provider, action)| {
            if let Some(validation) = action.validation.as_ref() {
                println!(
                    "{}\nkeyscope validate {} -p {}\n",
                    style(validation.request.get_id()).magenta(),
                    style(provider).yellow(),
                    style(
                        validation
                            .request
                            .params
                            .as_ref()
                            .map(|p| p
                                .iter()
                                .map(|p| p.name.clone())
                                .collect::<Vec<_>>()
                                .join(" "))
                            .unwrap_or_default()
                    )
                    .blue()
                )
            }
        });
        println!(
            "Total {} providers available.",
            vec.iter()
                .filter(|(_, action)| action.validation.is_some())
                .count(),
        );
        Ok(true)
    } else if cmd_matches.is_present("csv_in") {
        let file = cmd_matches
            .value_of("csv_in")
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("missing csv file"))?;
        let mut rdr = csv::Reader::from_path(file)?;
        let mut interactions = vec![];
        let mut context = Context::new();
        for res in rdr.records() {
            let record = res?;
            let provider = record.iter().next().unwrap();

            // push all keys as: provider_0, provider_1 ..
            record.iter().skip(1).enumerate().for_each(|(i, v)| {
                context
                    .vars_bag
                    .insert(format!("{}_{}", provider, i + 1), v.to_string());
            });
            let interaction = defs.validation_for(&context, provider)?;
            interactions.push(interaction);
        }

        let mut runner = SequenceRunner::from_opts(&opts);

        let resp = runner.run(&mut context, &interactions);
        Ok(resp.ok)
    } else if cmd_matches.is_present("requirements") {
        let provider = cmd_matches
            .value_of("provider")
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("missing provider parameter"))?;
        let params = defs.requirements_for(&provider)?;
        if let Some(params) = params {
            println!("provider {} requires:", provider);
            params.iter().for_each(|p| {
                println!(" - param: {}\n   desc: {}", p.name, p.desc);
            });
        } else {
            println!("provider {} has no requirements.", provider);
        }

        Ok(true)
    } else {
        let provider = cmd_matches
            .value_of("provider")
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("missing provider parameter"))?;
        let params = cmd_matches
            .values_of("params")
            .map(|vs| {
                vs.enumerate()
                    .map(|(i, s)| (format!("{}_{}", provider, i + 1), s.to_string()))
                    .collect::<Vec<_>>()
            })
            .ok_or_else(|| anyhow!("missing params parameter"))?;
        let mut context = Context::new();

        params.into_iter().for_each(|(k, v)| {
            context.vars_bag.insert(k, v);
        });
        let interaction = defs.validation_for(&context, &provider)?;

        let mut runner = SequenceRunner::from_opts(&opts);

        let resp = runner.run(&mut context, &vec![interaction]);
        Ok(resp.ok)
    }
}
