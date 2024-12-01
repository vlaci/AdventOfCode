# SPDX-FileCopyrightText: 2024 László Vaskó <vlaci@fastmail.com>
#
# SPDX-License-Identifier: EUPL-1.2

{
  git-hooks.hooks = {
    clippy.enable = true;
    deadnix.enable = true;
    nixfmt-rfc-style.enable = true;
    reuse.enable = true;
    statix.enable = true;
  };

  languages.rust.enable = true;
}
