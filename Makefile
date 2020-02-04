setup:
	mkdir sandbox
	cargo build

test: sandbox target/debug/nymphaea
	cargo build
	cd sandbox && \
	touch hoge.py && \
	echo "import __hello__" > hoge.py && \
	../target/debug/nymphaea init && \
	../target/debug/nymphaea add && \
	../target/debug/nymphaea commit "hoge.py"

show: sandbox/.nymphaea
	cat sandbox/.nymphaea/index
	./testool decompress

clean:
	rm -rf sandbox
