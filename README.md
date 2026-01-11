# pathfinder
Network topology mapper.

## Index

## Permissions
Some plugins may require admin privileges, when running a plugin with this requirement a dialog will be showed asking permission.
If you are on Linux make sure `pkexec` is installed.


## Connect to a GNS3 network
Create interface to connect host pc to a GNS3 network:
```sh
sudo ip tuntap add name tap0 mode tap
sudo ip addr add 10.1.1.10/24 dev tap0
sudo ip link set dev tap0 up
```