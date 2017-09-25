import configparser
import sys
import os

from merkava import blueprints
from sanic import Sanic


# from aoiklivereload import LiveReloader
# try:
#     reloader = LiveReloader()
#     reloader.start_watcher_thread()
# except Exception:
#     pass


service = Sanic()
service.blueprint(blueprints.bp)


@service.listener('before_server_start')
async def get_config(app, loop):
    if len(sys.argv) > 1:
        path = sys.argv[1]
        if os.path.isfile(path):
            config = configparser.ConfigParser()
            config.read(path)
            app.config.update(config['Storage'])


if __name__ == '__main__':
    service.run(host="127.0.0.1", port=6363)
