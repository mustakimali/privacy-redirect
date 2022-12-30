VERSION?=local
IMAGE_NAME?=mustakimali/private

.PHONY: docker-build
docker-build:
	docker build -t $(IMAGE_NAME):$(VERSION) -f Dockerfile .

.PHONY: build
build:
	@cargo build

	
.PHONY: docker-run
docker-run:
	docker run -ti --rm --name privacy-redirect --publish 8080:8080 $(IMAGE_NAME):$(VERSION)
