# HyppoTunes Server
Server application for the HyppoTunes mobile app.

Uses a gRPC connection to present data to the HyppoTunes mobile app about mp3 files stored in the /files folder.

Uses a SQLite database to store info about the mp3 files for faster access. Updates the database with info each time a new mp3 file is inserted into the /files folder.

Allows the HyppoTunes mobile app to download available mp3 files.
