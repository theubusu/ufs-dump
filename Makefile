PREFIX = /usr/local
MANPREFIX = ${PREFIX}/share/man
FUSE_UFS_FLAGS = -p ufs-dump --ignore-rust-version --no-default-features

SRC != find rufs/src ufs-dump/src -name '*.rs'

all: fuse-ufs-bin

clean:
	rm -f ufs-dump-bin
	cargo clean
	find . -name '*.core' -delete -print
	find . -name '*.orig' -delete -print
	find . -name '*.rej' -delete -print
	rm -f .patch

fuse-ufs-bin: Cargo.lock ${SRC}
	cargo build --release ${FUSE_UFS_FLAGS}
	cp -f target/release/ufs-dump ufs-dump-bin

