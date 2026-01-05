Switch1:
- PC2
- PC3
- PC11
- Cloud1

Switch2:
- PC4
- PC5
- PC6


VyOS Router:
```
# Enter configuration mode
configure

# Create a bridge interface
set interfaces bridge br0

# Add both switch interfaces to the bridge
set interfaces bridge br0 member interface eth1
set interfaces bridge br0 member interface eth2

# Assign an IP address to the bridge (this becomes your gateway)
set interfaces bridge br0 address 192.168.1.1/24

# Commit and save
commit
save
exit
```

Activate SNMP:
```
# Define a community
set service snmp community routers authorization ro

# Allow monitoring access from the entire network
set service snmp community routers network 192.0.2.0/24
set service snmp community routers network 2001::db8:ffff:eeee::/64

# Allow monitoring access from specific addresses
set service snmp community routers client 203.0.113.10
set service snmp community routers client 203.0.113.20

# Define optional router information
set service snmp location "UK, London"
set service snmp contact "admin@example.com"

# Trap target if you want asynchronous communication
set service snmp trap-target 203.0.113.10

# Listen only on specific IP addresses (port defaults to 161)
set service snmp listen-address 172.16.254.36 port 161
set service snmp listen-address 2001:db8::f00::1
```