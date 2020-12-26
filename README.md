# glot-run


## Overview
glot-run adds user management to [docker-run](https://github.com/glotcode/docker-run).
Note that glot-run will be be incorporated into [glot-www](https://github.com/prasmussen/glot-www) in the near future. Do not expect any new features in this repo.
If you have user management needs that is not covered by this project I would recommend implementing that yourself in your own application.
See the [overview](https://github.com/prasmussen/glot) on how everything is connected.


## Run
Download the latest release and start it with the required environment variables.
See the [systemd service](https://github.com/glotcode/glot-run/blob/main/systemd/glot-run.service) as an example how to start it.


## Environment variables

#### Required

| Variable name                          | Type                          | Description                                                                  |
|:---------------------------------------|:------------------------------|:-----------------------------------------------------------------------------|
| SERVER_LISTEN_ADDR                     | &lt;ipv4 address&gt;          | Listen ip                                                                    |
| SERVER_LISTEN_PORT                     | 1-65535                       | Listen port                                                                  |
| SERVER_WORKER_THREADS                  | &lt;integer&gt;               | How many simultaneous requests that should be processed                      |
| SERVER_BASE_URL                        | &lt;url&gt;                   | Base url where the service is hosted. i.e. (http://localhost:8089)           |
| SERVER_DATA_ROOT                       | &lt;path&gt;                  | Path to where the data files should be saved                                 |
| API_ADMIN_ACCESS_TOKEN                 | &lt;string&gt;                | Access token for the admin api                                               |
| DOCKER_RUN_BASE_URL                    | &lt;url&gt;                   | Url to docker-run                                                            |
| DOCKER_RUN_ACCESS_TOKEN                | &lt;string&gt;                | docker-run access token


## Api users
An api token is required to run code. Users can be created with the `/admin/users` endpoint.
See the [api docs](https://github.com/prasmussen/glot-run/tree/master/api_docs/admin) for more details.

## Languages
Languages can be added with the `/admin/languages` endpoint. A language has
a name, version and the name of a docker image that will be used when running
code for the given language/version.
See the [api docs](https://github.com/prasmussen/glot-run/tree/master/api_docs/admin) for more details.
