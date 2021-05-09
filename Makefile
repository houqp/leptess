check:
	cargo clippy
	make test

test:
	cargo test

gen:
	GEN_BINDING=1 cargo build

variables_list.txt:
	tesseract --print-parameters > $@

src/variable.rs: variables_list.txt build_variables.py
	python build_variables.py < variables_list.txt | rustfmt > $@
