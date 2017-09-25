"""
Tests for `merkava` module.
"""

import merkava
import os


# raise Exception(merkava)

service = merkava.server.service
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'data')
channel_name = 'text_channel'


class TestMerkava(object):

    @classmethod
    def setup_class(cls):
        service.config.update({
            'path': path
        })

        folder = service.config.get('path')
        if os.path.exists(folder):
            for file in os.listdir(folder):
                file_path = os.path.join(folder, file)
                if os.path.isfile(file_path):
                    os.unlink(file_path)

    def url(self, *args):
        url = os.path.join('/', 'v1', 'text_channel', *args)
        print(url)
        return url

    def test_channel_create_index(self):
        request, response = service.test_client.get(self.url('9999'))
        file_path = os.path.join(path, '{}.mrkv'.format(channel_name))
        assert os.path.exists(file_path)
        assert os.path.isfile(file_path)

    def test_retrieve_not_exists(self):
        request, response = service.test_client.get(self.url('9999'))
        assert response.json is None
        assert response.status == 404

    @classmethod
    def teardown_class(cls):
        pass
