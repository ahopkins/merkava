import os
import msgpack
from datetime import datetime
import pytz


# import asyncio


class Result:

    def __init__(self, data):
        self.data = data

    @property
    def is_deleted(self):
        return self.data.get(b'is_deleted', False)


class Channel:
    # TODO:
    # - Set maximum_recent to configurable value
    maximum_recent = 25

    def __init__(self, channel_name, data_path):
        self.channel_name = channel_name.lower()
        self.data_path = data_path
        self.index_path = os.path.join(
            data_path, '{}.mrkv'.format(self.channel_name)
        )
        if not os.path.exists(self.index_path):
            self.index = 1
            self._store_index()
        else:
            self.index = self._load(self.index_path)

    # async def listen(self):
    #     with open(self.index_path, 'rb') as file:
    #         file.seek(0, 0)
    #         while True:
    #             line = file.readline()
    #             if not line:
    #                 await asyncio.sleep(0.1)
    #                 continue
    #             print(line)
    #             yield line

    def create(self, raw):
        file_path = self._build_path(self.index)
        index = self.index
        data = {
            'id': index,
            'created': datetime.now(tz=pytz.utc).isoformat(),
            'data': raw,
        }
        self._dump(file_path, data)
        self += 1
        return 201, data

    def retrieve(self, id, override=False):
        file_path = self._build_path(id)
        if not os.path.exists(file_path):
            return 404, None

        data = self._load(file_path)
        if data.get(b'is_deleted', False) and not override:
            return 404, None

        return 200, data

    def update(self, id, raw):
        file_path = self._build_path(id)
        _, data = self.retrieve(id)
        inner_data = data.get(b'data')
        if isinstance(inner_data, dict):
            data.get(b'data').update(raw)
        else:
            data[b'data'] = raw
        self._dump(file_path, data)
        return 200, data

    def delete(self, id):
        file_path = self._build_path(id)
        _, data = self.retrieve(id)
        data.update({'is_deleted': True})
        self._dump(file_path, data)
        return 204, {'ok': True}

    def restore(self, id):
        file_path = self._build_path(id)
        _, data = self.retrieve(id, override=True)
        data.update({'is_deleted': False, 'was_restored': True})
        self._dump(file_path, data)
        return 200, data

    def recent(self, num):
        results = []
        index = self.index - 1
        num = min(num, self.maximum_recent)
        while len(results) < num and index > 0:
            result = self.get_result(index)
            if result is not None and not result.is_deleted:
                results.append(result.data)
            index -= 1
        return results

    def get_result(self, id):
        _, raw = self.retrieve(id)
        if raw is None:
            return None

        return Result(raw)

    def __add__(self, n):
        self.index += n
        self._store_index()
        return self.index

    def _build_path(self, id):
        file_name = '{}.{}.mrkv'.format(self.channel_name, id)
        return os.path.join(self.data_path, file_name)

    def _load(self, path):
        with open(path, 'rb') as store:
            return msgpack.load(store, use_list=False)

    def _dump(self, path, data):
        with open(path, 'wb') as store:
            msgpack.dump(data, store)

    def _store_index(self):
        self._dump(self.index_path, self.index)
