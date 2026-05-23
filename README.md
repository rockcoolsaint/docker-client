## Recommended IDE Setup

- Create .env file at the root of the project 
- add the variable UNIX_SOCKET_PATH
- eg. UNIX_SOCKET_PATH="Docker-unix-endpoint"

To get the docker UNIX_SOCKET_PATH, do the following:
Launch your terminal and type the following:

`docker context inspect` and look for the following in the result

```
"Endpoints": {
            "docker": {
                "Host": "unix:///home/rockxxxsaint/.docker/desktop/docker.sock",
                "SkipTLSVerify": false
            }
        },
```


Copy the "Host" key value, i.e "unix:///home/rockxxxsaint/.docker/desktop/docker.sock" as your environmental variable
