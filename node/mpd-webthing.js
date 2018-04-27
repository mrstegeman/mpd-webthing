/**
 * MPD client represented as a Web Thing.
 */

const mpd = require('mpd');
const {
  Action,
  Event,
  Property,
  Thing,
  Value,
  WebThingServer,
} = require('webthing');
const uuidv4 = require('uuid/v4');


/**
 * Action to start playback.
 */
class PlayAction extends Action {
  /**
   * Initialize the action.
   */
  constructor(thing, input) {
    super(uuidv4(), thing, 'play', input);
  }

  /**
   * Perform the action, i.e. start playback.
   */
  performAction() {
    return this.thing.play();
  }
}

/**
 * Action to pause playback.
 */
class PauseAction extends Action {
  /**
   * Initialize the action.
   */
  constructor(thing, input) {
    super(uuidv4(), thing, 'pause', input);
  }

  /**
   * Perform the action, i.e. pause playback.
   */
  performAction() {
    return this.thing.pause();
  }
}

/**
 * Action to stop playback.
 */
class StopAction extends Action {
  /**
   * Initialize the action.
   */
  constructor(thing, input) {
    super(uuidv4(), thing, 'stop', input);
  }

  /**
   * Perform the action, i.e. stop playback.
   */
  performAction() {
    return this.thing.stop();
  }
}

/**
 * Action to skip to the next song.
 */
class NextAction extends Action {
  /**
   * Initialize the action.
   */
  constructor(thing, input) {
    super(uuidv4(), thing, 'next', input);
  }

  /**
   * Perform the action, i.e. skip song.
   */
  performAction() {
    return this.thing.next();
  }
}

/**
 * Action to skip to the previous song.
 */
class PreviousAction extends Action {
  /**
   * Initialize the action.
   */
  constructor(thing, input) {
    super(uuidv4(), thing, 'previous', input);
  }

  /**
   * Perform the action, i.e. skip song.
   */
  performAction() {
    return this.thing.previous();
  }
}

/**
 * Action to queue random songs.
 */
class QueueRandomAction extends Action {
  /**
   * Initialize the action.
   */
  constructor(thing, input) {
    super(uuidv4(), thing, 'queueRandom', input);
  }

  /**
   * Perform the action, i.e. queue songs.
   */
  performAction() {
    return this.thing.list().then((songs) => {
      let promises = [];

      if (songs) {
        for (let i = 0; i < this.input.count; ++i) {
          const uri = songs[Math.floor(Math.random() * songs.length)].file;
          promises.push(this.thing.add(uri));
        }

        // Since we just updated the playlist, emit an event.
        promises.push(this.thing.getPlaylist().then((playlist) => {
          if (playlist) {
            this.thing.addEvent(new PlaylistUpdatedEvent(this.thing, playlist));
          }
        }));
      }

      return Promise.all(promises);
    });
  }
}

/**
 * Event to indicate that the current playlist was updated.
 */
class PlaylistUpdatedEvent extends Event {
  /**
   * Initialize the event.
   */
  constructor(thing, data) {
    super(thing, 'playlistUpdated', data);
  }
}

/**
 * MPD client Web Thing.
 */
