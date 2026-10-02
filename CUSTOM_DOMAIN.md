# Custom domain support

This fork adds authenticated custom-domain assignment while keeping the original random-domain flow compatible with existing clients.

## Relay configuration

Set the normal QUIClime variables plus:

```text
QUICLIME_CUSTOM_DOMAIN_TOKEN=replace-with-a-long-random-secret
QUICLIME_CUSTOM_DOMAIN_SUFFIXES=wolfetraincloud.xyz
```

Multiple suffixes can be comma-separated. A requested hostname must equal one configured suffix or be a subdomain of one.

Example:

```text
QUICLIME_BASE_DOMAIN=relay.wolfetraincloud.xyz
QUICLIME_BIND_ADDR_MC=[::]:25565
QUICLIME_BIND_ADDR_QUIC=[::]:25575
QUICLIME_BIND_ADDR_WEB=127.0.0.1:8080
QUICLIME_CERT_PATH=/path/to/fullchain.pem
QUICLIME_KEY_PATH=/path/to/privkey.pem
QUICLIME_CUSTOM_DOMAIN_TOKEN=replace-with-a-long-random-secret
QUICLIME_CUSTOM_DOMAIN_SUFFIXES=wolfetraincloud.xyz
```

The relay's QUIC certificate must be valid for the hostname configured as `relayHost` in the Minecraft mod.

## Minecraft mod configuration

Use the matching custom-domain e4mc fork and configure:

```text
useBroker = false
relayHost = "relay.wolfetraincloud.xyz"
relayPort = 25575
customDomain = "fabric-1.21.10.wolfetraincloud.xyz"
customDomainToken = "replace-with-the-same-secret"
```

Leaving `customDomain` blank keeps the original random-domain behavior.

## DNS

The public Minecraft hostname must resolve to the public IP of this QUIClime relay. For a default Minecraft TCP port, an A/AAAA record is sufficient. An SRV record may be used when the public Minecraft listener is on a non-default port.

The relay validates the Minecraft handshake hostname against its active routing table. This is why simply pointing a custom DNS name at the stock public e4mc relay does not work; the custom hostname must be registered by this relay fork.
