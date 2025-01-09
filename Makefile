# 变量定义
APP_NAME := instant-monitor
VERSION := v0.1.0
DOCKER_IMAGE := $(APP_NAME):$(VERSION)
DOCKER_LATEST := $(APP_NAME):latest

# 构建 Docker 镜像
.PHONY: build
build:
	docker build -t $(DOCKER_IMAGE) -t $(DOCKER_LATEST) .

# 运行容器
.PHONY: run
run:
	docker-compose up -d

# 停止容器
.PHONY: stop
stop:
	docker-compose down

.PHONY: logs
logs:
	docker-compose logs -f

