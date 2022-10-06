#!/bin/bash

# The KJV and Lexham link were found here:
# http://simoncozens.github.io/open-source-bible-data/

mkdir bible
wget -P "$PWD/bible" --no-clobber http://simoncozens.github.io/open-source-bible-data/cooked/sqlite/kjv.db
wget -P "$PWD/bible" --no-clobber http://simoncozens.github.io/open-source-bible-data/cooked/sqlite/LEB.db


