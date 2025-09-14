build:
	wasm-pack build --target web --dev

serve: build
	python3 -m http.server 8080