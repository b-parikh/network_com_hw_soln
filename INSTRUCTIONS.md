### Introduction

There is currently support for two communication methods:
1. Raw TCP sockets
2. NNG sockets (https://nng.nanomsg.org/)

NNG is a messaging framework that is very similar to ZeroMQ. It requires no broker and implements
common messaging patterns such as publisher-subscriber, request-reply, pipes, etc. For sake of simplicity,
only the `Pair` messaging pattern was implemented in this project. Note that in this project, the 
underlying transport protocol for NNG sockets is still TCP.

In general, there were quite a few shortcuts taken for the sake of simplicity. Read the Appendix to
understand the extensions that can be made to the project.

### Build Docker container
Build this code within a Docker container to ensure that the correct dependencies are pulled in.

```
sudo docker build --pull --file Dockerfile -t network_com_hw_soln:latest .
```

### Enter Docker container
Start an interactive shell session in the built Docker container:

```
docker run -v $PWD:/mnt/network_com_hw_soln --network host -it --entrypoint /bin/bash network_com_hw_soln:latest
```

Then, start another interactive shell session in the same Docker container:
```
docker ps # Find the name of the running container
docker exec -it ${NAME_OF_RUNNING_CONTAINER} /bin/bash
```

### Build code
Once you've entered the Docker container, run `cargo build` to run the Rust compiler and build the code. After
it's built, you should see two binaries: `/mnt/network_com_hw_soln/target/debug/client` and 
`/mnt/network_com_hw_soln/target/debug/client`.

### Running code

Run
```
./target/debug/server --help
./target/debug/client --help
```
for a list of options with their descriptions.

Assuming you're in the `/mnt/network_com_hw_soln` directory within the Docker container, you can run this command
in one shell to start the server
```
./target/debug/server --client-recv-socket-addr "127.0.0.1:6666" --client-transport-protocol nng --server-recv-socket-addr "127.0.0.1:5555" --server-transport-protocol tcp
```

and this command in another shell to start the client
```
./target/debug/client --stl-file-path ./cad_mesh.stl --client-transport-protocol nng --server-transport-protocol tcp --client-recv-socket-addr "127.0.0.1:6666" --server-recv-socket-addr "127.0.0.1:5555"
```

In this example, the server is expecting messages on an `nng` socket from the client and it will sending messages back using a TCP socket. The client is expecting to send messages using an NNG
socket and it will expect responses on a TCP socket.

The server expects to receive messages on `0.0.0.0:5555` and sends responses to `localhost:6666`.

### Verifying correctness

Run
```
diff output.stl cad_mesh.stl
```
to verify that there is no change between the original file and the received file.


### Appendix
#### Shortcuts
1. The server can handle only one message, and it cannot handle multiple concurrent client requests.
2. The client and the server have to be started with the correct CLI arguments to make sure they communicate properly. For example, if the client was
started with `--server-transport-protocol tcp` and the server was started with `--server-transport-protocol nng`, the processes would fail to communicate.
3. Lack of generics used to abstract network protocols and transport mechanisms.
