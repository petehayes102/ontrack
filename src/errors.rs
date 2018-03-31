// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use std::io;
use std::sync::mpsc;
use toml::de;

error_chain! {
    foreign_links {
        Io(io::Error);
        RecvError(mpsc::RecvError);
        SendBoolError(mpsc::SendError<bool>);
        SendU8Error(mpsc::SendError<u8>);
        TomlDeserialize(de::Error);
    }
}
