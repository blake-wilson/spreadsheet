http_server:
	cd frontend && python -m http.server &

envoy_proxy:
	cd frontend && docker run -v "$$(pwd)"/envoy.yaml:/etc/envoy/envoy.yaml:ro -p 8080:8080 -p 9901:9901 --network=host envoyproxy/envoy:v1.17.1

frontend:
	cd frontend/spreadsheet-client && npm run start

run:
	cd frontend && docker run -v "$$(pwd)"/envoy.yaml:/etc/envoy/envoy.yaml:ro -p 8080:8080 -p 9901:9901 --network=host envoyproxy/envoy:v1.17.1 &
	cd frontend/spreadsheet-client && npm run start &
	cd frontend && python -m http.server &


generate_rs_protobuf:
	cargo build

generate_js_protobuf:
	export PATH="$$PATH:$$(pwd)/frontend"; cd frontend && protoc -I=../src/proto/grpc ../src/proto/grpc/api.proto --js_out=import_style=commonjs:. --grpc-web_out=import_style=commonjs,mode=grpcwebtext:.


build-docker:
	docker build . -t spreadsheet

push-docker:
	aws ecr get-login-password  --region region | docker login --username --password-stdin ${DOCKER_HOST}
	docker tag spreadsheet:latest ${DOCKER_HOST}/spreadsheet:latest
	docker push ${DOCKER_HOST}/spreadsheet:latest
