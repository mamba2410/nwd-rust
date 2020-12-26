#!/bin/sh

language_home="$1/languages/latex"
project_name="$2"
do_init=$3

rm -r include
mkdir outs images

cp $language_home/Makefile ./Makefile
cp $language_home/build_number.mak ./build/build_number.mak

project_name_upper=$(echo $project_name | tr '[:lower:]' '[:upper:]')
project_name_upper=$(echo $project_name_upper | sed 's/[^[:alpha:]]/\_/g')

if [ $do_init = 1 ]; then
	cp $language_home/template-notes.tex ./$project_name.tex
	cp $language_home/customlib.sty ./customlib.sty

	sed -i "s/PROJECT_NAME/$project_name/g" ./$project_name.tex

fi
