//! The RFC959 Allocate (`ALLO`) command
//
// This command may be required by some servers to reserve
// sufficient storage to accommodate the new file to be
// transferred.  The argument shall be a decimal integer
// representing the number of bytes (using the logical byte
// size) of storage to be reserved for the file.  For files
// sent with record or page structure a maximum record or page
// size (in logical bytes) might also be necessary; this is
// indicated by a decimal integer in a second argument field of
// the command.  This second argument is optional, but when
// present should be separated from the first by the three
// Telnet characters <SP> R <SP>.  This command shall be
// followed by a STORe or APPEnd command.  The ALLO command
// should be treated as a NOOP (no operation) by those servers
// which do not require that the maximum size of the file be
// declared beforehand, and those servers interested in only
// the maximum record or page size should accept a dummy value
// in the first argument and ignore it.

use crate::server::commands::Cmd;
use crate::server::error::FTPError;
use crate::server::reply::{Reply, ReplyCode};
use crate::server::CommandArgs;
use crate::storage;

pub struct Allo;

impl<S, U> Cmd<S, U> for Allo
where
    U: Send + Sync + 'static,
    S: 'static + storage::StorageBackend<U> + Sync + Send,
    S::File: tokio_io::AsyncRead + Send,
    S::Metadata: storage::Metadata,
{
    fn execute(&self, _args: &CommandArgs<S, U>) -> Result<Reply, FTPError> {
        // ALLO is obsolete and we'll just ignore it.
        Ok(Reply::new(ReplyCode::CommandOkayNotImplemented, "Ignored"))
    }
}
