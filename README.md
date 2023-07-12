There are 2 contracts.

There will be a 3 way handshake between the contracts, which starts from the ping contract's
ping functions

1. Ping which has a fun
    ping()
        which calls the pong contract's pong() and stores its address, 
        it expects a response from pong contract.
        it increments the ping count, in its own storage.
        assert(pongResponse == pingCount)

2. Pong contract 
    pong()
        which works as a handlePing() function, returns a response to the ping contract.
        returns the serial no of ping,
        and increments the ping count, in its own storage.