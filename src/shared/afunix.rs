// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Definitions for the AF_UNIX socket address family.
use shared::ws2def::{ADDRESS_FAMILY, IOC_VENDOR};
use shared::minwindef::DWORD;
use um::winnt::CHAR;
pub const UNIX_PATH_MAX: usize = 108;
STRUCT!{struct SOCKADDR_UN {
    sun_family: ADDRESS_FAMILY,
    sun_path: [CHAR; UNIX_PATH_MAX],
}}
pub const SIO_AF_UNIX_GETPEERPID: DWORD = _WSAIOR!(IOC_VENDOR, 256);
