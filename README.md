# TODO List helper

## Actions

    1. select a todo list element
    2. add an item to your todo list
    3. reload your settings from your settings.json file
    4. exit the program

## File structure and purpose
    The program works through pre specified settings in a file on the path ./res/settings.json.
    Which will be created for you on first starting the program along with the ./res/ directory
    and ./res/input.list where your new todo list will live on the disk

    default values given below

    ./res/settings.json:
        {
            file_path: "./res/input.list"
            random_selection: false,
            remove_on_select: true,
        }

    ./res/input.list:
        empty
