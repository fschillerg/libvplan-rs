use chrono::{Datelike, Weekday};
use chrono_tz::Europe::Berlin;
use libvplan::{Client, WeekType};
use log::error;
use std::process;
use structopt::StructOpt;
use tokio::runtime::Runtime;

macro_rules! errorexit {
    ($($error:tt)*) => {{
        error!($($error)*);
        process::exit(1);
    }}
}

/// libvplan CLI tool
#[derive(StructOpt, Debug)]
#[structopt(name = "vplan")]
struct Cli {
    /// Login username
    #[structopt(short = "u", long = "username")]
    username: String,
    /// Login password
    #[structopt(short = "p", long = "password")]
    password: String,
    /// Weekday to fetch vplan for
    #[structopt(name = "weekday", parse(from_str))]
    weekday: String,
}

fn main() {
    let cli = Cli::from_args();

    pretty_env_logger::init();

    let weekday = match cli.weekday.to_lowercase().as_ref() {
        "monday" => Weekday::Mon,
        "tuesday" => Weekday::Tue,
        "wednesday" => Weekday::Wed,
        "thursday" => Weekday::Thu,
        "friday" => Weekday::Fri,
        _ => errorexit!("invalid weekday"),
    };

    let client = Client::with_credentials(
        "https://fssgym.de/vplan",
        cli.username.as_ref(),
        cli.password.as_ref(),
    );

    let future = client.get(weekday);

    let mut rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(error) => errorexit!("{}", error),
    };

    match rt.block_on(future) {
        Ok(vplan) => {
            let date = vplan.date.date.with_timezone(&Berlin);

            let week_type = match vplan.date.week_type {
                WeekType::A => "A",
                WeekType::B => "B",
            };

            println!(
                "{}.{}.{} ({}-Woche)\n",
                date.day(),
                date.month(),
                date.year(),
                week_type
            );

            for change in vplan.changes {
                println!(
                    "{}: {} {} {} {} {}",
                    change.form,
                    change.lesson,
                    change.subject,
                    change.teacher,
                    change.room,
                    change.info
                );
            }

            for info in vplan.info {
                println!("{}", info);
            }
        }
        Err(error) => errorexit!("{}", error),
    }
}
