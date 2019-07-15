"""MPD client represented as a Web Thing."""

from mpd import CommandError, MPDClient
from random import choice
from select import select
from webthing import (Action, Event, SingleThing, Property, Thing, Value,
                      WebThingServer)
import socket
import sys
import tornado.ioloop
import uuid


class PlayAction(Action):
    """Action to start playback."""

    def __init__(self, thing, input_):
        """Initialize the action."""
        Action.__init__(self, uuid.uuid4().hex, thing, 'play', input_=input_)

    def perform_action(self):
        """Perform the action, i.e. start playback."""
        self.thing.play()


class PauseAction(Action):
    """Action to pause playback."""

    def __init__(self, thing, input_):
        """Initialize the action."""
        Action.__init__(self, uuid.uuid4().hex, thing, 'pause', input_=input_)

    def perform_action(self):
        """Perform the action, i.e. pause playback."""
        self.thing.pause()


class StopAction(Action):
    """Action to stop playback."""

    def __init__(self, thing, input_):
        """Initialize the action."""
        Action.__init__(self, uuid.uuid4().hex, thing, 'stop', input_=input_)

    def perform_action(self):
        """Perform the action, i.e. stop playback."""
        self.thing.stop()


class NextAction(Action):
    """Action to skip to the next song."""

    def __init__(self, thing, input_):
        """Initialize the action."""
        Action.__init__(self, uuid.uuid4().hex, thing, 'next', input_=input_)

    def perform_action(self):
        """Perform the action, i.e. skip song."""
        self.thing.next()


class PreviousAction(Action):
    """Action to skip to the previous song."""

    def __init__(self, thing, input_):
        """Initialize the action."""
        Action.__init__(
            self, uuid.uuid4().hex, thing, 'previous', input_=input_)

    def perform_action(self):
        """Perform the action, i.e. skip song."""
        self.thing.previous()


class QueueRandomAction(Action):
    """Action to queue random songs."""

    def __init__(self, thing, input_):
        """Initialize the action."""
        Action.__init__(
            self, uuid.uuid4().hex, thing, 'queueRandom', input_=input_)

    def perform_action(self):
        """Perform the action, i.e. queue songs."""
        songs = self.thing.list()
        if songs:
            for _ in range(0, int(self.input['count'])):
                self.thing.add(choice(songs))

            # Since we just updated the playlist, emit an event.
            playlist = self.thing.get_playlist()
            if playlist is not None:
                self.thing.add_event(
                    PlaylistUpdatedEvent(self.thing, playlist))


class PlaylistUpdatedEvent(Event):
    """Event to indicate that the current playlist was updated."""

    def __init__(self, thing, data):
        """Initialize the event."""
        Event.__init__(self, thing, 'playlistUpdated', data=data)


