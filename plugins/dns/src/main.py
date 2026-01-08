import asyncio
import json
import pathfinder_py
import dns.resolver
from sqlglot import exp


async def main():
    plugin = pathfinder_py.Plugin()
    
    await plugin.toast(pathfinder_py.Plugin.ToastType.INFO, "Plugin running.")

    params = await plugin.form({
        "host": {
            "title": "Host",
            "type": "ipv4"
        },
        "port": {
            "title": "Port",
            "type": "int",
            "min": 1,
            "max": 65535,
            "default": 53
        },
        "protocol": {
            "title": "Protocol",
            "type": "str",
            "options": ["TCP", "UDP", "TLS", "HTTPS"],
            "default": "TCP"
        },
        "domain": {
            "title": "Domain",
            "type": "str"
        },
        "a": {
            "title": "A",
            "type": "bool",
            "default": True
        },
        "aaaa": {
            "title": "AAAA",
            "type": "bool",
            "default": True
        },
        "caa": {
            "title": "CAA",
            "type": "bool",
            "default": True
        },
        "cname": {
            "title": "CNAME",
            "type": "bool",
            "default": True
        },
        "ptr": {
            "title": "PTR",
            "type": "bool",
            "default": True
        },
        "mx": {
            "title": "MX",
            "type": "bool",
            "default": True
        },
        "ns": {
            "title": "NS",
            "type": "bool",
            "default": True
        },
        "srv": {
            "title": "SRV",
            "type": "bool",
            "default": True
        },
        "txt": {
            "title": "TXT",
            "type": "bool",
            "default": True
        },
        "hinfo": {
            "title": "HINFO",
            "type": "bool",
            "default": True
        }
    })
    
    res = dns.resolver.make_resolver_at(params["host"], params["port"])
    
    records_type = []
    if params["a"]:
        records_type.append("A")
    if params["aaaa"]:
        records_type.append("AAAA")
    if params["caa"]:
        records_type.append("CAA")
    if params["cname"]:
        records_type.append("CNAME")
    if params["ptr"]:
        records_type.append("PTR")
    if params["mx"]:
        records_type.append("MX")
    if params["ns"]:
        records_type.append("NS")
    if params["srv"]:
        records_type.append("SRV")
    if params["txt"]:
        records_type.append("TXT")
    if params["hinfo"]:
        records_type.append("HINFO")
    
    records = []
    for r in records_type:
        answer = res.resolve(params["domain"], r, raise_on_no_answer=False)
    
        for rr in answer:
            records.append({
                "name": answer.qname.to_text(),
                "type": answer.rdtype.name,
                "class": answer.rdclass.name,
                "data": rr.to_text()
            })
    
    await plugin.execute_raw_query("CREATE TABLE IF NOT EXISTS dns (id UUID PRIMARY KEY, report UUID, host STRING, port UINT16, protocol STRING, domain STRING, records STRING);")

    sql = exp.Insert(
        this=exp.Table(this=exp.Identifier(this="dns")),
        expression=exp.Values(
            expressions=[
                exp.Tuple(
                    expressions=[
                        exp.func("uuidv7"),
                        exp.Null() if plugin.report is None else exp.Literal.string(plugin.report),
                        exp.Literal.string(params["host"]),
                        exp.Literal.number(params["port"]),
                        exp.Literal.string(params["protocol"]),
                        exp.Literal.string(params["domain"]),
                        exp.Literal.string(json.dumps(records))
                    ]
                )
            ]
        ),
        columns=[
            exp.Identifier(this="id"),
            exp.Identifier(this="report"),
            exp.Identifier(this="host"),
            exp.Identifier(this="port"),
            exp.Identifier(this="protocol"),
            exp.Identifier(this="domain"),
            exp.Identifier(this="records")
        ]
    ).sql()
    
    await plugin.execute_raw_query(sql)
    
    await plugin.toast(pathfinder_py.Plugin.ToastType.SUCCESS, "DNS queries completed.")
    
    plugin.exit()


if __name__ == "__main__":
    asyncio.run(main())
