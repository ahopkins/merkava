import msgpack
import os
import trio
import commands

from channel import Channel
from collections import deque
from server import Server
from time import time


PORT = 6363
BUFSIZE = 16384
DIR = '/var/lib/merkava/data'
SLEEP_TIME = 0.01

packet_schema = {
    'channel': str,
    'command': str,
    'payload': object,
}


class Receiver(Server):
    def __init__(self, *args, **kwargs):
        self.attach(self.incoming)
        self.attach(self.handle)
        self.connections = {}
        self.q = deque()

    async def incoming(self, *args):
        """
        Listen to incoming requests from a client
        """
        await trio.serve_tcp(self._receive, PORT)

    async def respond(self, stream, raw):
        msg = msgpack.packb(raw)
        await stream.send_all(msg)

    async def handle(self, *args):
        while True:
            if len(self.q) > 0:
                t, packet, stream = self.q.popleft()
                dump_file = trio.Path(DIR) / f'{t}.pkt'
                print('processing', t)
                message = self.unpack(packet)

                if message:
                    channel_name = message.get('channel').lower()
                    command = message.get('command').lower()
                    payload = message.get('payload')

                    # TODO:
                    # - Optimize special command calling
                    if hasattr(commands, command):
                        getattr(commands, command)()
                    else:
                        # TODO:
                        # - Only open the channel if NOT in a cache
                        # - If channel is not open, then open it and put it in the cache
                        # - Spawn a new task that will poll for the index, and set next index on the channel
                        # - On connect, keep track of the number of connections
                        # - Close channel when the number of connections goes to 0
                        channel = await Channel.open(channel_name, trio.Path(DIR) / channel_name)

                        if hasattr(channel, command):
                            method = getattr(channel, command)
                            success, raw = await method(t, payload)

                            if success:
                                os.remove(dump_file)

                            if raw:
                                print('Responding', raw)
                                await self.respond(stream, raw)
                                # TODO:
                                # - Send response back to client
                        else:
                            # Log
                            print('invalid command')
                else:
                    # Log error - invalid packet
                    print('invalid packet')
            await trio.sleep(SLEEP_TIME)

    async def _receive(self, stream):
        while True:
            packet = await stream.receive_some(BUFSIZE)
            print(f'received {packet}')

            if not packet:
                return self._close_connection()

            t = hash(time() * 10_000_000)
            dump_file = trio.Path(DIR) / f'{t}.pkt'
            # with open(dump_file, 'wb') as f:
            async with await trio.open_file(dump_file, 'wb') as f:
                await f.write(packet)

            self.q.append((t, packet, stream))

            await trio.sleep(SLEEP_TIME)

    def _close_connection(self):
        print('_close_connection')
        return

    @staticmethod
    def unpack(packet):
        # TODO:
        # - better exception handling
        try:
            data = msgpack.unpackb(packet, use_list=False, raw=False)
        except Exception as e:
            print(f'Error: {e}')
            return None

        check_keys = all(x in data for x in packet_schema.keys())
        check_values = all(isinstance(data.get(key), value)
                           for key, value in packet_schema.items())

        if check_keys and check_values:
            return data
        return None
