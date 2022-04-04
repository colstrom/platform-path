# platform-path

## What is this?

`platform-path` is a small tool (and library) that tells you (or your programs) where to put things, according to whatever standard applies to the system you're running it on.

## Why does this exist?

Because it encodes domain-specific knowledge in a scriptable tool. It knows the standards, so you don't have to.

Every platform has some sort of standard(ish) set of paths where it expects things to be. Conforming to improves user experience, and OS integration.

As an example, on macOS, caches are excluded from iCloud backups, Spotlight searches, can be purged automatically when disk space is very low (when the app is not running), and should be candidate for the "Optimize Storage" utility.

On Linux, applications installed with FlatPak or AppImage may be sandboxed with application-specific paths. If your application relies on a fixed path, it may not be writable (or even readable) depending on sandbox policies.

On Windows, Roaming Profiles may use different data paths than Local Profiles.

These are just a few examples.

`platform-path` can tell you where to put things.

## How do I get this majestic tool?

```
cargo install platform-path
```

## How do I use it?

Options can be provided via environment or commandline. These examples assume a user on macOS named "DemoUser", with the following environment:

```
PROJECT_QUALIFIER="com.suse"
PROJECT_ORGANIZATION="SUSE Software Solutions"
```

What is the base directory for preferences?

```
$ platform-path print base preference

/Users/DemoUser/Library/Preferences
```

Where should [NiftyGate](https://github.com/colstrom/niftygate) put it's cache?

```
$ platform-path print project cache --project-application NiftyGate

/Users/DemoUser/Library/Caches/com.suse.SUSE-Software-Solutions.NiftyGate
```

Where should runtime files like sockets be stored?

```
platform-path print base runtime

Error: platform standard does not define requested directory
```

(error written to STDERR, exit status is non-zero)

Where is the user's Download folder?

```
$ platform-path print user download

/Users/DemoUser/Downloads
```

For a full list, consult the built-in help.

```
platform-path --help
```

## Optional Features

There are a few less conventional features that are not enabled by default.

### Structured Output

If, for some reason, you want the output in a structured format, you can build `platform-path` with the `json` and/or `yaml` features.

These will add some variants to the `--format` option.

### HTTP Service

If you need to access this information in a context where shell output is not ideal, you can build `platform-path` with the `http` or `https` features.

This will add a `serve` subcommand that exposes the information over HTTP(S).

## How does it work (internally)?

Internally, it relies on a series of excellent crates for all the complicated things, and provides a CLI around them.

- Commandline options and argument parsing are provided by `clap` (and `structopt`).
- The platform-specific standards knowledge is provided by `directories`.
- Unicode path validation is provided by `camino`.
- Structured output is provided by `serde` generally and format-specific crates (`serde_json`, and `serde_yaml`).
- The various failure scenarios are captured in a single `Error` type (which implements `std::error::Error`).
- The (optional) HTTP service is built using `tide`.
- For each way things can fail, an option exists in the CLI to handle it in a reasonable way, such as:
  - a `--default` option if a directory is not defined by the platform standards.
  - a `--unicode=enforced` option to coerce non-Unicode paths.

## License

`platform-path` is available under the MIT License. See `LICENSE.txt` for the full text.
