CC=cargo b

default: release

release: FORCE
	$(CC) --release

clean: FORCE
	-rm -r target

FORCE:
