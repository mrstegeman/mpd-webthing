package com.github.mrstegeman;

import org.apache.commons.collections.map.HashedMap;
import org.bff.javampd.file.MPDFile;
import org.bff.javampd.monitor.StandAloneMonitor;
import org.bff.javampd.player.PlayerBasicChangeEvent;
import org.bff.javampd.player.VolumeChangeEvent;
import org.bff.javampd.playlist.PlaylistBasicChangeEvent;
import org.bff.javampd.server.MPD;
import org.bff.javampd.server.ServerStatus;
import org.bff.javampd.song.MPDSong;
import org.json.JSONArray;
import org.json.JSONObject;
import org.mozilla.iot.webthing.Action;
import org.mozilla.iot.webthing.Event;
import org.mozilla.iot.webthing.Property;
import org.mozilla.iot.webthing.Thing;
import org.mozilla.iot.webthing.Value;
import org.mozilla.iot.webthing.WebThingServer;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;
import java.util.UUID;

/**
 * MPD client represented as a Web Thing.
 */
public class MPDThing extends Thing {
    private MPD client;
    private StandAloneMonitor monitor;
    private Value stateValue;
    private Value artistValue;
    private Value albumValue;
    private Value titleValue;
    private Value volumeValue;
    private Value repeatValue;
    private Value randomValue;

