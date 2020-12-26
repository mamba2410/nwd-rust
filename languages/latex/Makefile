###########################################################################################################
# Makefile includes
###########################################################################################################

include ./build/build_number.mak	# build number tracking

###########################################################################################################
# Main recipe
###########################################################################################################

DEBUG = 

# If main relies on any libraries, add them to the LD_FLAGS variable
# Remember to add in-house libraries as dependencies to main recipe,
# ie make them before trying to link
#PDF_VIEWER		= mupdf
PDF_VIEWER		= firefox
CC				= pdflatex
REF_MAKER		= biber
CC_FLAGS		= --output-directory $(MAIN_OUT_D)
REF_FLAGS		= $(CC_FLAGS)
MAIN_SRC_D		= .
MAIN_OUT_D		= ./outs
MAIN_TEX_SRC	= $(wildcard $(MAIN_SRC_D)/*.tex)
MAIN_TEX_PDF	= $(patsubst $(MAIN_SRC_D)/%.tex, $(MAIN_OUT_D)/%.pdf, $(MAIN_TEX_SRC))
MAIN_TEX_BBL	= $(patsubst $(MAIN_SRC_D)/%.tex, $(MAIN_OUT_D)/%.bbl, $(MAIN_TEX_SRC))

DEPS_TEX_D		= ./src
DEPS_TEX_SRC	= $(wildcard $(DEPS_SRC_D)/*.tex)

FORCE:

$(MAIN_OUT_D)/%.pdf: $(MAIN_SRC_D)/%.tex $(MAIN_STY_SRC) $(DEPS_TEX_SRC) FORCE
ifndef DEBUG
	$(CC) $(CC_FLAGS) $< > /dev/null
else
	@echo "Debugging " $<
	$(CC) $(CC_FLAGS) $<
endif

$(MAIN_OUT_D)/%.bbl: $(MAIN_OUT_D)/%.tex
ifndef DEBUG
	$(CC) $(CC_FLAGS) $< > /dev/null
	$(REF_MAKER) $(REF_FLAGS) $(patsubst %.tex, %, $<) > /dev/null
else
	$(CC) $(CC_FLAGS) $<
	$(REF_MAKER) $(REF_FLAGS) $(patsubst %.tex, %, $<)
	@echo "Refs done " $<
endif



###########################################################################################################
# General recipes
###########################################################################################################

# Recipe for building, (re-)links the executable and triggers the build number
# This is the default recipe
.DEFAULT_GOAL = build
build: $(MAIN_TEX_BIB) $(MAIN_TEX_pdf) build_number

# Recipe for running, just builds and executes the binary
run: build
	$(PDF_VIEWER) $(MAIN_PDF) & disown

# Recipe for cleaning. Removes all objects and binaries
clean:
	rm -rf $(MAIN_OUT_D)/*
	@echo ""

