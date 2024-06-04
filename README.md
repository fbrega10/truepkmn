[![Rust Community](https://img.shields.io/badge/Rust_Community%20-Join_us-brightgreen?style=plastic&logo=rust)](https://www.rust-lang.org/community)

![image info](truepkmn.png)


## How to run this app?

### Quick Start
Rust (v 1.76 or later) is needed to run the code, you can install the latest stable release  in Unix-based like systems with:

* ``` sh curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh ```

I assume you have cURL installed (for api testing), if not get it 

* ``` sudo apt update && sudo apt install curl ```

It could be useful to also have jq (the json formatter), which can also be installed as cURL : 

* ``` sudo apt install jq ```

Being in the root of the folder, run this command to run the app :

* ``` cargo run  ```

Under /bash you can find some curl examples to inquiry the server : 


* ``` sh /src/bash/test-pokemon-translated.sh | jq .  ```



If you want to run the app as a Docker container you should:

1.  Build the docker image with : 

 ``` docker build -t truepkmn  ```

2.  Run the image as a container (in detached mode) :

``` docker run -d --name truepkmn -p 8080:8080 truepkmn ```

#### Now you're ready to go! Gotta catch 'em all!'


### What would I'd do for a production API?

Considering that the same pokemon could be called several times in different
requests, caching could be an interesting idea to improve the overall throughput 
of the service as it wouldn't require to call any external api's (pokeapi.co and funtranslations).

As we're saying that for multiple requests (requiring the same resource) we could have some benefits,
we should also consider that it introduces the problem of cache invalidation (when should I remove
an element from my cache? How can I make this mechanism work?), which is not easy to solve.

For a production backend API I would rather use gRPC over REST: the data exchange happens
through Protobufs, which are lighter than JSON messages and they come for free as you
compile using the protoc utility (for most of the languages).
The only thing you need in this case is a .proto file defining your service and the objects you wish to use.
Grpc is faster than REST and it offers real-time streaming, allowing clients and servers to send and receive messages
bidirectionally.