class MPDThing(Thing):
    """MPD client Web Thing."""

    def __init__(self):
        """Initialize the thing."""
        Thing.__init__(
            self,
            'urn:dev:ops:mpd',
            'MPD',
            [],
            'Music Player Daemon'
        )

        # Connect to MPD.
        self.client = MPDClient()
        try:
            self.client.connect(host='localhost', port='6600')
            self.idling = False
        except socket.error:
            print('Failed to connect to MPD')
            sys.exit(1)

        # Get the current status for initialization purposes.
        status = self.get_status()
        song = self.get_current_song()

        # Add a 'volume' property.
        self.add_property(
            Property(self,
                     'volume',
                     Value(self.get_volume(status), self.set_volume),
                     metadata={
                         '@type': 'LevelProperty',
                         'type': 'number',
                         'description': 'Playback volume',
                         'minimum': 0,
                         'maximum': 100,
                         'unit': 'percent',
                         'title': 'Volume',
                     }))

        # Add a 'repeat' property.
        self.add_property(
            Property(self,
                     'repeat',
                     Value(self.get_repeat(status), self.set_repeat),
                     metadata={
                         '@type': 'BooleanProperty',
                         'type': 'boolean',
                         'description': 'Repeat mode',
                         'title': 'Repeat',
                     }))

        # Add a 'random' property.
        self.add_property(
            Property(self,
                     'random',
                     Value(self.get_random(status), self.set_random),
                     metadata={
                         '@type': 'BooleanProperty',
                         'type': 'boolean',
                         'description': 'Random mode',
                         'title': 'Random',
                     }))

        # Add a 'state' property, which indicates playback state.
        self.add_property(
            Property(self,
                     'state',
                     Value(self.get_state(status)),
                     metadata={
                         'type': 'string',
                         'enum': [
                             'play',
                             'stop',
                             'pause',
                         ],
                         'description': 'Current playback state',
                         'title': 'State',
                         'readOnly': True,
                     }))

        # Add an 'artist' property.
        self.add_property(
            Property(self,
                     'artist',
                     Value(self.get_artist(song)),
                     metadata={
                         'type': 'string',
                         'description': 'Artist of current song',
                         'title': 'Artist',
                         'readOnly': True,
                     }))

        # Add an 'album' property.
        self.add_property(
            Property(self,
                     'album',
                     Value(self.get_album(song)),
                     metadata={
                         'type': 'string',
                         'description': 'Album current song belongs to',
                         'title': 'Album',
                         'readOnly': True,
                     }))

        # Add a 'title' property.
        self.add_property(
            Property(self,
                     'title',
                     Value(self.get_title(song)),
                     metadata={
                         'type': 'string',
                         'description': 'Title of current song',
                         'title': 'Title',
                         'readOnly': True,
                     }))

        # Add a 'play' action.
        self.add_available_action(
            'play',
            {
                'description': 'Start playback',
                'title': 'Play',
            },
            PlayAction)

        # Add a 'pause' action.
        self.add_available_action(
            'pause',
            {
                'description': 'Pause playback',
                'title': 'Pause',
            },
            PauseAction)

        # Add a 'stop' action.
        self.add_available_action(
            'stop',
            {
                'description': 'Stop playback',
                'title': 'Stop',
            },
            StopAction)

        # Add a 'next' option.
        self.add_available_action(
            'next',
            {
                'description': 'Skip to next song',
                'title': 'Next',
            },
            NextAction)

        # Add a 'previous' action.
        self.add_available_action(
            'previous',
            {
                'description': 'Skip to previous song',
                'title': 'Previous',
            },
            PreviousAction)

        # Add a 'queueRandom' action.
        self.add_available_action(
            'queueRandom',
            {
                'description': 'Queue a series of random songs',
                'title': 'Queue Random',
                'input': {
                    'type': 'object',
                    'required': [
                        'count',
                    ],
                    'properties': {
                        'count': {
                            'type': 'number',
                            'minimum': 1,
                        },
                    },
                },
            },
            QueueRandomAction)

        # Add a 'playlistUpdated' event.
        self.add_available_event(
            'playlistUpdated',
            {'description': 'The current playlist has been updated',
             'type': 'string'})

        # Start the idle loop.
        self.client.send_idle()
        self.idling = True
        tornado.ioloop.IOLoop.current().call_later(1, self.idle)

    def send_command(self, command, *args):
        """Try to send a command to MPD and return the result."""
        result = None
        try:
            if self.idling:
                self.client.noidle()

            result = getattr(self.client, command)(*args)

            if self.idling:
                self.client.send_idle()
        except CommandError as e:
            print(e)
        finally:
            return result

    def get_status(self):
        """Get the current status."""
        return self.send_command('status')

    def get_volume(self, status=None):
        """Get the current volume."""
        if status is None:
            status = self.get_status()
            if status is None:
                return None

        return int(status['volume'])

    def get_random(self, status=None):
        """Get the current 'random' state."""
        if status is None:
            status = self.get_status()
            if status is None:
                return None

        return bool(int(status['random']))

    def get_repeat(self, status=None):
        """Get the current 'repeat' state."""
        if status is None:
            status = self.get_status()
            if status is None:
                return None

        return bool(int(status['repeat']))

    def get_state(self, status=None):
        """Get the current playback state."""
        if status is None:
            status = self.get_status()
            if status is None:
                return None

        return status['state']

    def get_current_song(self):
        """Get the current song."""
        return self.send_command('currentsong')

    def get_artist(self, song=None):
        """Get the artist of the current song."""
        if song is None:
            song = self.get_current_song()
            if song is None:
                return None

        if 'artist' in song:
            return song['artist']

        return None

    def get_album(self, song=None):
        """Get the album of the current song."""
        if song is None:
            song = self.get_current_song()
            if song is None:
                return None

        if 'album' in song:
            return song['album']

        return None

    def get_title(self, song=None):
        """Get the title of the current song."""
        if song is None:
            song = self.get_current_song()
            if song is None:
                return None

        if 'title' in song:
            return song['title']

        return None

    def set_volume(self, level):
        """Set the volume."""
        self.send_command('setvol', level)

    def set_random(self, random):
        """Set the 'random' state."""
        self.send_command('random', int(random))

    def set_repeat(self, repeat):
        """Set the 'repeat' state."""
        self.send_command('repeat', int(repeat))

    def update_volume(self, volume=None):
        """Update the volume property."""
        if volume is None:
            volume = self.get_volume()
            if volume is None:
                return

        prop = self.find_property('volume')
        prop.value.notify_of_external_update(volume)

    def update_random(self, random=None):
        """Update the random property."""
        if random is None:
            random = self.get_random()
            if random is None:
                return

        prop = self.find_property('random')
        prop.value.notify_of_external_update(random)

    def update_repeat(self, repeat=None):
        """Update the repeat property."""
        if repeat is None:
            repeat = self.get_repeat()
            if repeat is None:
                return

        prop = self.find_property('repeat')
        prop.value.notify_of_external_update(repeat)

    def update_state(self, state=None):
        """Update the playback state property."""
        if state is None:
            state = self.get_state()
            if state is None:
                return

        prop = self.find_property('state')
        prop.value.notify_of_external_update(state)

    def update_album(self, album=None):
        """Update the album property."""
        if album is None:
            album = self.get_album()
            if album is None:
                return

        prop = self.find_property('album')
        prop.value.notify_of_external_update(album)

    def update_artist(self, artist=None):
        """Update the artist property."""
        if artist is None:
            artist = self.get_artist()
            if artist is None:
                return

        prop = self.find_property('artist')
        prop.value.notify_of_external_update(artist)

    def update_title(self, title=None):
        """Update the title property."""
        if title is None:
            title = self.get_title()
            if title is None:
                return

        prop = self.find_property('title')
        prop.value.notify_of_external_update(title)

    def play(self):
        """Start or resume playback."""
        state = self.get_state()
        if state == 'pause':
            self.send_command('pause', 0)
        elif state == 'stop':
            self.send_command('play', 0)

    def pause(self):
        """Pause playback."""
        state = self.get_state()
        if state == 'play':
            self.send_command('pause', 1)

    def stop(self):
        """Stop playback."""
        state = self.get_state()
        if state in ['play', 'pause']:
            self.send_command('stop')

    def next(self):
        """Skip to the next song."""
        state = self.get_state()
        if state in ['play', 'pause']:
            self.send_command('next')

    def previous(self):
        """Skip to the previous song."""
        state = self.get_state()
        if state in ['play', 'pause']:
            self.send_command('previous')

    def list(self):
        """Get a list of all songs MPD knows about."""
        return self.send_command('list', 'file')

    def add(self, uri):
        """Add a song to the current playlist."""
        self.send_command('add', uri)

    def get_playlist(self):
        """Get the current playlist."""
        playlist = self.send_command('playlistinfo')
        if playlist is None:
            return None

        songs = []
        for song in playlist:
            artist = song['artist'] if 'artist' in song else 'Unknown'
            title = song['title'] if 'title' in song else 'Unknown'
            songs.append('{} - {}'.format(artist, title))

        return '\n'.join(songs)

    def idle(self):
        """Watch for events."""
        if self.idling and select([self.client], [], [], 0)[0]:
            subsystems = []
            try:
                subsystems = self.client.fetch_idle()
                self.client.send_idle()
            except CommandError as e:
                print(e)
            else:
                for subsystem in subsystems:
                    if subsystem == 'playlist':
                        # The current playlist was updated, so emit an event.
                        playlist = self.get_playlist()
                        if playlist is not None:
                            self.add_event(
                                PlaylistUpdatedEvent(self, playlist))

                    elif subsystem == 'player':
                        # The player state was updated, so update the current
                        # song info and playback state.
                        song = self.get_current_song()
                        self.update_album(self.get_album(song))
                        self.update_artist(self.get_artist(song))
                        self.update_title(self.get_title(song))
                        self.update_state()

                    elif subsystem == 'mixer':
                        # The mixer was updated, so update the volume.
                        self.update_volume(self.get_volume())

                    elif subsystem == 'options':
                        # One of the options was updated, so check repeat and
                        # random.
                        status = self.get_status()
                        self.update_repeat(self.get_repeat(status))
                        self.update_random(self.get_random(status))

        tornado.ioloop.IOLoop.current().call_later(1, self.idle)


def run_server():
    """Create our MPD Web Thing and run the server."""
    thing = MPDThing()

    server = WebThingServer(SingleThing(thing), port=8888)

    try:
        server.start()
    except KeyboardInterrupt:
        server.stop()


if __name__ == '__main__':
    run_server()
