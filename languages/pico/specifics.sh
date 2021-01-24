#!/bin/sh

language_home="$1/languages/pico"
common_home="$1/common"
project_name="$2"
do_init=$3

cp -vvv $PICO_SDK_PATH/external/pico_sdk_import.cmake ./pico_sdk_import.cmake
cp $language_home/Makefile ./Makefile
cp $language_home/CMakeLists.txt ./CMakeLists.txt
rm -r ./include

#[ -f .gitignore ] && echo "$modules_dir" >> .gitignore

project_name_upper=$(echo $project_name | tr '[:lower:]' '[:upper:]')
project_name_upper=$(echo $project_name_upper | sed 's/[^[:alpha:]]/\_/g')

sed -i "s/PROJECT_NAME/$project_name/g" ./CMakeLists.txt

if [ $do_init = 1 ]; then
	cp $language_home/init.c ./src/$project_name.c

	sed -i "s/PROJECT_NAME/$project_name/g" ./src/$project_name.c

fi
