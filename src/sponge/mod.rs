// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

mod sponge;

#[cfg(feature = "std")]
pub use sponge::sponge_gadget as gadget;
pub use sponge::sponge_hash as hash;