class MPDThing extends Thing {
  /**
   * Initialize the thing.
   */
  constructor() {
    super('MPD', 'musicPlayer', 'Music Player Daemon');

    // Connect to MPD.
    this.client = mpd.connect({host: 'localhost', port: 6600});

    this.ready = new Promise((resolve) => {
      this.client.on('ready', () => {
        resolve();
      });
    });

    this.client.on('end', () => {
      console.log('Connection was closed');
      process.exit(1);
    });

    this.client.on('system-playlist', () => {
      // The current playlist was updated, so emit an event.
      this.getPlaylist().then((playlist) => {
        if (playlist !== null) {
          this.addEvent(new PlaylistUpdatedEvent(this, playlist));
        }
      });
    });

    this.client.on('system-player', () => {
      // The player state was updated, so update the current
      // song info and playback state.
      this.getCurrentSong().then((song) => {
        if (song !== null) {
          this.getAlbum(song).then((a) => this.updateAlbum(a));
          this.getArtist(song).then((a) => this.updateArtist(a));
          this.getTitle(song).then((t) => this.updateTitle(t));
        }
      });
      this.updateState();
    });

    this.client.on('system-mixer', () => {
      // The mixer was updated, so update the volume.
      this.getVolume().then((v) => this.setVolume(v));
    });

    this.ready.then(() => {
      // Get the current status for initialization purposes.
      this.getStatus().then((status) => {
        if (status === null) {
          console.log('Failed to get system status');
          process.exit(1);
        }

        // Add a 'volume' property.
        this.getVolume(status).then((v) => {
          this.addProperty(
            new Property(this,
                         'volume',
                         new Value(v, this.setVolume.bind(this)),
                         {
                           type: 'number',
                           description: 'Playback volume',
                           minimum: 0,
                           maximum: 100,
                         }));
        });

        // Add a 'repeat' property.
        this.getRepeat(status).then((r) => {
          this.addProperty(
            new Property(this,
                         'repeat',
                         new Value(r, this.setRepeat.bind(this)),
                         {
                           type: 'boolean',
                           description: 'Repeat mode',
                         }));
        });

        // Add a 'random' property.
        this.getRandom(status).then((r) => {
          this.addProperty(
            new Property(this,
                         'random',
                         new Value(r, this.setRandom.bind(this)),
                         {
                           type: 'boolean',
                           description: 'Random mode',
                         }));
        });

        // Add a 'state' property, which indicates playback state.
        this.getState(status).then((s) => {
          this.addProperty(
            new Property(this,
                         'state',
                         new Value(s),
                         {
                           type: 'string',
                           description: 'Current playback state',
                         }));
        });
      });

      this.getCurrentSong().then((song) => {
        if (song === null) {
          console.log('Failed to get current song');
          process.exit(1);
        }

        // Add an 'artist' property.
        this.getArtist(song).then((a) => {
          this.addProperty(
            new Property(this,
                         'artist',
                         new Value(a),
                         {
                           type: 'string',
                           description: 'Artist of current song',
                         }));
        });

        // Add an 'album' property.
        this.getAlbum(song).then((a) => {
          this.addProperty(
            new Property(this,
                         'album',
                         new Value(a),
                         {
                           type: 'string',
                           description: 'Album current song belongs to',
                         }));
        });

        // Add a 'title' property.
        this.getTitle(song).then((t) => {
          this.addProperty(
            new Property(this,
                         'title',
                         new Value(t),
                         {
                           type: 'string',
                           description: 'Title of current song',
                         }));
        });
      });

      // Add a 'play' action.
      this.addAvailableAction('play',
                              {description: 'Start playback'},
                              PlayAction);

      // Add a 'pause' action.
      this.addAvailableAction('pause',
                              {description: 'Pause playback'},
                              PauseAction);

      // Add a 'stop' action.
      this.addAvailableAction('stop',
                              {description: 'Stop playback'},
                              StopAction);

      // Add a 'next' option.
      this.addAvailableAction('next',
                              {description: 'Skip to next song'},
                              NextAction);

      // Add a 'previous' action.
      this.addAvailableAction('previous',
                              {description: 'Skip to previous song'},
                              PreviousAction);

      // Add a 'queueRandom' action.
      this.addAvailableAction(
        'queueRandom',
        {
          description: 'Queue a series of random songs',
          input: {
            type: 'object',
            required: [
              'count',
            ],
            properties: {
              count: {
                type: 'number',
                minimum: 1,
              },
            },
          },
        },
        QueueRandomAction);

      // Add a 'playlistUpdated' event.
      this.addAvailableEvent(
        'playlistUpdated',
        {
          description: 'The current playlist has been updated',
          type: 'string',
        });
    });
  }

  /**
   * Try to send a command to MPD and return the result.
   */
  sendCommand(command, ...args) {
    return new Promise((resolve) => {
      this.client.sendCommand(mpd.cmd(command, args), (err, msg) => {
        if (err) {
          console.log(err);
          resolve(null);
        } else {
          if (['list', 'playlistinfo'].includes(command)) {
            resolve(mpd.parseArrayMessage(msg));
          } else {
            resolve(mpd.parseKeyValueMessage(msg));
          }
        }
      });
    });
  }

  /**
   * Get the current status.
   */
  getStatus() {
    return this.sendCommand('status');
  }

  /**
   * Get the current 'random' state.
   */
  getRandom(status) {
    if (typeof status === 'undefined') {
      return this.getStatus().then((s) => this.getRandom(s));
    } else if (status === null) {
      return Promise.resolve(null);
    } else {
      return Promise.resolve(status.random === '1');
    }
  }

  /**
   * Get the current 'repeat' state.
   */
  getRepeat(status) {
    if (typeof status === 'undefined') {
      return this.getStatus().then((s) => this.getRepeat(s));
    } else if (status === null) {
      return Promise.resolve(null);
    } else {
      return Promise.resolve(status.repeat === '1');
    }
  }

  /**
   * Get the current volume.
   */
  getVolume(status) {
    if (typeof status === 'undefined') {
      return this.getStatus().then((s) => this.getVolume(s));
    } else if (status === null) {
      return Promise.resolve(null);
    } else {
      return Promise.resolve(parseInt(status.volume));
    }
  }

  /**
   * Get the current playback state.
   */
  getState(status) {
    if (typeof status === 'undefined') {
      return this.getStatus().then((s) => this.getState(s));
    } else if (status === null) {
      return Promise.resolve(null);
    } else {
      return Promise.resolve(status.state);
    }
  }

  /**
   * Get the current song.
   */
  getCurrentSong() {
    return this.sendCommand('currentsong');
  }

