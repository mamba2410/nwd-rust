#!/bin/sh

language_home="$1/languages/rust"
common_home="$1/common"
project_name="$2"
do_init=$3

cp $common_home/Makefile ./Makefile
cp $common_home/build_number.mak ./build/build_number.mak

rm -r ./bin/
rm -r ./include/

#patch -sl -i $language_home/make.patch

project_name_upper=$(echo $project_name | tr '[:lower:]' '[:upper:]')
project_name_upper=$(echo $project_name_upper | sed 's/[^[:alpha:]]/\_/g')

if [ $do_init = 1 ]; then
	main_file="$project_name.rs"
	cp $language_home/init.rs ./src/$main_file
	cp $language_home/Cargo.toml ./

	sed -i "s/PROJECT_NAME/$project_name/g" ./src/$main_file
	sed -i "s/PROJECT_NAME/$project_name/g" ./Cargo.toml

fi
