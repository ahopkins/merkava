from aionursery import Nursery
import asyncio
# import uvloop

# asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())

HOST = '0.0.0.0'
PORT = 6363

class Receiver:

    def __init__(self):
        self.intake = asyncio.PriorityQueue()
        self.loop = asyncio.get_event_loop()

    async def _intake(self):
        print('intake')

    async def _process(self):
        print('_process')
        
    async def _run(self):
        async with Nursery() as nursery:
            nursery.start_soon(self._intake())
            nursery.start_soon(self._process())
    
    def run(self):
        starter = asyncio.start_server(
            self._run, HOST, PORT, loop=self.loop)
        self.server = loop.run_until_complete(starter)