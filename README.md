# fedora-coreos-metrics-client

Client-side service for metrics collection and reporting in the Fedora CoreOS
distribution of Linux.

The goal of this project is to collect information about the OS, such as the
Fedora CoreOS version running and the platform used. Having this information
(in an aggregated form) helps developers of Fedora CoreOS get a clearer picture
of the user base, and know which platforms, use cases, architectures, etc. to
prioritize for development and support. For more context, please see the
[fedora-coreos-tracker ticket](https://github.com/coreos/fedora-coreos-tracker/issues/86)
where this is discussed.

Currently, the `fedora-coreos-metrics-client` binary is a stub that only parses
config snippets to check that the config is correct. The reason for shipping a
stub first is to allow users to provide a valid configuration for the client
with their desired settings (including disabling metrics reporting) during the
Fedora CoreOS preview period. Then, once functionality for transmitting
collected data is later added, the configuration specified by the user will
remain.

## Configuration

The client is configured by dropping configuration fragments (files) of TOML
format in the following directories:

```
/etc/fedora-coreos-metrics-client/config.d/
/run/fedora-coreos-metrics-client/config.d/
/usr/lib/fedora-coreos-metrics-client/config.d/
```

Files in `/etc/fedora-coreos-metrics-client/config.d/` override files of the
same name in `/run/fedora-coreos-metrics-client/config.d/`, which override
files of the same name in `/usr/lib/fedora-coreos-metrics-client/config.d/`.
Config files are read in alphanumeric order; config files ordered later
override config files ordered earlier.

The metrics reporting `enabled` flag must be explicitly set by a config file.
If not specified, the service will exit with error. If metrics reporting is
enabled, then by default the level of information collected is set to
`"minimal"`. An example of the default config is as follows:

```TOML
# /usr/lib/fedora-coreos-metrics-client/config.d/0000-client-default.toml
# fedora-coreos-metrics client configuration

[collecting]
# Default collecting.level is `minimal`. May be set to `"minimal"` or `"full"`.
level = "minimal"

[reporting]
# Required. May be set to `true` or `false`.
enabled = true

```

To disable metrics reporting, a config snippet containing the following can be
dropped at e.g.
`/etc/fedora-coreos-metrics-client/config.d/9000-client-disable-reporting.toml`.

```TOML
[reporting]
enabled = false

```

With reporting disabled, no information is collected nor transmitted by the
client.

Once installed, the systemd service unit running to collect the metrics is
`fedora-coreos-metrics-client.service`.

Note that until a stable release of Fedora CoreOS has been made, the above
config format may change freely during development. In the case of a format
change, the metrics client service will fail if an incorrect config format is
given.

## Development

To build:

```
cargo build
```

To run:

```
cargo run
```
