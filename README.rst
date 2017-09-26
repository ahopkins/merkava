=============================
MerkavaDB
=============================

.. image:: https://img.shields.io/pypi/v/merkava.svg
    :target: https://pypi.python.org/pypi/merkava
    :alt: Latest PyPI version

.. image:: https://img.shields.io/pypi/status/merkava.svg
    :target: https://pypi.python.org/pypi/merkava
    :alt: Version status

.. image:: https://img.shields.io/pypi/pyversions/merkava.svg
    :target: https://pypi.python.org/pypi/merkava
    :alt: Python 3.5 and 3.6

.. image:: https://travis-ci.org/ahopkins/merkava.png?branch=master
    :target: https://travis-ci.org/ahopkins/merkava

.. image:: https://readthedocs.org/projects/merkava/badge/?version=latest
    :target: http://merkava.readthedocs.io/en/latest/?badge=latest
    :alt: Documentation Status

A fast ordered NoSQL database.

`Documentation <http://merkava.readthedocs.io/en/latest/>`_ | `GitHub <https://github.com/ahopkins/merkava>`_

.. note::
    This is still in **active** development. Things will change. If you are interested in helping out, or would like to see any particular features added, let me know.

.. image:: https://raw.githubusercontent.com/ahopkins/merkava/master/assets/logo.png

What is MerkavaDB?
------------------

A very fast, and lightweight DB for storing ordered data. The order it comes out is the order that it went in. The API is meant to be small and without bloated queries. Consequently, there is a limited set of queries to be made since the primary tool is getting objects in and out of storage in a specific order.

So, why would I use it?
-----------------------

Because it is fast. And it is simple.

Let's say, for example, you are building an application. As a part of your application, you want to have a chat or a news feed. The data will ALWAYS be displayed in the same order. Well, you can persist your data objects and feel condifent that they will always be in the same order, no matter what.

MerkavaDB stores data in a similar format to JSON. So, it is schemaless and will allow you to store data in whatever format you need.

How do I use it?
----------------

By making HTTP calls to the database server. All you need to do is specify a "channel" and some data.

What kind of data?
++++++++++++++++++

- nulls
- booleans
- integers
- floats
- strings
- arrays/lists
- maps/dicts

Basically anything you would pass by JSON.

What is a channel?
++++++++++++++++++

A channel is a division of data. All data is stored in a sequential order given the channel that it is in. For example, it could be a single chat room or news feed.

Supported Operations
--------------------

- ``HTTP POST /<channel>/`` - create
- ``HTTP GET /<channel>/<id>/`` - retrieve a single record
- ``HTTP PATCH /<channel>/<id>/`` - update a record
- ``HTTP DELETE /<channel>/<id>/`` - delete a record
- ``HTTP PUT /<channel>/<id>/`` - restore a deleted record
- ``HTTP GET /<channel>/recent/<X>`` - retrieve an array of the X most recent records

Roadmap
-------

- Drivers for: Python, NodeJS, Java
- Test coverage
- Documentation
- Clean up utilities
- User interface
- Debian installer
- Single script installer
- Configuration options
- Examples
- Logging

Current Version
---------------
version 0.2.0
