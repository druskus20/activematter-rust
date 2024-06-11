docker build -t build-image2 .
docker run --name build-container2 build-image2
docker cp build-container2:/build2 ./build2
docker rm build-container2
