# stegomage

A rust binary package that embeds secret messages in files for you.

## How to install

Simply run:

    cargo install stegomage

to install the package.

## How to use

You can encode an image using the command below:

    stegomage -e -i <path-to-image>

And you can decode an image using this command:

    stegomage -d -i <path-to-image>
