from new import Receiver


DEBUG = True

if DEBUG:
    from aoiklivereload import LiveReloader
    reloader = LiveReloader()
    reloader.start_watcher_thread()


if __name__ == '__main__':
    Receiver.run()
