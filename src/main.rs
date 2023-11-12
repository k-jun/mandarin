use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use std::{env, fs};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = env!("CARGO_HOME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "Declarative TOML configuration for Gmail filters")]
enum Mandarin {
    Init {},
    Path {},
    Run {},
}

#[derive(Deserialize, Debug)]
struct Filter {
    query: String,
    label: Option<String>,
    archive: Option<bool>,
    read: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct Config {
    filter: Vec<Filter>,
}

fn main() -> Result<()> {
    let h = env::var("HOME")?;
    let dp = Path::new(&h).join(".mandarin");
    let cp = Path::new(&dp).join("config.toml");

    let args = Mandarin::from_args();
    match args {
        Mandarin::Init {} => {
            let contents = format!(
                "[[filter]]
query = \"to:(dammy@gmail.com)\"
label = \"000\"
[[filter]]
query = \"to:(dammy@gmail.com)\"
label = \"100\"

[[filter]]
query = \"to:(hoge@gmail.com)\"
label = \"000/001_hoge\"
archive = true
read = true"
            );
            if !dp.exists() {
                fs::create_dir(&dp)?;
            }
            if !cp.exists() {
                fs::write(&cp, contents)?;
            }
        }
        Mandarin::Path {} => println!("{}", cp.display()),
        Mandarin::Run {} => {
            let cs = fs::read_to_string(cp)?;
            let ct: Config = toml::from_str(&cs)?;

            let property = |k: &str, v: &str| -> String {
                format!("<apps:property name='{}' value='{}'/>", k, v)
            };

            let entry = |s: &str| -> String {
                format!("<entry><category term='filter'></category>{}</entry>", s)
            };

            let feed = |s: &str| -> String {
                format!("<?xml version='1.0' encoding='UTF-8'?>
<feed xmlns='http://www.w3.org/2005/Atom' xmlns:apps='http://schemas.google.com/apps/2006'>
    <title>Mail Filters</title>
    <author>
        <name>k-jun</name>
        <email>k-jun@gmail.com</email>
    </author>
    {}
</feed>", s)
            };

            let mut output = String::new();
            for f in ct.filter {
                let mut o = String::new();
                let kv = f.query.split(':').collect::<Vec<&str>>();
                let (k, v) = (kv[0], &kv[1..].join(":"));
                o += &property(k, v);

                if let Some(label) = f.label {
                    o += &property("label", &label);
                }
                if let Some(b) = f.archive {
                    o += &property("shouldArchive", &b.to_string());
                }
                if let Some(b) = f.read {
                    o += &property("shouldMarkAsRead", &b.to_string());
                }
                output += &entry(&o);
            }
            println!("{}", feed(&output));
        }
    }
    Ok(())
}