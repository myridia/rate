# ![email_gateway](pages/public/img/logo.png) rate
* Myridia's online currency rate service


```
JPY
CZK
DKK
GBP
HUF
PLN
RON
SEK
CHF
ISK
NOK
AUD
BRL
CAD
CNY
HKD
IDR
ILS
INR
KRW
MXN
MYR
NZD
PHP
SGD
THB
ZAR
RUB
USD
EUR
```
# Demo 
https://mrate.myridia.com 


# How to Enable the mrate Server

This guide provides step-by-step instructions on how to enable and manage the `mrate` server using systemd.

## Create Service File

Create the service file for `mrate` at `/lib/systemd/system/mrate.service` with the following content:

```ini
[Unit]
Description=mrate Service
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=1
User=root
AmbientCapabilities=CAP_SYS_RAWIO
ExecStart=/usr/bin/env /var/customers/webs/mrate/mrate


[Install]
WantedBy=multi-user.target
```

## Test Service

Before enabling the service, verify the service file for any errors:

```bash
systemd-analyze verify mrate.service
```

## Enable Service

To enable the `mrate` service to start on boot, run the following commands:

```bash
systemctl enable mrate
systemctl daemon-reload
```

## Start Service

After enabling the service, you can start it with the following command:

```bash
systemctl start mrate
```

## Check Service Status

To check the status of the `mrate` service, use:

```bash
systemctl status mrate
```

## Stop Service

If you need to stop the `mrate` service, you can do so with:

```bash
systemctl stop mrate
```

## Restart Service

To restart the `mrate` service, use:

```bash
systemctl restart mrate
```

## Conclusion

You have successfully enabled and started the `mrate` server. For further management, you can use the `systemctl` commands as needed.

