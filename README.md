# pathfinder
Network topology mapper.


It is necessary to run the program as root or grant CAP_NET_RAW capabilities in order for all the features to work properly.


Create interface to connect host pc to GNS3 network:
```sh
sudo ip tuntap add name tap0 mode tap
sudo ip addr add 10.1.1.10/24 dev tap0
sudo ip link set dev tap0 up
```