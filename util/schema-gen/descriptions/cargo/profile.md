Profiles provide a way to alter the compiler settings, influencing things like optimizations and debugging symbols.

Cargo has 4 built-in profiles: dev, release, test, and bench. It automatically chooses the profile based on which command is being run, the package and target that is being built, and command-line flags like --release.