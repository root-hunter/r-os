build-dev:
	rm -rf static/pkg
	wasm-pack build --target web --dev
	mv pkg static/

build:
	rm -rf static/pkg
	wasm-pack build --target web --release
	mv pkg static/

serve-dev: build-dev
	python3 -m http.server 8080

serve: build
	python3 -m http.server 8080

docker-build:
	docker build -t r-os .