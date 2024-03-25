# networking and async

## step 1: simple tcp chat server

2 binaries: 1 for server, 1 for client

flow:

1. start server
1. start client. client connects to server
1. client reads from stdin and sends to server
1. server reads from client and writes to stdout
1. server reads from stdin and writes to client
1. client reads from server and writes to stdout
1. repeat 3-5 until client or server sends `exit`
1. close client and server

notes:

* should be implemented with sockets

## step 2: extending chat server

flow:

1. start server
1. start client. client connects to server
1. client identifies itself with a username
1. server sends a message to all existing clients about the new client
1. client sends a message to server
1. server relays that message to all other clients
1. when a client disconnects, server sends a message to all existing clients
about the client that disconnected

notes:

* server should not allow duplicate usernames or sending messages without
identifying first
