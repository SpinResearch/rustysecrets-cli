use rusty_secrets::{sss, wrapped_secrets};

use errors::*;

use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write};

pub fn recover(shares_paths: Vec<&Path>, output_path: Option<&Path>, verify_signatures: bool, raw: bool) -> Result<()> {
    let mut shares = Vec::with_capacity(shares_paths.len());

    for share_path in shares_paths {
        if !share_path.exists() {
            bail!(ErrorKind::ShareDoesNotExists(format!(
                "{}",
                share_path.display()
            ),))
        }
        if !share_path.is_file() {
            bail!(ErrorKind::ShareIsNotAFile(format!(
                "{}",
                share_path.display()
            ),))
        }

        debug!("Reading share {:?}... ", share_path);

        let mut share_file =
            File::open(share_path).chain_err(|| ErrorKind::CannotOpenShare(format!("{}", share_path.display())))?;

        let mut share = String::new();
        let size = share_file
            .read_to_string(&mut share)
            .chain_err(|| ErrorKind::CannotReadShare(format!("{}", share_path.display())))?;

        debug!("Read {} bytes.", size);

        shares.push(share);
    }

    debug!("Recovering secret... ");

    let secret = if raw {
        sss::recover_secret(&shares, verify_signatures).chain_err(|| ErrorKind::CannotRecoverSecret)?
    } else {
        let mut res =
            wrapped_secrets::recover_secret(&shares, verify_signatures).chain_err(|| ErrorKind::CannotRecoverSecret)?;

        debug!("Version: {:?}", res.get_version());

        if res.get_mime_type() != "" {
            debug!("MIME-Type: {}", res.get_mime_type());
        }

        res.take_secret()
    };

    match output_path {
        Some(output_path) => {
            let mut output_file = File::create(output_path)
                .chain_err(|| ErrorKind::CannotCreateSecretFile(format!("{}", output_path.display())))?;
            output_file
                .write_all(&secret)
                .chain_err(|| ErrorKind::CannotWriteSecretToFile(format!("{}", output_path.display())))?;
        }
        None => {
            // See https://github.com/romac/rustysecrets-cli/issues/9
            // let secret_str = String::from_utf8(secret)
            //     .chain_err(|| "Could not parse secret as UTF-8, consider outputting it to a file instead")?;

            io::stdout()
                .write_all(&secret)
                .chain_err(|| ErrorKind::CannotWriteSecretToStdout)?;
        }
    }

    Ok(())
}
