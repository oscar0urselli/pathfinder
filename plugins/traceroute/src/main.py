import asyncio
import json
import pathfinder_py
import scapy.all as scapy
from sqlglot import exp


async def main():
    plugin = pathfinder_py.Plugin()
    
    params = await plugin.form("Traceroute settings", [
        [
            {
                "name": "protocol",
                "title": "Protocol",
                "type": "str",
                "options": ["TCP", "UDP", "ICMP"]
            },
            {
                "name": "dst",
                "title": "Destination",
                "type": "str"
            },
            {
                "name": "min-ttl",
                "title": "Min TTL",
                "type": "int",
                "default": 1
            },
            {
                "name": "max-ttl",
                "title": "Max TTL",
                "type": "int",
                "default": 20
            }
        ]
    ])
    
    print(params)
    
    ans = []
    if params["protocol"] == "TCP":
        ans, unans = scapy.sr(scapy.IP(dst=params["dst"], ttl=(params["min-ttl"], params["max-ttl"])) / scapy.TCP(dport=53, flags="S"))
    elif params["protocol"] == "UDP":
        ans, unans = scapy.sr(scapy.IP(dst=params["dst"], ttl=(params["min-ttl"], params["max-ttl"])) / scapy.UDP() / scapy.DNS(qd=scapy.DNSQR(qname="example.com")))
    elif params["protocol"] == "ICMP":
        ans, unans = scapy.sr(scapy.IP(dst=params["dst"], ttl=(params["min-ttl"], params["max-ttl"])) / scapy.ICMP())
    
    routes = []
    for i in ans:
        routes.append(i[1].src)
        
    print(routes)
    
    await plugin.execute_raw_query("CREATE TABLE IF NOT EXISTS traceroute (id UUID PRIMARY KEY, report UUID, protocol STRING, dst STRING, min_ttl UINT64, max_ttl UINT64, routes STRING);")
    
    sql = exp.Insert(
        this=exp.Table(this=exp.Identifier(this="traceroute")),
        expression=exp.Values(
            expressions=[
                exp.Tuple(
                    expressions=[
                        exp.func("uuidv7"),
                        exp.Null() if plugin.report is None else exp.Literal.string(plugin.report),
                        exp.Literal.string(params["protocol"]),
                        exp.Literal.string(params["dst"]),
                        exp.Literal.number(params["min-ttl"]),
                        exp.Literal.number(params["max-ttl"]),
                        exp.Literal.string(json.dumps(routes))
                    ]
                )
            ]
        ),
        columns=[
            exp.Identifier(this="id"),
            exp.Identifier(this="report"),
            exp.Identifier(this="protocol"),
            exp.Identifier(this="dst"),
            exp.Identifier(this="min_ttl"),
            exp.Identifier(this="max_ttl"),
            exp.Identifier(this="routes")
        ]
    ).sql()
    
    await plugin.execute_raw_query(sql)
    
    await plugin.toast(pathfinder_py.Plugin.ToastType.SUCCESS, "Traceroute scan completed.")
    
    # https://scapy.readthedocs.io/en/latest/usage.html#index-9
    # https://scapy.readthedocs.io/en/latest/usage.html#index-18
    # https://jvns.ca/blog/2013/10/31/day-20-scapy-and-traceroute/
    
    plugin.exit()


if __name__ == "__main__":
    asyncio.run(main())
