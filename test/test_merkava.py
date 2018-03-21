import merkava
import os
import json

service = merkava.server.service
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'data')
channel_name = 'text_channel'


class TestMerkava(object):

    @classmethod
    def setup_class(cls):
        service.config.update({'path': path})

    def url(self, *args):
        url = os.path.join(
            '/', 'v1', 'text_channel', *(str(arg) for arg in args)
        )
        if not url.endswith('/'):
            url += '/'
        return url

    ######################################
    #   Basic CRUD and operation tests   #
    ######################################

    def test_channel_create_index(self):
        _, response = service.test_client.get(self.url('9999'))
        file_path = os.path.join(path, '{}.mrkv'.format(channel_name))
        assert os.path.exists(file_path)
        assert os.path.isfile(file_path)

    def test_retrieve_not_exists(self):
        _, response = service.test_client.get(self.url('9999'))
        assert response.json is None
        assert response.status == 404

    def test_create(self):
        data = {'foo': 'bar', 'action': 'create'}
        _, response = service.test_client.post(
            self.url(), data=json.dumps(data)
        )
        assert response.json is not None
        assert response.status == 201
        assert response.json.get('result').get('data') == data
        assert response.json.get('result').get('data').get(
            'action'
        ) == 'create'

    def test_retrieve(self):
        data = {'foo': 'bar', 'action': 'retrieve'}
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(2))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert response.json.get('data').get('action') == 'retrieve'
        assert response.json.get('id') == 2

    def test_update(self):
        data = {'one': 1, 'two': 2}
        update = {'two': 200, 'three': 3}
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.patch(
            self.url(3), data=json.dumps(update)
        )
        assert response.status == 200
        _, response = service.test_client.get(self.url(3))
        assert response.json is not None
        assert response.json.get('data') != data
        assert response.json.get('id') == 3
        assert response.json.get('data').get('one') == 1
        assert 'two' in response.json.get('data')
        assert 'three' in response.json.get('data')
        assert response.json.get('data').get('two') == 200
        assert response.json.get('data').get('three') == 3

    def test_delete(self):
        data = {'foo': 'bar', 'action': 'delete'}
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.delete(self.url(4))
        assert response.status == 204
        assert response.json is None
        _, response = service.test_client.get(self.url(4))
        assert response.json is None
        assert response.status == 404

    def test_restore(self):
        data = {'foo': 'bar', 'action': 'restore'}
        service.test_client.post(self.url(), data=json.dumps(data))
        service.test_client.delete(self.url(5))
        _, response = service.test_client.put(self.url(5))
        assert response.status == 200
        assert response.json is not None
        assert response.json.get('data') == data
        assert response.json.get('id') == 5
        assert response.json.get('data').get('action') == 'restore'
        assert response.json.get('is_deleted') is False
        assert response.json.get('was_restored') is True

    def test_recent(self):
        _, response = service.test_client.get(self.url('recent'))
        assert response.json is not None
        assert response.status == 200
        assert isinstance(response.json, list)
        assert len(response.json) > 0
        _, response = service.test_client.get(self.url('recent', 2))
        assert response.json is not None
        assert response.status == 200
        assert isinstance(response.json, list)
        assert len(response.json) == 2

    ###########################################
    #   Specific feature and use case tests   #
    ###########################################

    def test_storage_of_string(self):
        data = 'foo'
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(6))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert isinstance(response.json.get('data'), str)

    def test_storage_of_boolean(self):
        data = False
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(7))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert isinstance(response.json.get('data'), bool)
        data = True
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(8))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert isinstance(response.json.get('data'), bool)

    def test_storage_of_none(self):
        data = None
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(9))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert response.json.get('data') is None

    def test_storage_of_int(self):
        data = 1
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(10))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert isinstance(response.json.get('data'), int)

    def test_storage_of_float(self):
        data = 1.0
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(11))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert isinstance(response.json.get('data'), float)

    def test_storage_of_list(self):
        data = [65535, (65536 * 65536), 0x7fffffff, 0x80000000, 1e10]
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.get(self.url(12))
        assert response.json is not None
        assert response.status == 200
        assert response.json.get('data') == data
        assert all(
            isinstance(x, (float, int)) for x in response.json.get('data')
        )

    def test_update_of_non_dict(self):
        data = 'foo'
        update = 'bar'
        service.test_client.post(self.url(), data=json.dumps(data))
        _, response = service.test_client.patch(
            self.url(13), data=json.dumps(update)
        )
        assert response.status == 200
        _, response = service.test_client.get(self.url(13))
        assert response.json is not None
        assert response.json.get('data') != data
        assert response.json.get('id') == 13
        assert response.json.get('data') == update

    @classmethod
    def teardown_class(cls):
        folder = service.config.get('path')
        if os.path.exists(folder):
            for file in os.listdir(folder):
                file_path = os.path.join(folder, file)
                if os.path.isfile(file_path) and '.gitkeep' not in file_path:
                    os.unlink(file_path)
