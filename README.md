# Consul Registrar

A tiny app which allows to register a service 
via the consul agent

## Usage
Env vars:

| Name | Example | Default value | Description |
| --- | --- | --- | --- |
| CONSUL_URL | http://consul:8500 | http://localhost:8500 |  Consul agent url |
| REG_SERVICE_NAME | `Srv1` | - | Service name to register |
| REG_INSTANCE_NAME | `Srv1_1` | - | Name of the instance of registered service, which will be used as id |
| REG_INSTANCE_ADDR | `service_1`/`192.168.123.123` | - | Dns name or address for usage within the network |
| REG_INSTANCE_PORT | 8080 | - | instance port |
| REG_INSTANCE_TAGS | "foo, bar, buzz" | "" | Instance Tags |
| REG_INSTANCE_META | "foo = bar, bar=buzz" | "" | Instance meta (additional kv information) |
| REG_HEALTHCHECK_DEREGISTER_AFTER | `2m` | "" | time in minutes, when service will be deregistered on unhealthy state |
| REG_HEALTHCHECK_URL | http://minio:9000/minio/health/live | "" | |
| REG_HEALTHCHECK_INTERVAL | 10s | 1m | How often to perform a healtcheck |
| REG_HEALTHCHECK_TIMEOUT | 30s | 5s | Duration after check will timeout |

Run locally:
```shell
    REG_SERVICE_NAME=test REG_INSTANCE_NAME=test1 cargo_registrar
```

Run with docker:
```shell
    docker run \
    -e REG_SERVICE_NAME=test \
    -e REG_INSTANCE_NAME=test1 \
    j0rsa/cargo-registrar
```

Testing:

    docker-compose up -d