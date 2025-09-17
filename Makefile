build-dev:
	wasm-pack build --target web --dev

build:
	wasm-pack build --target web

serve-dev: build-dev
	python3 -m http.server 8080

serve: build
	python3 -m http.server 8080

docker-build:
	docker build -t r-os .