from sanic import Sanic, Blueprint, response, views
from aoiklivereload import LiveReloader
import configparser
import sys
from channel import Channel


try:
    reloader = LiveReloader()
    reloader.start_watcher_thread()
except Exception:
    pass


service = Sanic()
bp = Blueprint('channels', url_prefix='/v1/<channel>')


class BaseChannelView(views.HTTPMethodView):
    def set_channel(self, request, channel_name):
        self.channel = Channel(
            channel_name=channel_name,
            data_path=request.app.config.get('path')
        )

    def dispatch_request(self, request, *args, **kwargs):
        self.set_channel(request, kwargs.get('channel'))
        return super().dispatch_request(request, *args, **kwargs)


class ChannelView(BaseChannelView):
    async def post(self, request, channel, id=None):
        # TODO:
        # - make the CRUD operations async/await
        result = self.channel.create(request.json)
        return response.json({
            'result': result
        })

    async def get(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        result = self.channel.retrieve(id)
        return response.json(result)

    async def patch(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        result = self.channel.update(id, request.json)
        return response.json(result)

    async def delete(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        result = self.channel.delete(id)
        return response.json(result)

    async def put(self, request, channel, id):
        # TODO:
        # - make the CRUD operations async/await
        result = self.channel.restore(id)
        return response.json(result)


class RecentChannelView(BaseChannelView):
    async def get(self, request, channel):
        result = self.channel.recent()
        return response.json(result)


@service.listener('before_server_start')
async def get_config(app, loop):
    if len(sys.argv) > 0:
        path = sys.argv[1]
        config = configparser.ConfigParser()
        config.read(path)
        app.config.update(config['Storage'])


bp.add_route(ChannelView.as_view(), '/<id:[0-9]*>')
bp.add_route(RecentChannelView.as_view(), '/recent')

service.blueprint(bp)


if __name__ == '__main__':
    service.run(host="127.0.0.1", port=6363)