  /**
   * Get the artist of the current song.
   */
  getArtist(song) {
    if (typeof song === 'undefined') {
      return this.getCurrentSong().then((s) => this.getArtist(s));
    } else if (song === null) {
      return Promise.resolve(null);
    } else if (song.hasOwnProperty('Artist')) {
      return Promise.resolve(song.Artist);
    } else {
      return Promise.resolve(null);
    }
  }

  /**
   * Get the album of the current song.
   */
  getAlbum(song) {
    if (typeof song === 'undefined') {
      return this.getCurrentSong().then((s) => this.getAlbum(s));
    } else if (song === null) {
      return Promise.resolve(null);
    } else if (song.hasOwnProperty('Album')) {
      return Promise.resolve(song.Album);
    } else {
      return Promise.resolve(null);
    }
  }

  /**
   * Get the title of the current song.
   */
  getTitle(song) {
    if (typeof song === 'undefined') {
      return this.getCurrentSong().then((s) => this.getTitle(s));
    } else if (song === null) {
      return Promise.resolve(null);
    } else if (song.hasOwnProperty('Title')) {
      return Promise.resolve(song.Title);
    } else {
      return Promise.resolve(null);
    }
  }

  /**
   * Set the volume.
   */
  setVolume(level) {
    return this.sendCommand('setvol', level);
  }

  /**
   * Set the 'random' state.
   */
  setRandom(random) {
    return this.sendCommand('random', random ? 1 : 0);
  }

  /**
   * Set the 'repeat' state.
   */
  setRepeat(repeat) {
    return this.sendCommand('repeat', repeat ? 1 : 0);
  }

  /**
   * Update the playback state property.
   */
  updateState(state) {
    if (typeof state === 'undefined') {
      return this.getState().then((s) => this.updateState(s));
    }

    const prop = this.findProperty('state');
    return prop.value.notifyOfExternalUpdate(state);
  }

  /**
   * Update the album property.
   */
  updateAlbum(album) {
    if (typeof album === 'undefined') {
      return this.getAlbum().then((a) => this.updateAlbum(a));
    }

    const prop = this.findProperty('album');
    return prop.value.notifyOfExternalUpdate(album);
  }

  /**
   * Update the artist property.
   */
  updateArtist(artist) {
    if (typeof artist === 'undefined') {
      return this.getArtist().then((a) => this.updateArtist(a));
    }

    const prop = this.findProperty('artist');
    return prop.value.notifyOfExternalUpdate(artist);
  }

  /**
   * Update the title property.
   */
  updateTitle(title) {
    if (typeof title === 'undefined') {
      return this.getTitle().then((t) => this.updateTitle(t));
    }

    const prop = this.findProperty('title');
    return prop.value.notifyOfExternalUpdate(title);
  }

  /**
   * Start or resume playback.
   */
  play() {
    return this.getState().then((state) => {
      if (state === 'pause') {
        return this.sendCommand('pause', 0);
      } else if (state === 'stop') {
        return this.sendCommand('play', 0);
      }
    });
  }

  /**
   * Pause playback.
   */
  pause() {
    return this.getState().then((state) => {
      if (state === 'play') {
        return this.sendCommand('pause', 1);
      }
    });
  }

  /**
   * Stop playback.
   */
  stop() {
    return this.getState().then((state) => {
      if (['play', 'pause'].includes(state)) {
        return this.sendCommand('stop');
      }
    });
  }

  /**
   * Skip to the next song.
   */
  next() {
    return this.getState().then((state) => {
      if (['play', 'pause'].includes(state)) {
        return this.sendCommand('next');
      }
    });
  }

  /**
   * Skip to the previous song.
   */
  previous() {
    return this.getState().then((state) => {
      if (['play', 'pause'].includes(state)) {
        return this.sendCommand('previous');
      }
    });
  }

  /**
   * Get a list of all songs MPD knows about.
   */
  list() {
    return this.sendCommand('list', 'file');
  }

  /**
   * Add a song to the current playlist.
   */
  add(uri) {
    return this.sendCommand('add', uri);
  }

  /**
   * Get the current playlist.
   */
  getPlaylist() {
    return this.sendCommand('playlistinfo').then((playlist) => {
      if (playlist === null) {
        return null;
      }

      const songs = [];
      for (const song of playlist) {
        const artist = song.hasOwnProperty('Artist') ? song.Artist : 'Unknown';
        const title = song.hasOwnProperty('Title') ? song.Title : 'Unknown';
        songs.push(`${artist} - ${title}`);
      }

      return songs.join('\n');
    });
  }
}

/**
 * Create our MPD Web Thing and run the server.
 */
function runServer() {
  const thing = new MPDThing();

  const server = new WebThingServer([thing], null, 8888);

  process.on('SIGINT', () => {
    server.stop();
    process.exit();
  });

  server.start();
}

runServer();
