from sanic import Blueprint
from merkava import views

bp = Blueprint('channels', url_prefix='/v1/<channel>')
bp.add_route(views.ChannelView.as_view(), '/<id:[0-9]*>')
bp.add_route(views.RecentChannelView.as_view(), '/recent/<num:[0-9]*>')
