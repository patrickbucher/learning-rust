PDF_PARAMS=-N -s --toc --pdf-engine=xelatex -V documentclass=scrartcl -V papersize=a4 -V lang=en -V fontsize=11pt -V geometry=left=2cm,right=2cm,top=2cm,bottom=4cm
FONT_PARAMS=-V mainfont="DejaVu Serif" -V sansfont="DejaVu Sans" -V monofont="DejaVu Sans Mono"
DATE=-V date="`date +'%Y-%m-%d'`"

all: learning-rust-notes.epub learning-rust-notes.pdf

learning-rust-notes.epub: *.md
	pandoc -s -N --toc *.md -o $@

learning-rust-notes.pdf: *.md
	pandoc $(PDF_PARAMS) $(FONT_PARAMS) $(DATE) *.md -o $@

clean:
	rm -f learning-rust-notes.epub learning-rust-notes.pdf
