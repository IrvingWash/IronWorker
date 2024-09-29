## blacksmith

a simple cli tool to manually scrobble tracks and albums to lastfm

### installation
```bash
cargo install blacksmith
```

### usage
authenticate:
```bash
blacksmith auth
```

list recent tracks:
```bash
blacksmith list
```

scrobble a track:
```bash
blacksmith scrobble-track Krallice "Ygg Huur" "Over Spirit"
```

scrobble an album:
```bash
blacksmith scrobble-album Orgone "The Goliath"
```

for more:
```bash
blacksmith --help
```
