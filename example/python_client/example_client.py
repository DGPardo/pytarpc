from rpc_model import PyRpcAPIClient, CustomDataType
import asyncio


async def main():
    stub = await PyRpcAPIClient.connect("0.0.0.0:5000")
    response = await stub.hello("Diego")
    print(f'stub.hello("Diego"): {response}')

    response = await stub.sum_numbers(1, 2)
    print(f"stub.sum_numbers(1, 2): {response}")

    foo = CustomDataType(field1="Diego", field2=[0.0, 1.0, 2.0])
    response: CustomDataType = await stub.echo(foo)
    print(f"stub.echo(CustomDataType(Diego, [0., 1., 2.])) = {response}")


asyncio.run(main())
