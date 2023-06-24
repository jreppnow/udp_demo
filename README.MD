# Connection-Like UDP API

This is a demo library to show how you can build a connection-like API for TCP sockets using Linux kernel UDP 
load-balancing based around `SO_REUSEADDR`.

It's a demo project without proper testing or any sort of quality check (besides running clippy..).

# Building/Running 

1. Build the project using `cargo b`.
2. Run a server using `./target/debug/echo-server`.
3. Start a client with `./target/debug/test-client`.
4. Repeat 3. as often as you like :) 

# License

This software is licensed under the Unlicense license. You can do whatever you want with it, no liability on my part though :) 