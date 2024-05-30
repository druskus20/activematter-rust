docker build -t build-image .
docker run --name build-container build-image
docker cp build-container:/build ./build
docker rm build-container
