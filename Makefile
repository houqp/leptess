check:
	cargo clippy
	make test

test:
	cargo test

gen:
	GEN_BINDING=1 cargo build

set_variables_list.txt:
	tesseract --print-parameters > $@
