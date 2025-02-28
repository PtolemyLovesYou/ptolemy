"""Setup script for Ptolemy. Use for local development only."""

import maturin_import_hook

# install the import hook with default settings.
# this call must be before any imports that you want the hook to be active for.
maturin_import_hook.install()
