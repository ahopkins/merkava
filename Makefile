.PHONY: help bump clean clean-pyc clean-build dev list history history-update test test-all coverage docs release sdist

help:
	@echo "bump - bump the version number, commit, and tag, ex make bump part=minor"
	@echo "clean-build - remove build artifacts"
	@echo "clean-pyc - remove Python file artifacts"
	@echo "dev - load the development server"
	@echo "lint - check style with flake8"
	@echo "history - add item to the HISTORY.rst, ex make history h='foo bar'"
	@echo "history-update - update HISTORY.rst, ex make history-update v=1.0.0"
	@echo "test - run tests quickly with the default Python"
	@echo "test-all - run tests on every Python version with tox"
	@echo "coverage - check code coverage quickly with the default Python"
	@echo "docs - generate Sphinx HTML documentation, including API docs"
	@echo "release - package and upload a release"
	@echo "sdist - package"

bump:
	bumpversion $(part) --verbose

clean: clean-build clean-pyc

clean-build:
	rm -fr build/
	rm -fr dist/
	rm -fr *.egg-info

clean-pyc:
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +

dev:
	./merkava/merkava.sh ../example/config.ini

lint:
	flake8 merkava test

test:
	py.test

test-all:
	tox

coverage:
	coverage run --source merkava setup.py test
	coverage report -m
	coverage html
	open htmlcov/index.html

docs:
	rm -f docs/merkava.rst
	rm -f docs/modules.rst
	sphinx-apidoc -o docs/ merkava
	$(MAKE) -C docs clean
	$(MAKE) -C docs html
	open docs/_build/html/index.html

release: clean
	python setup.py sdist upload
	python setup.py bdist_wheel upload

sdist: clean
	python setup.py sdist
	python setup.py bdist_wheel upload
	ls -l dist
