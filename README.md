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