    /**
     * Initialize the thing.
     */
    public MPDThing() {
        super("urn:dev:ops:mpd",
              "MPD",
              new JSONArray(),
              "Music Player Daemon");

        // Connect to MPD.
        this.client = new MPD.Builder().server("localhost").port(6600).build();

        this.monitor = this.client.getMonitor();
        this.monitor.addPlaylistChangeListener((PlaylistBasicChangeEvent event) -> {
            String playlist = this.getPlaylist();
            this.addEvent(new PlaylistUpdatedEvent(this, playlist));

            MPDSong song = this.getCurrentSong();
            this.updateAlbum(this.getAlbum(song));
            this.updateArtist(this.getArtist(song));
            this.updateTitle(this.getTitle(song));
        });
        this.monitor.addPlayerChangeListener((PlayerBasicChangeEvent event) -> {
            ServerStatus status = this.getStatus();
            this.updateState(status);
            this.updateRepeat(status);
            this.updateRandom(status);
        });
        this.monitor.addVolumeChangeListener((VolumeChangeEvent event) -> {
            this.updateVolume(event.getVolume());
        });

        // Start a separate thread to watch for repeat/random/state events, as
        // the monitor above is unreliable for those.
        new Thread(() -> {
            while (true) {
                try {
                    Thread.sleep(2000);
                    ServerStatus status = this.getStatus();
                    this.updateState(status);
                    this.updateRepeat(status);
                    this.updateRandom(status);
                } catch (InterruptedException e) {
                    break;
                }
            }
        }).start();

        ServerStatus status = this.client.getServerStatus();

        // Add a 'volume' property.
        JSONObject volumeMetadata = new JSONObject();
        volumeMetadata.put("@type", "LevelProperty");
        volumeMetadata.put("type", "number");
        volumeMetadata.put("description", "Playback volume");
        volumeMetadata.put("minimum", 0);
        volumeMetadata.put("maximum", 100);
        volumeMetadata.put("unit", "percent");
        volumeMetadata.put("label", "Volume");
        this.volumeValue =
                new Value(status.getVolume(), v -> this.setVolume((int)v));
        this.addProperty(new Property(this,
                                      "volume",
                                      this.volumeValue,
                                      volumeMetadata));

        // Add a 'repeat' property.
        JSONObject repeatMetadata = new JSONObject();
        repeatMetadata.put("@type", "BooleanProperty");
        repeatMetadata.put("type", "boolean");
        repeatMetadata.put("description", "Repeat mode");
        repeatMetadata.put("label", "Repeat");
        this.repeatValue =
                new Value(status.isRepeat(), r -> this.setRepeat((boolean)r));
        this.addProperty(new Property(this,
                                      "repeat",
                                      this.repeatValue,
                                      repeatMetadata));

        // Add a 'random' property.
        JSONObject randomMetadata = new JSONObject();
        randomMetadata.put("@type", "BooleanProperty");
        randomMetadata.put("type", "boolean");
        randomMetadata.put("description", "Random mode");
        randomMetadata.put("label", "Random");
        this.randomValue =
                new Value(status.isRandom(), r -> this.setRandom((boolean)r));
        this.addProperty(new Property(this,
                                      "random",
                                      this.randomValue,
                                      randomMetadata));

        // Add a 'state' property, which indicates playback state.
        JSONObject stateMetadata = new JSONObject();
        stateMetadata.put("type", "string");
        stateMetadata.put("enum", Arrays.asList("play", "pause", "stop"));
        stateMetadata.put("description", "Current playback state");
        stateMetadata.put("label", "State");
        stateMetadata.put("readOnly", true);
        this.stateValue = new Value(status.getState());
        this.addProperty(new Property(this,
                                      "state",
                                      this.stateValue,
                                      stateMetadata));

        MPDSong song = this.client.getPlayer().getCurrentSong();

        // Add an 'artist' property.
        JSONObject artistMetadata = new JSONObject();
        artistMetadata.put("type", "string");
        artistMetadata.put("description", "Artist of current song");
        artistMetadata.put("label", "Artist");
        artistMetadata.put("readOnly", true);
        this.artistValue = new Value(song.getArtistName());
        this.addProperty(new Property(this,
                                      "artist",
                                      this.artistValue,
                                      artistMetadata));

        // Add an 'album' property.
        JSONObject albumMetadata = new JSONObject();
        albumMetadata.put("type", "string");
        albumMetadata.put("description", "Album current song belongs to");
        albumMetadata.put("label", "Album");
        albumMetadata.put("readOnly", true);
        this.albumValue = new Value(song.getAlbumName());
        this.addProperty(new Property(this,
                                      "album",
                                      this.albumValue,
                                      albumMetadata));

        // Add a 'title' property.
        JSONObject titleMetadata = new JSONObject();
        titleMetadata.put("type", "string");
        titleMetadata.put("description", "Title of current song");
        titleMetadata.put("label", "Title");
        titleMetadata.put("readOnly", true);
        this.titleValue = new Value(song.getTitle());
        this.addProperty(new Property(this,
                                      "title",
                                      this.titleValue,
                                      titleMetadata));

        // Add a 'play' action.
        JSONObject playMetadata = new JSONObject();
        playMetadata.put("description", "Start playback");
        playMetadata.put("label", "Play");
        this.addAvailableAction("play", playMetadata, PlayAction.class);

        // Add a 'pause' action.
        JSONObject pauseMetadata = new JSONObject();
        pauseMetadata.put("description", "Pause playback");
        pauseMetadata.put("label", "Pause");
        this.addAvailableAction("pause", pauseMetadata, PauseAction.class);

        // Add a 'stop' action.
        JSONObject stopMetadata = new JSONObject();
        stopMetadata.put("description", "Stop playback");
        stopMetadata.put("label", "Stop");
        this.addAvailableAction("stop", stopMetadata, StopAction.class);

        // Add a 'next' option.
        JSONObject nextMetadata = new JSONObject();
        nextMetadata.put("description", "Skip to next song");
        nextMetadata.put("label", "Next");
        this.addAvailableAction("next", nextMetadata, NextAction.class);

        // Add a 'previous' action.
        JSONObject previousMetadata = new JSONObject();
        previousMetadata.put("description", "Skip to previous song");
        previousMetadata.put("label", "Previous");
        this.addAvailableAction("previous",
                                previousMetadata,
                                PreviousAction.class);

        // Add a 'queueRandom' action.
        JSONObject queueRandomMetadata = new JSONObject();
        queueRandomMetadata.put("description",
                                "Queue a series of random songs");
        queueRandomMetadata.put("label", "Queue Random");
        Map<String, Object> queueRandomInputMetadata = new HashMap<>();
        queueRandomInputMetadata.put("type", "object");
        queueRandomInputMetadata.put("required", new String[]{"count"});
        Map<String, Object> queueRandomInputPropertiesMetadata =
                new HashMap<>();
        Map<String, Object> queueRandomInputPropertiesCountMetadata =
                new HashedMap();
        queueRandomInputPropertiesCountMetadata.put("type", "number");
        queueRandomInputPropertiesCountMetadata.put("minimum", 1);
        queueRandomInputPropertiesMetadata.put("count",
                                               queueRandomInputPropertiesCountMetadata);
        queueRandomInputMetadata.put("properties",
                                     queueRandomInputPropertiesMetadata);
        queueRandomMetadata.put("input", queueRandomInputMetadata);
        this.addAvailableAction("queueRandom",
                                queueRandomMetadata,
                                QueueRandomAction.class);

        // Add a 'playlistUpdated' event.
        JSONObject playlistUpdatedMetadata = new JSONObject();
        playlistUpdatedMetadata.put("description",
                                    "The current playlist has been updated");
        playlistUpdatedMetadata.put("type", "string");
        this.addAvailableEvent("playlistUpdated", playlistUpdatedMetadata);

        // Start monitoring for events.
        this.monitor.start();
    }

