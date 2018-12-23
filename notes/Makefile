PDFPARAMS=--toc -N -s -t latex --pdf-engine=xelatex -V documentclass=scrartcl -V papersize=a4 -V lang=de -V urlcolor=blue -V linkcolor=blue
FONTS=-V mainfont='DejaVu Serif' -V sansfont='DejaVu Sans' -V monofont='DejaVu Sans Mono'
DATE=-V date="`date +'%d.%m.%Y'`"

all: notes.pdf

notes.pdf: *.md
	pandoc $(PDFPARAMS) $(FONTS) $(DATE) *.md -o notes.pdf

clean:
	rm -f notes.pdf
