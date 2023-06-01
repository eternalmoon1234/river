use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug)]
    enum App {
        Svelte,
        React,
        Vue,
        Angular,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum Runtime {
        Flask,
        Express,
        Django,
        Actix,
    }
}

#[derive(StructOpt, Debug)]
enum Cli {
    Deploy {
        #[structopt(short, long, possible_values = &App::variants(), case_insensitive = true)]
        app: App,

        #[structopt(short, long, possible_values = &Runtime::variants(), case_insensitive = true)]
        runtime: Runtime,

        #[structopt(short="t",long)]
        region: String,

        #[structopt(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);

    match args {
        Cli::Deploy { app, runtime, region, name } => {
            println!("Deploying {} {} app to {} with name {}", app, runtime, region, name);
        }
    }

}
