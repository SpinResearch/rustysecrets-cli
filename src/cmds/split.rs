use mime::Mime;
use rusty_secrets::{sss, wrapped_secrets};

use errors::*;
use input::Input;

use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
pub fn split(
    mut secret_input: Input,
    output_path: &Path,
    k: u8,
    n: u8,
    mime_type: Option<Mime>,
    sign_shares: bool,
    raw: bool,
    share_tmpl: &str,
) -> Result<()> {
    if k > n {
        bail!(ErrorKind::KMustBeSmallerThanN(k, n))
    }

    if raw && mime_type.is_some() {
        bail!(ErrorKind::RawMimeConflict);
    }

    debug!("Reading secret...");

    let mut secret = Vec::new();
    let size = secret_input
        .read_to_end(&mut secret)
        .chain_err(|| ErrorKind::CannotReadSecret(secret_input))?;

    debug!("Read {} bytes.", size);

    debug!("Generating shares...");

    let shares = if raw {
        sss::split_secret(k, n, &secret, sign_shares)
    } else {
        let mime_type = mime_type.map(|m| m.as_ref().to_string());
        wrapped_secrets::split_secret(k, n, &secret, mime_type, sign_shares)
    }.chain_err(|| "Could not generate shares")?;

    for (num, share) in shares.iter().enumerate() {
        let mut path_buf = output_path.to_path_buf();
        path_buf.push(share_tmpl.replace("{{num}}", &format!("{}", num)));
        let share_path = path_buf.as_path();

        debug!("Writing share #{} to '{}'...", num, share_path.display());

        let mut share_file = File::create(share_path)
            .chain_err(|| ErrorKind::CannotCreateShareFile(format!("{}", share_path.display())))?;

        share_file
            .write_all(share.as_bytes())
            .chain_err(|| ErrorKind::CannotWriteShareDataToFile(format!("{}", share_path.display())))?;
    }

    info!("Wrote {} shares to '{}'", n, output_path.display());

    Ok(())
}
