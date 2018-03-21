import configparser
import sys
import os

from merkava import blueprints
from sanic import Sanic
from sanic.response import json

if os.environ.get('DEVELOPMENT') == 'true':  # pragma: no cover
    from aoiklivereload import LiveReloader

    reloader = LiveReloader()
    reloader.start_watcher_thread()
service = Sanic()
service.blueprint(blueprints.bp)


@service.route("/")
async def is_alive(request):
    return json({'is_alive': True})


@service.listener('before_server_start')
async def get_config(app, loop):
    if len(sys.argv) > 1:  # pragma: no cover
        path = sys.argv[1]
        if os.path.isfile(path):
            config = configparser.ConfigParser()
            config.read(path)
            app.config.update(config['Storage'])


if __name__ == '__main__':  # pragma: no cover
    service.run(host="127.0.0.1", port=6363)
