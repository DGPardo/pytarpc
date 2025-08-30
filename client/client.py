from rpc_model import PyRpcClient
import asyncio


async def main():
    stub = await PyRpcClient.connect("127.0.0.1:5000")

    many_tasks = asyncio.gather(*[
        stub.hello(f"Diego + {i}")
        for i in range(10_000)
    ])
    await many_tasks


asyncio.run(main())

