import asyncio
import pathfinder_py


async def main():
    plugin = pathfinder_py.Plugin()
    
    await asyncio.sleep(10)
    
    plugin.exit()


if __name__ == "__main__":
    asyncio.run(main())
