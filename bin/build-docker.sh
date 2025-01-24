#!/bin/bash
bash bin/build.sh

cd bin || exit

docker build -t web.yunnet.top:12500/docksense .