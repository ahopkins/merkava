import os
import msgpack
from datetime import datetime
import pytz


class Result(object):
    def __init__(self, data):
        self.data = data

    def __iter__(self):
        return {key: value for key, value in self.data}

    @property
    def is_deleted(self):
        return self.data.get(b'is_deleted', False)


class Channel(object):
    def __init__(self, channel_name, data_path):
        self.channel_name = channel_name.lower()
        self.data_path = data_path
        self.index_path = os.path.join(data_path, '{}.mrkv'.format(self.channel_name))
        if not os.path.exists(self.index_path):
            self.index = 0
            self._store_index()
        else:
            with open(self.index_path, 'rb') as store:
                self.index = msgpack.load(store, use_list=False)

    def create(self, raw):
        file_name = '{}.{}.mrkv'.format(self.channel_name, self.index)
        file_path = os.path.join(self.data_path, file_name)
        index = self.index
        data = {
            'id': index,
            'created': datetime.now(tz=pytz.utc).isoformat(),
            'data': raw
        }
        with open(file_path, 'wb') as store:
            msgpack.dump(data, store)
        self += 1
        return index, data

    def retrieve(self, id):
        file_name = '{}.{}.mrkv'.format(self.channel_name, id)
        file_path = os.path.join(self.data_path, file_name)
        if not os.path.exists(file_path):
            return None
        with open(file_path, 'rb') as store:
            data = msgpack.load(store)
        return data

    def update(self, id, raw):
        file_name = '{}.{}.mrkv'.format(self.channel_name, id)
        file_path = os.path.join(self.data_path, file_name)
        data = self.retrieve(id)
        inner_data = data.get(b'data')
        if isinstance(inner_data, dict):
            data.get(b'data').update(raw)
        else:
            data[b'data'] = raw
        with open(file_path, 'wb') as store:
            msgpack.dump(data, store)
        return data

    def delete(self, id):
        file_name = '{}.{}.mrkv'.format(self.channel_name, id)
        file_path = os.path.join(self.data_path, file_name)
        data = self.retrieve(id)
        data.update({'is_deleted': True})
        with open(file_path, 'wb') as store:
            msgpack.dump(data, store)
        return {'ok': True}

    def restore(self, id):
        file_name = '{}.{}.mrkv'.format(self.channel_name, id)
        file_path = os.path.join(self.data_path, file_name)
        data = self.retrieve(id)
        data.update({
            'is_deleted': False,
            'was_restored': True,
        })
        with open(file_path, 'wb') as store:
            msgpack.dump(data, store)
        return data

    def recent(self):
        results = []
        index = self.index - 1
        while len(results) < 5 and index > 0:
            result = self.get_result(index)
            if result is not None and not result.is_deleted:
                results.append(result.data)
            index -= 1
        return results

    def get_result(self, id):
        raw = self.retrieve(id)
        if raw is None:
            return None
        return Result(raw)

    def __add__(self, n):
        self.index += n
        self._store_index()
        return self.index

    def _store_index(self):
        with open(self.index_path, 'wb') as store:
            msgpack.dump(self.index, store)
