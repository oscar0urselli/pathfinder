import asyncio
import pathfinder_py


async def main():
    plugin = pathfinder_py.Plugin()
    
    #await asyncio.sleep(10)
    """plugin.add_net_node({
        "name": "Node 1",
        "type": "Unknown",
        "interfaces": {},
        "services": []
    })
    
    plugin.add_net_node({
        "name": "Node 2",
        "type": "Unknown",
        "interfaces": {},
        "services": []
    })"""
    
    graph = await plugin.get_net_graph()
    
    #if len(graph["nodes"]) >= 2:
    #    plugin.add_net_edge(0, 1)
    
    #if len(graph["nodes"]) > 1:
    #    plugin.remove_net_node(0)
    
    #if len(graph["edges"]) >= 1:
    #    plugin.remove_net_edge(0)
    
    if len(graph["nodes"]) >= 1:
        plugin.update_net_node(0, {
            "name": "HTTP SERVER",
            "type": "Unknown",
            "interfaces": {},
            "services": []
        })
    
    plugin.exit()


if __name__ == "__main__":
    asyncio.run(main())
