# HyppoTunes Server
Server application for the HyppoTunes mobile app.

Presents to the mobile app data about songs stored in the /files folder via a gRPC connection.

Uses a SQLite database to store info about these songs for faster access. Updates the database with info each time the server is started, by default.

Allows the mobile app to download songs from the /files folder each time a request is received.

Compiled exe binary is available in the repository, in the case of missing compiler on the side of the one cloning the repository.
