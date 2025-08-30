from rpc_model import PyRpcAPIClient
import asyncio


async def main():
    stub = await PyRpcAPIClient.connect("127.0.0.1:5000")
    response = await stub.hello("Diego")
    print(response)

    response = await stub.sum_numbers(1, 2)
    print(response)


asyncio.run(main())