    /**
     * Create our MPD Web Thing and run the server.
     */
    public static void main(String[] args) {
        MPDThing thing = new MPDThing();

        try {
            WebThingServer server =
                    new WebThingServer(new WebThingServer.SingleThing(thing),
                                       8888);

            Runtime.getRuntime()
                   .addShutdownHook(new Thread(() -> server.stop()));

            server.start(false);
        } catch (IOException e) {
            System.out.println(e);
            System.exit(1);
        }
    }

    /**
     * Get the current status.
     */
    private ServerStatus getStatus() {
        return this.client.getServerStatus();
    }

    /**
     * Get the current volume.
     */
    private int getVolume(ServerStatus status) {
        if (status == null) {
            status = this.getStatus();
        }

        return status.getVolume();
    }

    /**
     * Get the current 'random' state.
     */
    private boolean getRandom(ServerStatus status) {
        if (status == null) {
            status = this.getStatus();
        }

        return status.isRandom();
    }

    /**
     * Get the current 'repeat' state.
     */
    private boolean getRepeat(ServerStatus status) {
        if (status == null) {
            status = this.getStatus();
        }

        return status.isRepeat();
    }

    /**
     * Get the current playback state.
     */
    private String getState(ServerStatus status) {
        if (status == null) {
            status = this.getStatus();
        }

        return status.getState();
    }

    /**
     * Get the current song.
     */
    private MPDSong getCurrentSong() {
        return this.client.getPlayer().getCurrentSong();
    }

    /**
     * Get the artist of the current song.
     */
    private String getArtist(MPDSong song) {
        if (song == null) {
            song = this.getCurrentSong();
        }

        return song.getArtistName();
    }

    /**
     * Get the album of the current song.
     */
    private String getAlbum(MPDSong song) {
        if (song == null) {
            song = this.getCurrentSong();
        }

        return song.getAlbumName();
    }

    /**
     * Get the title of the current song.
     */
    private String getTitle(MPDSong song) {
        if (song == null) {
            song = this.getCurrentSong();
        }

        return song.getTitle();
    }

    /**
     * Set the volume.
     */
    private void setVolume(int level) {
        this.client.getPlayer().setVolume(level);
    }

    /**
     * Set the 'random' state.
     */
    private void setRandom(boolean random) {
        this.client.getPlayer().setRandom(random);
    }

    /**
     * Set the 'repeat' state.
     */
    private void setRepeat(boolean repeat) {
        this.client.getPlayer().setRepeat(repeat);
    }

    /**
     * Update the volume property.
     */
    private void updateVolume(int volume) {
        if (volume < 0) {
            volume = this.getVolume(null);
        }

        this.volumeValue.notifyOfExternalUpdate(volume);
    }

    /**
     * Update the random property.
     */
    private void updateRandom(ServerStatus status) {
        if (status == null) {
            status = this.getStatus();
        }

        this.randomValue.notifyOfExternalUpdate(this.getRandom(status));
    }

    /**
     * Update the repeat property.
     */
    private void updateRepeat(ServerStatus status) {
        if (status == null) {
            status = this.getStatus();
        }

        this.repeatValue.notifyOfExternalUpdate(this.getRepeat(status));
    }

    /**
     * Update the playback state property.
     */
    private void updateState(ServerStatus status) {
        if (status == null) {
            status = this.getStatus();
        }

        this.stateValue.notifyOfExternalUpdate(this.getState(status));
    }

    /**
     * Update the album property.
     */
    private void updateAlbum(String album) {
        if (album == null) {
            album = this.getAlbum(null);
        }

        this.albumValue.notifyOfExternalUpdate(album);
    }

    /**
     * Update the artist property.
     */
    private void updateArtist(String artist) {
        if (artist == null) {
            artist = this.getArtist(null);
        }

        this.artistValue.notifyOfExternalUpdate(artist);
    }

    /**
     * Update the title property.
     */
    private void updateTitle(String title) {
        if (title == null) {
            title = this.getTitle(null);
        }

        this.titleValue.notifyOfExternalUpdate(title);
    }

    /**
     * Start or resume playback.
     */
    private void play() {
        this.client.getPlayer().play();
    }

    /**
     * Pause playback.
     */
    private void pause() {
        this.client.getPlayer().pause();
    }

    /**
     * Stop playback.
     */
    private void stop() {
        this.client.getPlayer().stop();
    }

    /**
     * Skip to the next song.
     */
    private void next() {
        this.client.getPlayer().playNext();
    }

    /**
     * Skip to the previous song.
     */
    private void previous() {
        this.client.getPlayer().playPrevious();
    }

