# fedora-coreos-pinger

Telemetry service in Fedora CoreOS.

The goal of this project is to provide better insight to Fedora CoreOS
developers on how the distribution is used, by collecting information such
as the Fedora CoreOS version running and the platform used. Having this
information (in an aggregated form) helps give a clearer picture of the
user base, and know which platforms, use cases, architectures, etc. to
prioritize for development and support. For more context, please see the
[fedora-coreos-tracker ticket](https://github.com/coreos/fedora-coreos-tracker/issues/86)
where this is discussed.

Currently, the `fedora-coreos-pinger` binary is a stub that only parses
config snippets to check that the config is correct. The reason for shipping a
stub first is to allow users to provide a valid configuration for the pinger
with their desired settings (including disabling information reporting) during
the Fedora CoreOS preview period. Then, once functionality for transmitting
collected data is later added, the configuration specified by the user will
remain.

By default, reporting is enabled and information is collected at a minimal
level. Since this service is only a stub that does not have reporting
implemented, details of what the minimal level entails are still being
discussed in the above linked tracker ticket. Reporting can be disabled
completely before it is implemented, see [disabling reporting](#Disabling-reporting).

## Configuration

The pinger service is configured by dropping configuration fragments (files) of
TOML format in the following directories:

```
/etc/fedora-coreos-pinger/config.d/
/run/fedora-coreos-pinger/config.d/
/usr/lib/fedora-coreos-pinger/config.d/
```

Files in `/etc/fedora-coreos-pinger/config.d/` override files of the
same name in `/run/fedora-coreos-pinger/config.d/`, which override
files of the same name in `/usr/lib/fedora-coreos-pinger/config.d/`.
Config files are read in alphanumeric order; config files ordered later
override config files ordered earlier.

The reporting `enabled` flag must be explicitly set by a config file. If not
specified, the service will exit with error. If reporting is enabled, then by
default the level of information collected is set to `"minimal"`. An example of
a config is as follows:

```TOML
# /usr/lib/fedora-coreos-pinger/config.d/10-default-enable.toml
# fedora-coreos-pinger configuration

[collecting]
# Default collecting.level is `minimal`. May be set to `"minimal"` or `"full"`.
level = "minimal"

[reporting]
# Required. May be set to `true` or `false`.
enabled = true

```

### Disabling reporting

To disable information reporting, a config snippet containing the following can
be dropped at e.g.
`/etc/fedora-coreos-pinger/config.d/99-disable-reporting.toml`.

```TOML
[reporting]
enabled = false

```

With reporting disabled, no information is collected nor transmitted by the
pinger.

Once installed, `fedora-coreos-pinger` is run automatically by enabling the
`fedora-coreos-pinger.service` unit.

Note that until a stable release of Fedora CoreOS has been made, the above
config format may change freely during development. In the case of a format
change, the pinger service will fail if an incorrect config format is given.

## Development

To build and run, see the [Cargo command reference](https://doc.rust-lang.org/cargo/commands/index.html).

When testing the `fedora-coreos-pinger.service` unit, the files in the `dist` directory should be installed.
