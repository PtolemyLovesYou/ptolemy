.PHONY: format
format:
	black tvali tvali-client tvali-utils

.PHONY: install
install:
	pip install -e "tvali[client,api]"
