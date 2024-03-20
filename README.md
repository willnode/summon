
# HTTP Summon App NGINX Module

A simple NGINX module to launch any program and forward HTTP requests to it.

Not yet ready for production. Primilarity built for [DOM Cloud](https://domcloud.co).

## Installation and Running Demo

```sh
make nginx-install
make build
make run
```

## Configuration

#### `summon_app_command "<cmd>"`

**Required to activate summon module.**

Run app at `<cmd>`. App must listen to `PORT` envar.

Use `proxy_pass http://localhost:$summon_port` to forward HTTP requests.

#### `summon_app_root <path>`

Set app pwd at `<path>`. Defaults to parent of `root`.

#### `summon_app_user <user>`

Set app to run under user. Defaults to who owned the "file" found in args or `nobody`.

#### `summon_app_log <path>`

Set app log to path. Defaults log to NGINX error.

#### `summon_app_idle_timeout <duration>`

Set app maximum time before getting killed because of idle traffic. Defaults 15 minutes. Set 0 to run indefinitely.

#### `summon_app_min_instance <number>`

Spawn app at many times. Default is 1.

#### `summon_app_use_port <number>`

Spawn app using reserved port. Will try kill any app listening to port before hand.
If `summon_app_min_instance` > 1 then app must spawn with `SO_REUSEADDR`.

If `summon_app_use_port` uses same number, test config should fail.

#### `summon_app_show_crash <on|off>`

Show helpful diagnostic error information if app is crashing either at startup or in middle of request.
