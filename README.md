# pgm

----

## About

A command line tool that queries a pg_auto_failover monitor node to get various information about formations, status,
and events.

## Help menu

```
Usage: pgm [COMMAND]

Commands:
  add
  list
  show-state
  show-primaries
  show-secondaries
  list-events
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuration

Pgm relies on a configuration file to exist in `$HOME/.config`.
The configuration file is a yaml file that looks like the follow:

```
monitors:
- host: 127.0.0.1
  port: 5432
  user: postgres
  password: postgres
  database_name: pg_auto_failover
```

New monitors can be added by either directly editing the config file or by
using the pgm interface using the add subcommand.

```
Usage: pgm add [OPTIONS]

Options:
      --host <HOST>
      --port <PORT>
      --user <USER>
      --password <PASSWORD>
      --database-name <DATABASE_NAME>
  -h, --help                           Print help
```