    /**
     * Get a list of all songs MPD knows about.
     */
    private List<MPDFile> list() {
        return this.listDirectory(null);
    }

    private List<MPDFile> listDirectory(MPDFile directory) {
        Collection<MPDFile> collection;
        if (directory == null) {
            collection = this.client.getMusicDatabase()
                                    .getFileDatabase()
                                    .listRootDirectory();
        } else {
            collection = this.client.getMusicDatabase()
                                    .getFileDatabase()
                                    .listDirectory(directory);
        }

        List<MPDFile> files = new ArrayList<>();
        for (MPDFile file : collection) {
            if (file.isDirectory()) {
                files.addAll(this.listDirectory(file));
            } else {
                files.add(file);
            }
        }

        files.sort(Comparator.comparing(MPDFile::getPath));
        return files;
    }

    /**
     * Add a song to the current playlist.
     */
    private void add(MPDFile mpdFile) {
        this.client.getPlaylist().addFileOrDirectory(mpdFile);
    }

    /**
     * Get the current playlist.
     */
    private String getPlaylist() {
        List<String> songs = new ArrayList<>();

        for (MPDSong song : this.client.getPlaylist().getSongList()) {
            songs.add(String.format("%s - %s",
                                    song.getArtistName(),
                                    song.getTitle()));
        }

        return String.join("\n", songs);
    }

    /**
     * Action to start playback.
     */
    public static class PlayAction extends Action {
        /**
         * Initialize the action.
         */
        public PlayAction(Thing thing, JSONObject input) {
            super(UUID.randomUUID().toString(), thing, "play", input);
        }

        /**
         * Perform the action, i.e. start playback.
         */
        @Override
        public void performAction() {
            ((MPDThing)this.getThing()).play();
        }
    }

    /**
     * Action to pause playback.
     */
    public static class PauseAction extends Action {
        /**
         * Initialize the action.
         */
        public PauseAction(Thing thing, JSONObject input) {
            super(UUID.randomUUID().toString(), thing, "pause", input);
        }

        /**
         * Perform the action, i.e. pause playback.
         */
        @Override
        public void performAction() {
            ((MPDThing)this.getThing()).pause();
        }
    }

    /**
     * Action to stop playback.
     */
    public static class StopAction extends Action {
        /**
         * Initialize the action.
         */
        public StopAction(Thing thing, JSONObject input) {
            super(UUID.randomUUID().toString(), thing, "stop", input);
        }

        /**
         * Perform the action, i.e. stop playback.
         */
        @Override
        public void performAction() {
            ((MPDThing)this.getThing()).stop();
        }
    }

    /**
     * Action to skip to the next song.
     */
    public static class NextAction extends Action {
        /**
         * Initialize the action.
         */
        public NextAction(Thing thing, JSONObject input) {
            super(UUID.randomUUID().toString(), thing, "next", input);
        }

        /**
         * Perform the action, i.e. skip song.
         */
        @Override
        public void performAction() {
            ((MPDThing)this.getThing()).next();
        }
    }

    /**
     * Action to skip to the previous song.
     */
    public static class PreviousAction extends Action {
        /**
         * Initialize the action.
         */
        public PreviousAction(Thing thing, JSONObject input) {
            super(UUID.randomUUID().toString(), thing, "previous", input);
        }

        /**
         * Perform the action, i.e. skip song.
         */
        @Override
        public void performAction() {
            ((MPDThing)this.getThing()).previous();
        }
    }

    /**
     * Action to queue random songs.
     */
    public static class QueueRandomAction extends Action {
        /**
         * Initialize the action.
         */
        public QueueRandomAction(Thing thing, JSONObject input) {
            super(UUID.randomUUID().toString(), thing, "queueRandom", input);
        }

        /**
         * Perform the action, i.e. queue songs.
         */
        @Override
        public void performAction() {
            MPDThing thing = (MPDThing)this.getThing();
            Random random = new Random();
            List<MPDFile> songs = thing.list();

            for (int i = 0; i < this.getInput().getInt("count"); ++i) {
                MPDFile file = songs.get(random.nextInt(songs.size()));
                thing.add(file);
            }

            // Since we just updated the playlist, emit an event.
            String playlist = thing.getPlaylist();
            thing.addEvent(new PlaylistUpdatedEvent(thing, playlist));
        }
    }

    /**
     * Event to indicate that the current playlist was updated.
     */
    public static class PlaylistUpdatedEvent extends Event {
        /**
         * Initialize the event.
         */
        public PlaylistUpdatedEvent(Thing thing, String data) {
            super(thing, "playlistUpdated", data);
        }
    }
}
