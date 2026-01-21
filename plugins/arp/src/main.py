import asyncio
import json
import pathfinder_py
import scapy.all as scapy
from sqlglot import exp


async def main():
    plugin = pathfinder_py.Plugin()
    
    await plugin.toast(pathfinder_py.Plugin.ToastType.INFO, "Plugin running.")

    params = await plugin.form([
        [
            {
                "name": "interface",
                "title": "Interface",
                "type": "str",
                "options": [i for i in scapy.conf.ifaces]
            },
            {
                "name": "network",
                "title": "Network",
                "type": "ipv4_cidr"
            },
            {
                "name": "timeout",
                "title": "Timeout",
                "type": "int",
                "default": 2
            },
            {
                "name": "dst_mac",
                "title": "Destination MAC",
                "type": "mac",
                "default": "ff:ff:ff:ff:ff:ff"
            }
        ]
    ])
    
    plugin.add_net_node({
        "name": scapy.conf.route.route(params["network"])[2],
        "type": "Unknown",
        "interfaces": {},
        "services": []
    })
    
    arp_request = scapy.ARP(pdst=params["network"])
    br = scapy.Ether(dst=params["dst_mac"])
    request = br / arp_request
    
    answered, unanswered = scapy.srp(request, iface=params["interface"], timeout=params["timeout"])
    scans = []
    for i in answered:
        plugin.add_net_node({
            "name": i[1].psrc,
            "type": "Unknown",
            "interfaces": {},
            "services": []
        })
        
        net_graph = await plugin.get_net_graph()
        for n_index, n in enumerate(net_graph["nodes"]):
            if n["name"] == i[1].psrc:
                plugin.add_net_edge(0, n_index)
                break
        
        scans.append({
            "ipv4": i[1].psrc,
            "mac": i[1].hwsrc
        })
        
    await plugin.execute_raw_query("CREATE TABLE IF NOT EXISTS arp (id UUID PRIMARY KEY, report UUID, arp_count UINT64, duration_ms UINT64, packet_count UINT64, interface STRING, network STRING, timeout UINT64, dst_mac STRING, scans STRING);")
    
    sql = exp.Insert(
        this=exp.Table(this=exp.Identifier(this="arp")),
        expression=exp.Values(
            expressions=[
                exp.Tuple(
                    expressions=[
                        exp.func("uuidv7"),
                        exp.Null() if plugin.report is None else exp.Literal.string(plugin.report),
                        exp.Literal.number(0),
                        exp.Literal.number(0),
                        exp.Literal.number(0),
                        exp.Literal.string(params["interface"]),
                        exp.Literal.string(params["network"]),
                        exp.Literal.number(params["timeout"]),
                        exp.Literal.string(params["dst_mac"]),
                        exp.Literal.string(json.dumps(scans))
                    ]
                )
            ]
        ),
        columns=[
            exp.Identifier(this="id"),
            exp.Identifier(this="report"),
            exp.Identifier(this="arp_count"),
            exp.Identifier(this="duration_ms"),
            exp.Identifier(this="packet_count"),
            exp.Identifier(this="interface"),
            exp.Identifier(this="network"),
            exp.Identifier(this="timeout"),
            exp.Identifier(this="dst_mac"),
            exp.Identifier(this="scans")
        ]
    ).sql()
    
    await plugin.execute_raw_query(sql)
    
    await plugin.toast(pathfinder_py.Plugin.ToastType.SUCCESS, "ARP scan completed.")
    
    plugin.exit()


if __name__ == "__main__":
    asyncio.run(main())
