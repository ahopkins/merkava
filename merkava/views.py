from sanic import views, response
from merkava.channels import Channel


class BaseChannelView(views.HTTPMethodView):

    def set_channel(self, request, channel_name):
        self.channel = Channel(
            channel_name=channel_name,
            data_path=request.app.config.get('path', './'),
        )

    # service.add_task(self.channel.listen())

    def dispatch_request(self, request, *args, **kwargs):
        self.set_channel(request, kwargs.get('channel'))
        return super().dispatch_request(request, *args, **kwargs)


class ChannelView(BaseChannelView):

    async def post(self, request, channel, id=None):
        # TODO:
        # - make the CRUD operations async/await
        status, result = self.channel.create(request.json)
        return response.json({'result': result}, status=status)

    async def get(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        status, result = self.channel.retrieve(id)
        return response.json(result, status=status)

    async def patch(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        status, result = self.channel.update(id, request.json)
        return response.json(result, status=status)

    async def delete(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        status, result = self.channel.delete(id)
        return response.json(None, status=status)

    async def put(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        status, result = self.channel.restore(id)
        return response.json(result, status=status)


class RecentChannelView(BaseChannelView):

    async def get(self, request, channel, num=None):
        if not num:
            num = 5
        num = int(num)
        result = self.channel.recent(int(num))
        return response.json(result)
