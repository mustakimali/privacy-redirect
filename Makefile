VERSION?=local
IMAGE_NAME?=mustakimali/privacy-redirect

.PHONY: docker-build
docker-build:
	docker build -t $(IMAGE_NAME):$(VERSION) -f Dockerfile .

.PHONY: build
build:
	@cargo build

	
.PHONY: docker-run
docker-run:
	docker run -ti --rm --name onemillion --publish 8080:8080 $(IMAGE_NAME):$(VERSION)

.PHONY: docker-push
docker-push:
	./Dockerpush.sh
