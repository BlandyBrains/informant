# informant
A Rust library and tool for extracting metadata from various media formats.


## Installation


### Exiv2 (XMP) 
```
apt install libgexiv2-dev
```

### Archive

```

INF_DB_FILE=informant.db3 find originals -type f -exec informant {} archive --directory informant \;


INF_DB_FILE=informant.db3 find originals -type f -exec informant {} archive --create-database true --directory informant \;

RUST_LOG=info INF_DB_FILE=informant.db3 find processing/png -type f -exec informant {} archive --directory informant \;

```

### Usage

```
# build container
podman build -t informant -f Containerfile.debian.cli

# start container 
podman run -tdi --name informant -v informant_storage:/data -v /mnt/juggernaut/flash:/flash informant

# attach to container
podman exec -ti informant bash

# run
INF_DB_FILE=/data/informant.db3 find /flash/processing -type f -name *.jpg -exec informant {} archive --create-database true --directory /flash/informant \;
```


### Common Commands

```
# find unique file extensions
find . -type f | sed -n 's/.*\.\([a-zA-Z0-9]*\)$/\1/p' | sort -u

```