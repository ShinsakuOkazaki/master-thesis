all: create  clean

create:
	pdflatex  paper-main.tex
	bibtex    paper-main
	pdflatex  paper-main.tex

clean:
	rm -rf *.aux  *.nav  *.vrb  *.log  *.out *.snm *.toc *.gz *.bbl  *.blg *.fdb_latexmk *.fls
