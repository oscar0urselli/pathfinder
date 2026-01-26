import asyncio
import pathfinder_py


async def main():
    plugin = pathfinder_py.Plugin()
    
    res = await plugin.query_raw_sql("SELECT * FROM reports;")
    print(res)
    
    res = await plugin.execute_raw_query("SELECT * FROM reports;")
    print(res)
    
    plugin.exit()


if __name__ == "__main__":
    asyncio.run(main())
