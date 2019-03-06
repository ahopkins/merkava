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

Let's say you want to have a chat or a news feed. The data will ALWAYS be displayed in the same order. Why not persist your data objects in the order you want them in?

MerkavaDB stores whatever data you need it to. It is schemaless. If you can transfer it, it will store.

How do I use it?
----------------

By making TCP connection to the database server. All you need to do is specify a "channel" and some data. The protocol is simple:

::

    <channel> <command> <extras>\n
    
Note, the line break at the end of the message is critical. As is the single spacing.

Your response will be:

::

    OK <message>
    
    or
    
    ER <message>

What is a channel?
++++++++++++++++++

A channel is a division of data. All data is stored in a sequential order for a given channel. For example, it could be a single chat room or news feed.

Supported Operations
--------------------

- ``PUSH`` - add a new item to the channel
- ``RETRIEVE`` - get a single item by id from the channel
- ``RECENT`` - get ``n` items from the channel
- ``UPDATE`` - change a single item
- ``DELETE`` - remove an item from a channel _(not yet implemented)_
- ``RESTORE`` - return a deleted item to the channel _(not yet implemented)_
- ``PURGE`` - cleanup all deleted items _(not yet implemented)_
- ``FLUSH`` - empty a channel
- ``BACKUP`` - persist a channel to disk
- ``STATS`` - receive information and stats about a channel


Examples
--------

| ``foo PUSH this is a message``
| ``foo RECENT``
| ``foo RECENT 5`` 5 most recent messages
| ``foo RECENT 5 2`` 5 most recent messages, offset by 2
| ``foo RETRIEVE EaR1US7HVN6xuSG-2SgJtA``
| ``foo STATS``
| 

Roadmap
-------

There currently is a proof of concept implementation out there. It is being rebuilt from the ground up in Rust. More to come later.
