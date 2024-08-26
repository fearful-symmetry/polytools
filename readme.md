# Polytools

Polytools is a set of libraries and CLI tools for hacking on Polycom phones. This includes a small CLI utility, `polycli`, and a backend library, `libpoly`. 


## `polyci` Usage
```
Usage: polycli [OPTIONS] --pass <PASS> --url <URL> <COMMAND>

Commands:
  rest         Run REST commands
  push         Send a push notification to the device
  provisioner  Start up an HTTP file server server pointing to the specified path. If the Polycom device is configured to look for this server, it will use the config files at the given path to configure the device
  help         Print this message or the help of the given subcommand(s)

Options:
  -u, --user <USER>  Set the username for all queries [env: POLY_USER=Polycom] [default: Polycom]
  -p, --pass <PASS>  [env: POLY_PASS=789]
      --url <URL>    Set the device URL for all queries [env: POLY_URL=https://192.168.1.9]
  -h, --help         Print help
  -V, --version      Print version
```


## Configuring a Polycom device

In order to use most of the features here, you'll need to configure the Polycom phone for use with the Push/REST APIs. 
On the phone's web UI, navigate to `Settings` > Applications to enable and configure the services.

## Feature overview

`Polycli` (and by extension, `libpoly`) can send keypresses to the phone:

```
$ polycli push cmd key Home
```

As well as alerts:
```
$ polycli push alert "YUM" "I am so full of voip packets"
```

And HTML that will appear on the screen:

```
$ polycli push html '<h1>Some HTML</h1>'
```

As well as commands from the REST API:
```
polycli rest mgmt net-stats
NetworkStats {
    uptime: "0 day 4:40:18",
    rx_packets: "36559",
    tx_packets: "2097",
}
```

Note that the REST API is currently incomplete, and a work in progress.