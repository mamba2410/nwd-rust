#!/bin/sh

language_home="$1/languages/f90"
common_home="$1/common"
project_name="$2"
do_init=$3

cp $common_home/Makefile ./Makefile
cp $common_home/build_number.mak ./build/build_number.mak

patch -sl -i $language_home/make.patch

modules_dir="./build/target/modules/"

rm -r ./include
mkdir ./data
mkdir $modules_dir

#[ -f .gitignore ] && echo "$modules_dir" >> .gitignore

project_name_upper=$(echo $project_name | tr '[:lower:]' '[:upper:]')
project_name_upper=$(echo $project_name_upper | sed 's/[^[:alpha:]]/\_/g')

if [ $do_init = 1 ]; then
	cp $language_home/init.f90 ./src/$project_name.f90

	sed -i "s/PROJECT_NAME/$project_name/g" ./src/$project_name.f90

fi
