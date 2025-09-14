build:
	wasm-pack build --target web

serve: build
	python3 -m http.server 8080