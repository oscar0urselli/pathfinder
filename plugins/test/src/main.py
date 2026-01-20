import asyncio
import pathfinder_py


async def main():
    plugin = pathfinder_py.Plugin()
    
    #await asyncio.sleep(10)
    plugin.add_net_node({
        "name": "Node A",
        "type": "Unknown",
        "interfaces": {},
        "services": []
    })
    
    plugin.add_net_node({
        "name": "Node B",
        "type": "Unknown",
        "interfaces": {},
        "services": []
    })
    
    plugin.exit()


if __name__ == "__main__":
    asyncio.run(main())
