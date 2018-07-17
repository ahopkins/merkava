from aionursery import Nursery
from dataclasses import dataclass
from time import time
from asyncio import PriorityQueue, sleep, get_event_loop, start_server
import commands
import msgpack
import os
from pathlib import Path
# import contextvars
from channel import Channel


DIR = '/var/lib/merkava/data'
HOST = '0.0.0.0'
PORT = 6363
SLEEP = 0.01
MAX_QUEUE_SIZE = 1024

loop = None
channels = {}
# channel_var = contextvars.ContextVar("channel")


@dataclass
class Message:
    channel_name: str
    command: str
    payload: str


def _get_loop():
    global loop
    if loop is None:
        loop = get_event_loop()
    return loop


def _decode(raw_message):
    if not raw_message:
        return None
    parts = raw_message.split(' ')
    if len(parts) != 3:
        return None

    return Message(*parts)


async def _intake(message: Message, queue: PriorityQueue) -> None:
    print(f'Message Received:\n\t{message}')
    t = hash(time() * 10_000_000)
    await queue.put((t, message))


async def _receiver(reader, queue):
    while True:
        data = await reader.read(100)
        message = _decode(data.decode())

        if not message:
            break

        await _intake(message, queue)
        await sleep(SLEEP)


async def _handler(queue):
    while True:
        while not queue.empty():
            t, message = await queue.get()

            command = message.command.lower()
            print(f'\tHandling {message.channel_name}:{command}')

            if hasattr(commands, command):
                ret = getattr(commands, command)()
                if ret is not None:
                    print(ret)
                    # await self.respond(stream, ret)
            else:

                channel = channels.get(message.channel_name)

                if not channel:
                    channel_name = message.channel_name.lower()
                    path = Path(DIR) / channel_name
                    channel = await Channel.open(channel_name, path)

                if hasattr(channel, command):
                    method = getattr(channel, command)
                    success, raw = await method(t, message.payload)
                    channels[channel_name] = channel

                    # if success:
                    #     os.remove(dump_file)

                    if raw:
                        print('Responding', raw)
                        # await self.respond(stream, raw)
                        # TODO:
                        # - Send response back to client
                else:
                    # Log
                    print(f'invalid command {command} on {channel}')
        await sleep(SLEEP)


async def _start_server(reader, writer):
    # data = await reader.read(100)
    # message = data.decode()
    # addr = writer.get_extra_info('peername')
    # print("Received %r from %r" % (message, addr))

    # print("Send: %r" % message)
    # writer.write(data)
    # await writer.drain()

    # print("Close the client socket")
    # writer.close()
    queue = PriorityQueue(maxsize=MAX_QUEUE_SIZE)
    async with Nursery() as nursery:
        nursery.start_soon(_receiver(reader, queue))
        nursery.start_soon(_handler(queue))


def starter():
    loop = _get_loop()
    coro = start_server(_start_server, HOST, PORT, loop=loop)
    server = loop.run_until_complete(coro)

    # Serve requests until Ctrl+C is pressed
    print('\n\nMerkava DB\n')
    print('\nServing on {}\n'.format(server.sockets[0].getsockname()))
    try:
        loop.run_forever()
    except KeyboardInterrupt:
        pass

    # Close the server
    server.close()
    loop.run_until_complete(server.wait_closed())
    loop.close()
