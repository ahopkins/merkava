from pathlib import Path
import os
import msgpack
import pytz
from datetime import datetime
import trio


class Channel:
    maximum_recent = 20
    index_size = 17
    _cache = {}

    def __init__(self, channel_name, data_path):
        self.channel_name = channel_name.lower()
        self.data_path = data_path
        self.index_path = trio.Path(data_path) / f'{self.channel_name}.mrkv'

    @classmethod
    async def open(cls, channel_name, data_path):
        instance = cls(channel_name, data_path)
        if not await instance.data_path.exists():
            await instance.data_path.mkdir()
        if not await instance.index_path.exists():
            await instance._create_index()

        print(f'\n\nOpening {channel_name}\n\n')

        return instance

    async def connect(self, t, payload):
        """
        Connect a client to a channel
        """
        return True, None

    async def push(self, t, payload):
        """
        Push a new message into the channel
        """
        await self._store_index(t)
        file_path = self._build_path(t)
        data = {
            'index': t,
            'created': datetime.now(tz=pytz.utc).isoformat(),
            'data': payload,
        }
        await self._dump(file_path, data)
        self.set_cache(payload, data)
        return True, f'<Message {t}>'

    async def retrieve(self, t, payload, force=False):
        """
        Load a record and return it only if not marked deleted
        """
        if not self.in_cache(payload):
            path = self._build_path(payload)
            if not os.path.exists(path):
                return True, None

            data = await self._load(path)
            if data.get('is_deleted', False) and force is False:
                return True, None

            self.set_cache(payload, data)

            return True, data
        else:
            return True, self.get_cache(payload)

    async def update(self, t, payload):
        """
        Make changes to record
        """
        index = payload[:17]
        payload = payload[18:]
        path = self._build_path(index)
        _, result = await self.retrieve(t, index)
        if result is not None:
            updates = {
                'data': payload,
                'updated': datetime.now(tz=pytz.utc).isoformat(),
            }
            result.update(updates)
            await self._dump(path, result)
            self.set_cache(payload, result)
        return True, None

    async def delete(self, t, payload):
        """
        Mark a record as deleted
        """
        await self._change_delete(t, payload, True)
        return True, None

    async def restore(self, t, payload):
        """
        Restore a deleted record
        """
        record = await self._change_delete(t, payload, False)
        return True, record

    async def recent(self, t, payload):
        """
        Return recent number of records
        """
        if not payload:
            payload = 1

        # Make sure that the payload is number-like and can be handled
        try:
            num = int(payload)
        except ValueError:
            return False, None

        results = []
        checked = []
        num = min(num, self.maximum_recent)
        position = -1 * (num * self.index_size)
        file_name = self.index_path
        async with await trio.open_file(file_name, 'rb') as inf:
            await inf.seek(0, 2)
            tell = await inf.tell()

            start = int(-1 * tell)
            position = max(position, start)
            starting_position = position
            await inf.seek(position, 2)

            print('')
            print('')
            eof = False
            while len(results) < num and not eof:
                position = int(-1 * await inf.tell())
                content = await inf.read(self.index_size)
                if position not in checked:
                    print(f'   - seeking {position}, found {content}')
                    index = content.decode('utf-8')
                    _, result = await self.retrieve(t, index)
                    if result:
                        results.append(result)

                    if len(results) == num:
                        break

                    checked.append(position)
                else:
                    print(f'   - {position} already checked')

                if abs(position) == tell:
                    missing = num - len(results)
                    print(f'   - at end of file, {missing} missing')
                    starting_position -= missing * self.index_size
                    position = max(starting_position, start)
                    await inf.seek(position, 2)

                    if 0 in checked:
                        eof = True
            print('')
            print('')
        print('results', results)
        results.sort(key=lambda x: x.get('index'), reverse=True)
        return True, results

    async def purge(self, t, payload):
        """
        Remove all deleted records from memory and disk

        FOR FUTURE EDITIONS.
        - Perhaps rather than marking a flag on the record, the file_name
          will be changed. Then when running, retrieve() it will be unable to
          find the records. And, for purging, they can be easily identified in
          the file system with a glob.
        """
        return True, None

    async def flush(self, t, payload):
        """
        Remove all records from memory and disk
        """
        for mrkv in await self.data_path.iterdir():
            mrkv.unlink()
        await self.data_path.rmdir()
        return True, True

    def _build_path(self, index):
        """
        Build a path given an index
        """
        file_name = '{}.{}.mrkv'.format(self.channel_name, index)
        return Path(self.data_path) / file_name

    def _stats(self):
        """
        Provide stats on size of index, number of records, both on disk and in
        memory
        """
        pass

    async def _warmup(self, size=25):
        """
        Load records into memory of length size
        """
        _, results = await self.recent(None, size)
        for result in results:
            self.set_cache(result.get('index'), result)

    async def _change_delete(self, t, payload, is_deleted):
        print(f'_change_delete to {is_deleted}')
        path = self._build_path(payload)
        _, result = await self.retrieve(t, payload, force=True)
        print(result)
        if result is not None:
            result.update({'is_deleted': is_deleted})
            await self._dump(path, result)

        if is_deleted:
            return None
        else:
            return result

    async def _load(self, path):
        """
        Look for path in memory, if exists, else look for file.
        If the data is found in memory, check for path.
        """
        print(f'loading from disk: {path}')
        async with await trio.open_file(path, 'rb') as store:
            content = await store.read()
            return msgpack.loads(content, use_list=False, raw=False)

    async def _dump(self, path, data):
        """
        Store data in memory, setup uvloop and persist to disk.
        """
        async with await trio.open_file(path, 'wb') as store:
            content = msgpack.dumps(data)
            await store.write(content)

    async def _store_index(self, index):
        """
        Persist the index to disk
        """
        async with await trio.open_file(self.index_path, 'a') as store:
            if index:
                await store.write(f'{index}')

    async def _create_index(self):
        """
        Persist the index to disk
        """
        if not await self.index_path.exists():
            await self.index_path.touch()

    def set_cache(self, key, value):
        key = f'{self.channel_name}:{key}'
        self._cache[key] = value

    def get_cache(self, key):
        key = f'{self.channel_name}:{key}'
        return self._cache.get(key, None)

    def in_cache(self, key):
        key = f'{self.channel_name}:{key}'
        return key in self._cache
