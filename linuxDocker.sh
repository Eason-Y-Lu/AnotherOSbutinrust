#! /bin/bash
xauth list >xauth.txt
docker build -t anotheros .
docker run -d --rm --name another_os --net=host --volume /tmp/.X11-unix:/tmp/.X11-unix --env DISPLAY=$DISPLAY anotheros sleep 300
docker cp xauth.txt another_os:xauth.txt
docker cp bash_script/stage2.sh another_os:stage2.sh
shred -u xauth.txt
docker exec -it another_os ./stage2.sh
