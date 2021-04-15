http_server:
	cd frontend && python -m http.server &

envoy_proxy:
	cd frontend && docker run -v "$$(pwd)"/envoy.yaml:/etc/envoy/envoy.yaml:ro -p 8080:8080 -p 9901:9901 --network=host envoyproxy/envoy:v1.17.1

run: http_server envoy_proxy
