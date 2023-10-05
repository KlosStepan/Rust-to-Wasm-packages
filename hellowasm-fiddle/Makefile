PKG_DIR = pkg
TARGET_DIR = target
PID_FILE = server.pid
 
.PHONY: default

default:
	$(MAKE) run	

build:
	# Build 'wasm-pack build --target web used for all scenarios'.
	wasm-pack build --target web

run: build
	# Run 'Start Python3 webserver, app on localhost:8000' and CTRL+C in terminal.
	python3 -m http.server 8000

run-background: build
	# Run background 'wasm-pack build --target web & start Python3 webserver on localhost:8000' , stop background process by `make stop` pkill PID
	nohup python3 -m http.server 8000 & echo $$! > $(PID_FILE)

stop:
	# Stop the server if the PID file exists
	@if [ -f $(PID_FILE) ]; then \
		(pkill -F $(PID_FILE); rm -f $(PID_FILE)); \
	fi

clean:
	# Remove the pkg and target directories if they exist
	rm -rf $(PKG_DIR) $(TARGET_DIR)