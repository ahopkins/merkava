=============================
MerkavaDB
=============================

.. note::
    This is still in **active** development. Things will change. If you are interested in helping out, or would like to see any particular features added, let me know.

.. image:: https://raw.githubusercontent.com/ahopkins/merkava/master/assets/logo.png

What is MerkavaDB?
------------------

A fast, and lightweight DB for storing ordered data. The order it comes out is the order that it went in. The API is meant to be small and without bloated queries. Consequently, there is a limited set of queries to be made since the primary tool is getting objects in and out of storage in a specific order.

So, why would I use it?
-----------------------

Because it is fast. And it is simple.

Let's say you want to have a chat or a news feed. The data will ALWAYS be displayed in the same order. Why not persist your data objects and feel condifent that they will always be in the same order, no matter what.

MerkavaDB stores data in a similar format to JSON. So, it is schemaless and will allow you to store data in whatever format you need.

How do I use it?
----------------

By making TCP connection to the database server. All you need to do is specify a "channel" and some data.

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

A channel is a division of data. All data is stored in a sequential order for a given channel. For example, it could be a single chat room or news feed.

Supported Operations
--------------------

- ``PUSH`` - add a new item to the channel
- ``RETRIEVE`` - get a single item by id from the channel
- ``RECENT`` - get ``n` items from the channel
- ``UPDATE`` - change a single item
- ``DELETE`` - remove an item from a channel
- ``RESTORE`` - return a deleted item to the channel
- ``PURGE`` - cleanup all deleted items
- ``FLUSH`` - empty a channel
- ``STATS`` - receive information and stats about a channel


Roadmap
-------

There currently is a proof of concept implementation out there. It is being rebuilt from the ground up in Rust. More to come later.
