use std::process::ExitCode;

use std::collections::BTreeSet;

use std::io;

use std::io::BufRead;

use std::io::Write;

use rs_asn1_uniq_str2int::bind;
use rs_asn1_uniq_str2int::lift;

use rs_asn1_uniq_str2int::set2der_bytes;

fn stdin2strings() -> impl Iterator<Item = Result<String, io::Error>> {
    let i = io::stdin();
    let il = i.lock();
    il.lines()
}

fn strings2sorted<I>(s: I) -> Result<BTreeSet<String>, io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    s.collect()
}

fn stdin2set() -> Result<BTreeSet<String>, io::Error> {
    bind!(
        || { Ok::<_, io::Error>(stdin2strings()) },
        lift!(strings2sorted)
    )()
}

fn stdin2der() -> Result<Vec<u8>, io::Error> {
    bind!(stdin2set, lift!(set2der_bytes))()
}

fn bytes2stdout(b: Vec<u8>) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    ol.write_all(&b)?;
    ol.flush()?;
    Ok(())
}

fn stdin2lines2der2stdout() -> Result<(), io::Error> {
    bind!(stdin2der, lift!(bytes2stdout))()
}

fn main() -> ExitCode {
    stdin2lines2der2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
