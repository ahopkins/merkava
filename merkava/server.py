import trio


class DummyContext:
    async def __aenter__(self):
        pass

    async def __aexit__(self, *args, **kwargs):
        pass


class Server:
    attached_services = []
    context = None

    def attach(self, method, *args, **kwargs):
        self.attached_services.append((method, args, kwargs,))

    async def get_context(self):
        if self.context is not None:
            return await self.context
        return DummyContext()

    async def service(self):
        context = await self.get_context()
        async with context:
            async with trio.open_nursery() as nursery:
                for service, args, kwargs in self.attached_services:
                    nursery.start_soon(service, context, *args, **kwargs)

    def start(self):
        print('starting')
        trio.run(self.service)

    @classmethod
    def close(cls):
        print('\nClosing connection')
        return

    @classmethod
    def run(cls, *args, **kwargs):
        try:
            cls(*args, **kwargs).start()
        except KeyboardInterrupt:
            cls.close()
