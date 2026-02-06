Mirrored from https://gitlab.com/markwetter/mc-wrap

# mc-wrap

A lightweight wrapper for Minecraft Server running in Docker. Intercepts SIGINT signals from the environment and sends a graceful shutdown command to the child server process.

## Usage

Invoke mc-wrap before your normal Minecraft Server startup command.

```
mc-wrap java -jar server.jar
```
You can still pass JVM arguments.
```
mc-wrap java \
    -Xms2G -Xmx2G \
    -XX:+UseG1GC \
    -XX:+UnlockExperimentalVMOptions \
    -XX:MaxGCPauseMillis=100 \
    -XX:+DisableExplicitGC \
    -XX:TargetSurvivorRatio=90 \
    -XX:G1NewSizePercent=50 \
    -XX:G1MaxNewSizePercent=80 \
    -XX:InitiatingHeapOccupancyPercent=10 \
    -XX:G1MixedGCLiveThresholdPercent=50 \
    -XX:+AggressiveOpts \
    -XX:+AlwaysPreTouch \
    -jar server.jar
```

## Compiling from source

You will need the Rust language build tools.

https://www.rust-lang.org/en-US/install.html

```
git clone git@gitlab.com:markwetter/mc-wrap.git
cd mc-wrap
cargo build --release
```

## Authors
* **Mark Wetter** - [Gitlab](https://gitlab.com/markwetter)

## Acknowledgments

* **Geoff Bourne** - *I was inspired by Geoff's mc-server-runner that I found while researching my problem with shutting down Minecraft gracefully from inside a docker container* [Github](https://github.com/itzg/mc-server-runner)

## License

mc-wrap is distributed under the terms of the MIT license

See LICENSE for details.
