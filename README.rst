=============================
MerkavaDB
=============================

.. image:: https://badge.fury.io/py/merkava.png
    :target: http://badge.fury.io/py/merkava

.. image:: https://travis-ci.org/ahopkins/merkava.png?branch=master
    :target: https://travis-ci.org/ahopkins/merkava

A fast ordered NoSQL database.

.. note::
    This is still in **active** development. Things will change. If you are interested in helping out, or would like to see any particular features added, let me know.

What is MerkavaDB?
------------------

It is built to be a very fast, and lightweight DB for storing ordered data. The order it comes out is the order that it went in. The API is meant to be small and without bloated queries. There is no data reordering, or filtering.

So, why would I use it?
-----------------------

Because it is fast. And it is simple.

Let's say, for example, you are building an application. As a part of your application, you want to have a chat or a news feed. The data will ALWAYS be displayed in the same order. Well, why not store and retrieve it that way instead of doing ``ORDER BY ....``?

MerkavaDB stores data in a similar format to JSON. So, it is schemaless and will allow you to store data in whatever format you need.

Supported Operations
--------------------

- ``HTTP POST /<channel>/`` - create
- ``HTTP GET /<channel>/<id>/`` - retrieve a single record
- ``HTTP PATCH /<channel>/<id>/`` - update a record
- ``HTTP DELETE /<channel>/<id>/`` - delete a record
- ``HTTP PUT /<channel>/<id>/`` - restore a record
- ``HTTP GET /<channel>/recent/<X>`` - retrieve an array of the X most recent records
