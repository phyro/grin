# Setting up monitoring with Grafana

## Run metrics tools

We use Telegraf, InfluxDB and Grafana to store and visualize metrics. In order to observe the
metrics we need to first run these services. All of them are described in `docker-compose.yaml` so we first need to run the services.
```bash
cd etc/metrics
docker-compose up -d
```

## Setup monitoring through Grafana

For custom metrics sending, we will be using StatsD as an input and we will output the metrics to InfluxDB. In order to show the metrics in Grafana we need to set InfluxDB as its data source.

Visit http://localhost:3000 and login with admin:admin.

On the left menu select Configuration and then `Data source` and then select Influxdb. On the setup page we set the following values:
- HTTP url: http://influxdb:8086
- Database: telegraf
- user: admin
- password: supersecretpassword
