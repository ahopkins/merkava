#!/usr/bin/env python
import os
import sys

try:
    from setuptools import setup
except ImportError:
    from distutils.core import setup
if sys.argv[-1] == 'publish':
    os.system('python setup.py sdist upload')
    sys.exit()
readme = open('README.rst').read()
doclink = """
Documentation
-------------

The full documentation is at http://merkava.rtfd.org."""
history = open('HISTORY.rst').read().replace('.. :changelog:', '')
setup(
    name='merkava',
    version='0.3.0',
    description='A fast ordered NoSQL datastore.',
    long_description=readme + '\n\n' + doclink + '\n\n' + history,
    author='Adam Hopkins',
    author_email='admhpkns@gmail.com',
    url='https://github.com/ahopkins/merkava',
    packages=['merkava'],
    package_dir={'merkava': 'merkava'},
    include_package_data=True,
    install_requires=[],
    license='MIT',
    zip_safe=False,
    keywords='merkava',
    setup_requires=['pytest-runner'],
    tests_require=['pytest'],
    classifiers=['Development Status :: 4 - Beta', 'Intended Audience :: Developers', 'License :: OSI Approved :: MIT License', 'Natural Language :: English', 'Programming Language :: Python :: 3.6'],
    # 'Programming Language :: Python :: Implementation :: PyPy',
)
